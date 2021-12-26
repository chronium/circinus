use core::sync::atomic::{AtomicI32, Ordering};

use alloc::{collections::BTreeMap, sync::Arc};
use crossbeam::atomic::AtomicCell;
use environment::{
	address::VAddr,
	arch::PAGE_SIZE,
	page_allocator::{alloc_pages, AllocPageFlags},
	spinlock::SpinLock,
};

use crate::arch::{self, KERNEL_STACK_SIZE};

use super::SCHEDULER;

type ProcessTable = BTreeMap<Pid, Arc<Process>>;

pub(super) static PROCESSES: SpinLock<ProcessTable> =
	SpinLock::new(BTreeMap::new());

pub(super) fn alloc_pid(table: &mut ProcessTable) -> Pid {
	static NEXT_PID: AtomicI32 = AtomicI32::new(1);

	// let last_pid = NEXT_PID.load(Ordering::SeqCst);
	loop {
		// Note: `fetch_add` may wrap around.
		let pid = NEXT_PID.fetch_add(1, Ordering::SeqCst);
		if pid <= 1 {
			continue;
		}

		if !table.contains_key(&Pid::new(pid)) {
			return Pid::new(pid);
		}

		// TODO
		// if pid == last_pid {
		// return Err(Errno::EAGAIN.into());
		// }
	}
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Pid(i32);

impl Pid {
	pub const fn new(pid: i32) -> Self {
		Self(pid)
	}

	pub const fn as_i32(self) -> i32 {
		self.0
	}
}

/// Process states.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ProcessState {
	/// The process is runnable.
	Runnable,
	/// The process is sleeping. It can be resumed by signals.
	BlockedSignalable,
	/// The process has exited.
	Exited, /* TODO
	         * ExitedWith(c_int) */
}

pub struct Process {
	arch: arch::Process,
	pid: Pid,
	state: AtomicCell<ProcessState>,
}

impl Process {
	pub fn new_idle_thread() -> Arc<Self> {
		Arc::new(Self {
			arch: arch::Process::new_idle_thread(),
			pid: Pid::new(0),
			state: AtomicCell::new(ProcessState::Runnable),
		})
	}

	// TODO: Result<()>
	pub fn new_kernel_thread(f: fn() -> !) {
		let stack_bottom =
			alloc_pages(KERNEL_STACK_SIZE / PAGE_SIZE, AllocPageFlags::KERNEL)
				// TODO error
				.expect("todo");
		let kernel_sp = stack_bottom.as_vaddr().add(KERNEL_STACK_SIZE);

		let ip = VAddr::new(f as *const u8 as usize);
		let pid = alloc_pid(&mut PROCESSES.lock());

		let proc = Arc::new(Self {
			arch: arch::Process::new_kthread(ip, kernel_sp),
			pid,
			state: AtomicCell::new(ProcessState::Runnable),
		});

		PROCESSES.lock().insert(pid, proc);
		SCHEDULER.lock().enqueue(pid);
	}

	pub fn pid(&self) -> Pid {
		self.pid
	}

	pub fn state(&self) -> ProcessState {
		self.state.load()
	}

	pub fn arch(&self) -> &arch::Process {
		&self.arch
	}
}
