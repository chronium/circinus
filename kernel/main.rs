#![no_std]
#![no_main]
#![feature(asm)]
#![feature(global_asm)]
#![feature(never_type)]
#![feature(const_btree_new)]
#![feature(default_alloc_error_handler)]
#![feature(box_syntax)]

#[macro_use]
extern crate alloc;

#[macro_use]
extern crate log;

#[macro_use]
extern crate environment;

use api::kernel::KernelOps;
use environment::{
	arch::{idle, PtRegs},
	bootinfo::BootInfo,
};
use interrupt::attach_irq;
use schema::{block, system::gpt};

use crate::process::switch;

struct System;

impl environment::System for System {
	fn on_console_rx(&self, _char: u8) {
		println!("a");
	}

	fn on_irq(&self, irq: u8) {
		interrupt::handle_irq(irq);
	}

	fn on_timer_irq(&self) {
		crate::timer::handle_timer_irq();
	}

	fn on_page_fault(
		&self,
		_unaligned_vaddr: Option<environment::address::UserVAddr>,
		_ip: usize,
		_reason: environment::arch::PageFaultReason,
	) {
	}

	fn on_syscall(
		&self,
		_a1: usize,
		_a2: usize,
		_a3: usize,
		_a4: usize,
		_a5: usize,
		_a6: usize,
		_n: usize,
		_frame: *mut PtRegs,
	) -> isize {
		0
	}
}

struct ApiOps;

impl KernelOps for ApiOps {
	fn attach_irq(
		&self,
		irq: u8,
		f: alloc::boxed::Box<(dyn FnMut() + Send + Sync + 'static)>,
	) {
		attach_irq(irq, f);
	}

	fn register_block_driver(
		&self,
		driver: alloc::boxed::Box<dyn api::driver::block::BlockDriver>,
	) {
		block::register_block_driver(driver)
	}

	fn request_partitions(
		&self,
	) -> alloc::vec::Vec<alloc::sync::Arc<dyn api::schema::fs::Partition>> {
		gpt::partitions()
	}
}

fn thread_a() -> ! {
	idle_thread();
}

#[no_mangle]
pub fn boot_kernel(
	#[cfg_attr(debug_assertions, allow(unused))] bootinfo: &BootInfo,
) -> ! {
	framebuffer::init(bootinfo);
	logger::init();

	info!("Booting System...");

	environment::set_system(&System);
	api::kernel::set_kernel_ops(&ApiOps);

	virtio_net::init();
	virtio_blk::init();

	api::kernel::init_drivers(
		bootinfo.pci_enabled,
		&bootinfo.virtio_mmio_devices,
	);

	schema::system::init();

	ext2::init();
	api::schema::fs::init();

	interrupt::init();
	process::init();

	process::Process::new_kernel_thread(thread_a);

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
pub mod interrupt;
pub mod lang_items;
pub mod logger;
pub mod process;
pub mod schema;
pub mod timer;
