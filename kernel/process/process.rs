use alloc::{
	collections::BTreeMap,
	sync::{Arc, Weak},
};
use core::{
	borrow::BorrowMut,
	cmp::max,
	mem::size_of,
	sync::atomic::{AtomicI32, Ordering},
};

use atomic_refcell::{AtomicRef, AtomicRefCell};
use crossbeam::atomic::AtomicCell;
use goblin::elf64::program_header::{ProgramHeader, PT_LOAD};

use api::{
	cmdline::Cmdline,
	ctypes::c_int,
	io::{OpenFlags, OpenOptions},
	process::{PgId, Pid, ProcessState},
	vfs::{
		interface::PathComponent,
		mount::Rootfs,
		opened_file::{OpenedFile, OpenedFileTable},
		Fd,
	},
	ErrorKind, ProcessOps, Result,
};
use environment::{
	address::{UserVAddr, VAddr},
	arch::{PtRegs, PAGE_SIZE},
	page_allocator::{alloc_pages, AllocPageFlags},
	spinlock::SpinLock,
};
use utils::alignment::align_up;

use crate::{
	arch::{self, KERNEL_STACK_SIZE, USER_STACK_TOP},
	fs::devfs::SERIAL_TTY,
	mm::vm::{Vm, VmAreaType},
	process::{
		current_process,
		elf::Elf,
		init_stack::{estimate_user_init_stack_size, init_user_stack, Auxv},
		process_group::ProcessGroup,
		switch, SCHEDULER,
	},
	random::read_secure_random,
	INITIAL_ROOT_FS,
};

type ProcessTable = BTreeMap<Pid, Arc<Process>>;

pub(super) static PROCESSES: SpinLock<ProcessTable> = SpinLock::new(BTreeMap::new());

pub(super) fn alloc_pid(table: &mut ProcessTable) -> Result<Pid> {
	static NEXT_PID: AtomicI32 = AtomicI32::new(2);

	let last_pid = NEXT_PID.load(Ordering::SeqCst);
	loop {
		// Note: `fetch_add` may wrap around.
		let pid = NEXT_PID.fetch_add(1, Ordering::SeqCst);
		if pid <= 1 {
			continue;
		}

		if !table.contains_key(&Pid::new(pid)) {
			return Ok(Pid::new(pid));
		}

		if pid == last_pid {
			return Err(ErrorKind::PidAllocFailed.into());
		}
	}
}

pub struct Process {
	arch: arch::Process,
	is_idle: bool,
	process_group: AtomicRefCell<Weak<SpinLock<ProcessGroup>>>,
	pid: Pid,
	state: AtomicCell<ProcessState>,
	cmdline: AtomicRefCell<Cmdline>,
	vm: AtomicRefCell<Option<Arc<SpinLock<Vm>>>>,
	rootfs: Arc<SpinLock<Rootfs>>,
	opened_files: Arc<SpinLock<OpenedFileTable>>,
}

impl Process {
	pub fn new_idle_thread() -> Result<Arc<Self>> {
		let process_group = ProcessGroup::new(PgId::new(0));
		let proc = Arc::new(Self {
			arch: arch::Process::new_idle_thread(),
			is_idle: true,
			process_group: AtomicRefCell::new(Arc::downgrade(&process_group)),
			pid: Pid::new(0),
			state: AtomicCell::new(ProcessState::Runnable),
			cmdline: AtomicRefCell::new(Cmdline::new()),
			vm: AtomicRefCell::new(None),
			rootfs: INITIAL_ROOT_FS.clone(),
			opened_files: Arc::new(SpinLock::new(OpenedFileTable::new())),
		});

		process_group.lock().add(Arc::downgrade(&proc));
		Ok(proc)
	}

	pub fn new_kernel_thread(f: fn() -> !) -> Result<()> {
		let stack_bottom = alloc_pages(KERNEL_STACK_SIZE / PAGE_SIZE, AllocPageFlags::KERNEL)?;
		let kernel_sp = stack_bottom.as_vaddr().add(KERNEL_STACK_SIZE);

		let ip = VAddr::new(f as *const u8 as usize);
		let pid = alloc_pid(&mut PROCESSES.lock())?;

		let process_group = ProcessGroup::new(PgId::new(0));
		let proc = Arc::new(Self {
			is_idle: false,
			arch: arch::Process::new_kthread(ip, kernel_sp),
			process_group: AtomicRefCell::new(Arc::downgrade(&process_group)),
			pid,
			state: AtomicCell::new(ProcessState::Runnable),
			cmdline: AtomicRefCell::new(Cmdline::new()),
			vm: AtomicRefCell::new(None),
			rootfs: INITIAL_ROOT_FS.clone(),
			opened_files: Arc::new(SpinLock::new(OpenedFileTable::new())),
		});

		process_group.lock().add(Arc::downgrade(&proc));
		PROCESSES.lock().insert(pid, proc);
		SCHEDULER.lock().enqueue(pid);

		Ok(())
	}

	pub fn new_init_process(
		rootfs: Arc<SpinLock<Rootfs>>,
		executable_path: Arc<PathComponent>,
		console: Arc<PathComponent>,
		argv: &[&[u8]],
	) -> Result<()> {
		assert!(console.node.is_file());

		let mut opened_files = OpenedFileTable::new();
		opened_files.open_with_fixed_fd(
			Fd::new(0),
			Arc::new(OpenedFile::new(
				console.clone(),
				OpenFlags::O_RDONLY.into(),
				0,
			)),
			OpenOptions::empty(),
		)?;
		// Open stdout.
		opened_files.open_with_fixed_fd(
			Fd::new(1),
			Arc::new(OpenedFile::new(
				console.clone(),
				OpenFlags::O_WRONLY.into(),
				0,
			)),
			OpenOptions::empty(),
		)?;
		opened_files.open_with_fixed_fd(
			Fd::new(2),
			Arc::new(OpenedFile::new(console, OpenFlags::O_WRONLY.into(), 0)),
			OpenOptions::empty(),
		)?;

		let entry = setup_userspace(executable_path, argv, &[], &rootfs)?;
		let pid = Pid::new(1);
		let process_group = ProcessGroup::new(PgId::new(1));

		let proc = Arc::new(Self {
			is_idle: false,
			arch: arch::Process::new_user_thread(entry.ip, entry.user_sp),
			process_group: AtomicRefCell::new(Arc::downgrade(&process_group)),
			pid,
			state: AtomicCell::new(ProcessState::Runnable),
			cmdline: AtomicRefCell::new(Cmdline::from_argv(argv)),
			vm: AtomicRefCell::new(Some(Arc::new(SpinLock::new(entry.vm)))),
			rootfs,
			opened_files: Arc::new(SpinLock::new(opened_files)),
		});

		process_group.lock().add(Arc::downgrade(&proc));
		PROCESSES.lock().insert(pid, proc);
		SCHEDULER.lock().enqueue(pid);

		SERIAL_TTY.set_foreground_process_group(Arc::downgrade(&process_group));
		Ok(())
	}

	fn _pid(&self) -> Pid {
		self.pid
	}

	pub fn state(&self) -> ProcessState {
		self.state.load()
	}

	pub fn cmdline(&self) -> AtomicRef<'_, Cmdline> {
		self.cmdline.borrow()
	}

	pub fn arch(&self) -> &arch::Process {
		&self.arch
	}

	pub fn rootfs(&self) -> &Arc<SpinLock<Rootfs>> {
		&self.rootfs
	}

	pub fn _opened_files(&self) -> &Arc<SpinLock<OpenedFileTable>> {
		&self.opened_files
	}

	/// The virtual memory space. It's `None` if the process is a kernel thread.
	pub fn vm(&self) -> AtomicRef<'_, Option<Arc<SpinLock<Vm>>>> {
		self.vm.borrow()
	}

	pub fn belongs_to_process_group(&self, pg: &Weak<SpinLock<ProcessGroup>>) -> bool {
		Weak::ptr_eq(&self.process_group.borrow(), pg)
	}

	fn _set_state(&self, new_state: ProcessState) {
		let scheduler = SCHEDULER.lock();
		self.state.store(new_state);
		match new_state {
			ProcessState::Runnable => {}
			ProcessState::BlockedSignalable | ProcessState::Exited(_) => {
				scheduler.remove(self.pid);
			}
		}
	}

	fn get_opened_file_by_fd(&self, fd: Fd) -> Result<Arc<OpenedFile>> {
		Ok(self.opened_files.lock().get(fd)?.clone())
	}

	fn exit(status: c_int) -> ! {
		let current = current_process();
		if current.pid == Pid::new(1) {
			panic!("init (pid=0) tried to exit")
		}

		api::Process::set_state(ProcessState::Exited(status));
		// if let Some(parent) = current.parent.upgrade() {
		// if parent.signals().lock().get_action(SIGCHLD) == SigAction::Ignore
		// {
		// If the parent process is not waiting for a child,
		// remove the child from its list.
		// parent.children().retain(|p| p.pid() != current.pid);
		//
		// Keep the reference because we're using its kernel stack.
		// Postpone freeing the stack until we move from the current
		// thread.
		// EXITED_PROCESSES.lock().push(current.clone());
		// } else {
		// parent.send_signal(SIGCHLD)
		// }
		// }

		// Close opened files here instead of in Drop::drop because `proc` is
		// not dropped until it's joined by the parent process. Drop them to
		// make pipes closed.
		// current.opened_files.lock().close_all();

		PROCESSES.lock().remove(&current.pid);
		// JOIN_WAIT_QUEUE.wake_all();
		switch();
		unreachable!();
	}

	fn _has_pending_signals(&self) -> bool {
		false
	}

	pub fn _resume(&self) {
		let old_state = self.state.swap(ProcessState::Runnable);

		debug_assert!(!matches!(old_state, ProcessState::Exited(_)));

		if old_state == ProcessState::Runnable {
			return;
		}

		SCHEDULER.lock().enqueue(self.pid);
	}

	pub fn execve(
		frame: &mut PtRegs,
		executable_path: Arc<PathComponent>,
		argv: &[&[u8]],
		envp: &[&[u8]],
	) -> Result<()> {
		let current = current_process();
		current.opened_files().lock().close_cloexec_files();
		current.cmdline.borrow_mut().set_by_argv(argv);

		let entry = setup_userspace(executable_path, argv, envp, &current.rootfs)?;

		// TODO: Signal?

		entry.vm.page_table().switch();
		*current.vm.borrow_mut() = Some(Arc::new(SpinLock::new(entry.vm)));

		current
			.arch
			.setup_execve_stack(frame, entry.ip, entry.user_sp)?;

		Ok(())
	}

	pub fn process_group(&self) -> Arc<SpinLock<ProcessGroup>> {
		self.process_group.borrow().upgrade().unwrap()
	}

	fn current<'a>() -> &'a Arc<Process> {
		current_process()
	}
}

impl Drop for Process {
	fn drop(&mut self) {
		trace!(
			"dropping {:?} (cmdline={})",
			self.pid(),
			self.cmdline().as_str()
		);
		// Since the process's reference count has already reached to zero
		// (that's why the process is being dropped),
		// ProcessGroup::remove_dropped_processes should remove this process
		// from its list.
		self.process_group().lock().remove_dropped_processes();
	}
}

impl ProcessOps for Process {
	fn rootfs(&self) -> &Arc<SpinLock<Rootfs>> {
		self.rootfs()
	}

	fn exit(&self, status: c_int) -> ! {
		Process::exit(status)
	}

	fn get_open_file_by_fid(
		&self,
		fd: api::vfs::Fd,
	) -> Result<Arc<api::vfs::opened_file::OpenedFile>> {
		self.get_opened_file_by_fd(fd)
	}

	fn set_state(&self, new_state: ProcessState) {
		self._set_state(new_state)
	}

	fn has_pending_signals(&self) -> bool {
		self._has_pending_signals()
	}

	fn resume(&self) {
		self._resume();
	}

	fn pid(&self) -> api::process::Pid {
		self._pid()
	}

	fn opened_files(&self) -> Arc<SpinLock<OpenedFileTable>> {
		self._opened_files().clone()
	}

	fn cmdline(&self) -> AtomicRef<'_, Cmdline> {
		self.cmdline()
	}
}

struct UserspaceEntry {
	vm: Vm,
	ip: UserVAddr,
	user_sp: UserVAddr,
}

fn setup_userspace(
	executable_path: Arc<PathComponent>,
	argv: &[&[u8]],
	envp: &[&[u8]],
	root_fs: &Arc<SpinLock<Rootfs>>,
) -> Result<UserspaceEntry> {
	do_setup_userspace(executable_path, argv, envp, root_fs, true)
}

fn do_setup_userspace(
	executable_path: Arc<PathComponent>,
	argv: &[&[u8]],
	envp: &[&[u8]],
	root_fs: &Arc<SpinLock<Rootfs>>,
	_handle_shebang: bool,
) -> Result<UserspaceEntry> {
	// Read the ELF header in the executable file.
	let file_header_len = PAGE_SIZE;
	let file_header_top = USER_STACK_TOP;
	let file_header_pages = alloc_pages(file_header_len / PAGE_SIZE, AllocPageFlags::KERNEL)?;
	let buf =
		unsafe { core::slice::from_raw_parts_mut(file_header_pages.as_mut_ptr(), file_header_len) };

	let executable = executable_path.node.as_file()?;
	executable.read(0, buf.into(), &OpenOptions::readwrite())?;

	let elf = Elf::parse(buf)?;
	let ip = elf.entry()?;

	let mut end_of_image = 0;
	for phdr in elf.program_headers() {
		if phdr.p_type == PT_LOAD {
			end_of_image = max(end_of_image, (phdr.p_vaddr + phdr.p_memsz) as usize);
		}
	}

	let mut random_bytes = [0u8; 16];
	read_secure_random(((&mut random_bytes) as &mut [u8]).into())?;

	// Set up the user stack.
	let auxv = &[
		Auxv::Phdr(
			file_header_top
				.sub(file_header_len)
				.add(elf.header().e_phoff as usize),
		),
		Auxv::Phnum(elf.program_headers().len()),
		Auxv::Phent(size_of::<ProgramHeader>()),
		Auxv::Pagesz(PAGE_SIZE),
		Auxv::Random(random_bytes),
	];
	const USER_STACK_LEN: usize = 1024 * 1024; // TODO: Implement rlimit
	let init_stack_top = file_header_top.sub(file_header_len);
	let user_stack_bottom = init_stack_top.sub(USER_STACK_LEN).value();
	let user_heap_bottom = align_up(end_of_image, PAGE_SIZE);
	let init_stack_len = align_up(estimate_user_init_stack_size(argv, envp, auxv), PAGE_SIZE);
	if user_heap_bottom >= user_stack_bottom || init_stack_len >= USER_STACK_LEN {
		return Err(ErrorKind::TooBig.into());
	}

	let init_stack_pages = alloc_pages(init_stack_len / PAGE_SIZE, AllocPageFlags::KERNEL)?;
	let user_sp = init_user_stack(
		init_stack_top,
		init_stack_pages.as_vaddr().add(init_stack_len),
		init_stack_pages.as_vaddr(),
		argv,
		envp,
		auxv,
	)?;

	let mut vm = Vm::new(
		UserVAddr::new(user_stack_bottom).unwrap(),
		UserVAddr::new(user_heap_bottom).unwrap(),
	)?;
	for i in 0..(file_header_len / PAGE_SIZE) {
		vm.page_table_mut().map_user_page(
			file_header_top.sub(((file_header_len / PAGE_SIZE) - i) * PAGE_SIZE),
			file_header_pages.add(i * PAGE_SIZE),
		);
	}

	for i in 0..(init_stack_len / PAGE_SIZE) {
		vm.page_table_mut().map_user_page(
			init_stack_top.sub(((init_stack_len / PAGE_SIZE) - i) * PAGE_SIZE),
			init_stack_pages.add(i * PAGE_SIZE),
		);
	}

	// Register program headers in the virtual memory space.
	for phdr in elf.program_headers() {
		if phdr.p_type != PT_LOAD {
			continue;
		}

		let area_type = if phdr.p_filesz > 0 {
			VmAreaType::File {
				file: executable.clone(),
				offset: phdr.p_offset as usize,
				file_size: phdr.p_filesz as usize,
			}
		} else {
			VmAreaType::Anonymous
		};

		vm.add_vm_area(
			UserVAddr::new_nonnull(phdr.p_vaddr as usize)?,
			phdr.p_memsz as usize,
			area_type,
		)?;
	}

	Ok(UserspaceEntry { vm, ip, user_sp })
}
