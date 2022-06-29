#![no_std]
#![no_main]
#![feature(never_type)]
#![feature(box_syntax)]
#![feature(const_btree_new)]
#![feature(default_alloc_error_handler)]

#[macro_use]
extern crate alloc;

#[macro_use]
extern crate environment;

#[macro_use]
extern crate api;

use alloc::{boxed::Box, sync::Arc, vec::Vec};
use api::{
	driver::block::BlockDriver,
	kernel::KernelOps,
	schema::{fs::Partition, unix::Path},
	sync::SpinLock,
	vfs::mount::Rootfs,
};
use environment::{
	arch::{idle, PtRegs},
	bootinfo::BootInfo,
};
use fs::devfs::SERIAL_TTY;
use interrupt::attach_irq;
use process::Process;
use schema::{block, system::gpt};
use syscall::SyscallHandler;
use tempfs::{InMemoryFile, Tempfs};
use utils::once::Once;

use crate::{
	fs::devfs::{self, DEVFS},
	process::switch,
};

struct System;

impl environment::System for System {
	fn on_console_rx(&self, ch: u8) {
		SERIAL_TTY.input_char(ch);
	}

	fn on_irq(&self, irq: u8) {
		interrupt::handle_irq(irq);
	}

	fn on_timer_irq(&self) {
		crate::timer::handle_timer_irq();
	}

	fn on_page_fault(
		&self,
		unaligned_vaddr: Option<environment::address::UserVAddr>,
		ip: usize,
		reason: environment::arch::PageFaultReason,
	) {
		crate::mm::page_fault::handle_page_fault(unaligned_vaddr, ip, reason);
	}

	fn on_syscall(
		&self,
		a1: usize,
		a2: usize,
		a3: usize,
		a4: usize,
		a5: usize,
		a6: usize,
		n: usize,
		frame: *mut PtRegs,
	) -> isize {
		let mut handler = SyscallHandler::new(unsafe { &mut *frame });
		handler
			.dispatch(a1, a2, a3, a4, a5, a6, n)
			.unwrap_or_else(|err| -(err.errno() as isize))
	}

	#[cfg(debug_assertions)]
	fn usercopy_hook(&self) {
		use crate::process::current_process;

		// We should not hold the vm lock since we'll try to acquire it in the
		// page fault handler when copying caused a page fault.
		debug_assert!(!current_process().vm().as_ref().unwrap().is_locked());
	}
}

struct ApiOps;

impl KernelOps for ApiOps {
	fn attach_irq(&self, irq: u8, f: Box<(dyn FnMut() + Send + Sync + 'static)>) {
		attach_irq(irq, f);
	}

	fn register_block_driver(&self, driver: Box<dyn BlockDriver>) {
		block::register_block_driver(driver)
	}

	fn request_partitions(&self) -> Vec<Arc<SpinLock<dyn Partition>>> {
		gpt::partitions()
	}

	fn current_process(&self) -> Option<Arc<dyn api::ProcessOps>> {
		Some(process::current_process().clone() as Arc<dyn api::ProcessOps>)
	}
}

pub static INITIAL_ROOT_FS: Once<Arc<SpinLock<Rootfs>>> = Once::new();

#[no_mangle]
pub fn boot_kernel(#[cfg_attr(debug_assertions, allow(unused))] bootinfo: &BootInfo) -> ! {
	framebuffer::init(bootinfo);
	logger::init();

	info!("Booting System...");

	environment::set_system(&System);
	api::kernel::set_kernel_ops(&ApiOps);

	virtio_net::init();
	virtio_blk::init();

	api::kernel::init_drivers(bootinfo.pci_enabled, &bootinfo.virtio_mmio_devices);

	schema::system::init();

	// ext2::init();
	api::schema::fs::init();
	interrupt::init();

	devfs::init();

	let tempfs = Tempfs::new_root();

	tempfs.root().add_file(
		env!("INIT_FILE"),
		Arc::new(InMemoryFile::new(include_bytes!(concat!(
			"../userland/build/",
			env!("INIT_FILE")
		)))),
	);

	let dev_dir = tempfs.root().add_dir("Devices");

	let mut rootfs = Rootfs::new(Arc::new(tempfs)).unwrap();

	rootfs
		.mount(dev_dir, DEVFS.clone())
		.expect("failed to mount /Devices");

	let devcon = rootfs
		.lookup_path(Path::new("/Devices/devcon"), true)
		.expect("failed to open /Devices/devcon");

	let argv0 = env!("INIT_FILE");
	let executable_path = rootfs
		.lookup_path(Path::new(argv0), true)
		.expect("failed to open the init executable");

	INITIAL_ROOT_FS.init(|| Arc::new(SpinLock::new(rootfs)));

	process::init();

	info!("running /{}", env!("INIT_FILE"));
	Process::new_init_process(INITIAL_ROOT_FS.clone(), executable_path, devcon, &[b"/csh"])
		.expect("failed to execute init");

	switch();

	idle_thread();
}

fn idle_thread() -> ! {
	loop {
		idle();
	}
}

pub mod arch;
pub mod font;
pub mod framebuffer;
pub mod fs;
pub mod interrupt;
pub mod lang_items;
pub mod logger;
pub mod mm;
pub mod process;
pub mod random;
pub mod schema;
pub mod syscall;
pub mod timer;
pub mod tty;
