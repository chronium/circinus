use core::cell::UnsafeCell;

use crossbeam::atomic::AtomicCell;
use environment::{
	address::VAddr,
	arch::{cpu_local_head, PAGE_SIZE, TSS},
	page_allocator::{alloc_pages_owned, AllocPageFlags, OwnedPages},
};
use x86::current::segmentation::wrfsbase;

use super::KERNEL_STACK_SIZE;

#[repr(C, packed)]
pub struct Process {
	rsp: UnsafeCell<u64>,
	pub(super) fsbase: AtomicCell<u64>,
	pub(super) xsave_area: Option<OwnedPages>,
	kernel_stack: OwnedPages,
	interrupt_stack: OwnedPages,
	syscall_stack: OwnedPages,
}

unsafe impl Sync for Process {}

extern "C" {
	fn kthread_entry();
	fn userland_entry();
	fn forked_child_entry();
	fn do_switch_thread(prev_rsp: *const u64, next_rsp: *const u64);
}

unsafe fn push_stack(mut rsp: *mut u64, value: u64) -> *mut u64 {
	rsp = rsp.sub(1);
	rsp.write(value);
	rsp
}

impl Process {
	pub fn new_kthread(ip: VAddr, sp: VAddr) -> Self {
		let interrupt_stack = alloc_pages_owned(
			KERNEL_STACK_SIZE / PAGE_SIZE,
			AllocPageFlags::KERNEL | AllocPageFlags::DIRTY_OK,
		)
		.expect("failed to allocate kernel stack");
		let syscall_stack = alloc_pages_owned(
			KERNEL_STACK_SIZE / PAGE_SIZE,
			AllocPageFlags::KERNEL | AllocPageFlags::DIRTY_OK,
		)
		.expect("failed to allocate kernel stack");

		let kernel_stack = alloc_pages_owned(
			KERNEL_STACK_SIZE / PAGE_SIZE,
			AllocPageFlags::KERNEL | AllocPageFlags::DIRTY_OK,
		)
		.expect("failed to allocat kernel stack");

		let rsp = unsafe {
			let mut rsp: *mut u64 = sp.as_mut_ptr();

			// Registers to be restored in kthread_entry().
			rsp = push_stack(rsp, ip.value() as u64); // The entry point.

			// Registers to be restored in do_switch_thread().
			rsp = push_stack(rsp, kthread_entry as *const u8 as u64); // RIP.
			rsp = push_stack(rsp, 0); // Initial RBP.
			rsp = push_stack(rsp, 0); // Initial RBX.
			rsp = push_stack(rsp, 0); // Initial R12.
			rsp = push_stack(rsp, 0); // Initial R13.
			rsp = push_stack(rsp, 0); // Initial R14.
			rsp = push_stack(rsp, 0); // Initial R15.
			rsp = push_stack(rsp, 0x02); // RFLAGS (interrupts disabled).

			rsp
		};

		Self {
			rsp: UnsafeCell::new(rsp as u64),
			fsbase: AtomicCell::new(0),
			xsave_area: None,
			kernel_stack,
			interrupt_stack,
			syscall_stack,
		}
	}

	pub fn new_idle_thread() -> Process {
		let interrupt_stack = alloc_pages_owned(
			KERNEL_STACK_SIZE / PAGE_SIZE,
			AllocPageFlags::KERNEL | AllocPageFlags::DIRTY_OK,
		)
		.expect("failed to allocate kernel stack");
		let syscall_stack = alloc_pages_owned(
			KERNEL_STACK_SIZE / PAGE_SIZE,
			AllocPageFlags::KERNEL | AllocPageFlags::DIRTY_OK,
		)
		.expect("failed to allocate kernel stack");

		let kernel_stack = alloc_pages_owned(
			KERNEL_STACK_SIZE / PAGE_SIZE,
			AllocPageFlags::KERNEL | AllocPageFlags::DIRTY_OK,
		)
		.expect("failed to allocat kernel stack");

		Process {
			rsp: UnsafeCell::new(0),
			fsbase: AtomicCell::new(0),
			xsave_area: None,
			kernel_stack,
			interrupt_stack,
			syscall_stack,
		}
	}
}

pub fn switch_thread(prev: &Process, next: &Process) {
	let head = cpu_local_head();

	// Switch the kernel stack.
	head.rsp0 =
		(next.syscall_stack.as_vaddr().value() + KERNEL_STACK_SIZE) as u64;
	TSS.as_mut().set_rsp0(
		(next.interrupt_stack.as_vaddr().value() + KERNEL_STACK_SIZE) as u64,
	);

	// Save and restore the XSAVE area (i.e. XMM/YMM registrers).
	unsafe {
		use core::arch::x86_64::{_xrstor64, _xsave64};

		let xsave_mask = x86::controlregs::xcr0().bits();
		if let Some(xsave_area) = prev.xsave_area.as_ref() {
			_xsave64(xsave_area.as_mut_ptr(), xsave_mask);
		}
		if let Some(xsave_area) = next.xsave_area.as_ref() {
			_xrstor64(xsave_area.as_mut_ptr(), xsave_mask);
		}
	}

	// Fill an invalid value for now: must be initialized in interrupt handlers.
	head.rsp3 = 0xbaad_5a5a_5b5b_baad;

	unsafe {
		wrfsbase(next.fsbase.load());
		do_switch_thread(prev.rsp.get(), next.rsp.get());
	}
}
