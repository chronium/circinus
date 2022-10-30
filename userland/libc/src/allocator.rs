use core::{
	alloc::GlobalAlloc,
	mem::MaybeUninit,
	ptr,
	sync::atomic::{AtomicUsize, Ordering},
};

use crate::platform::{sys::Sys, types::c_void};

pub struct Allocator {
	current_brk: MaybeUninit<AtomicUsize>,
	end_brk: MaybeUninit<AtomicUsize>,
}

impl Allocator {
	pub const fn new() -> Self {
		Self {
			current_brk: MaybeUninit::uninit(),
			end_brk: MaybeUninit::uninit(),
		}
	}

	pub fn init(&mut self, initial_size: usize) {
		let brk = Sys::brk(ptr::null_mut()) as usize;
		let end = Sys::brk((brk + initial_size) as *mut c_void) as usize;

		self.current_brk.write(AtomicUsize::new(brk));
		self.end_brk.write(AtomicUsize::new(end));

		// crate::println!(
		// "heap initial position and size {:012x}-{:012x} @ {}Kib",
		// brk.blue(),
		// end.blue(),
		// (initial_size / 1024).red()
		// );
	}
}

unsafe impl GlobalAlloc for Allocator {
	unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
		let size = layout.size();

		self.current_brk
			.assume_init_ref()
			.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |current| {
				let end = self.end_brk.assume_init_ref().load(Ordering::Relaxed);

				assert!(current + size <= end);

				Some(current + size)
			})
			.unwrap() as *mut _
	}

	unsafe fn dealloc(&self, _ptr: *mut u8, _layout: core::alloc::Layout) {
		// TODO
	}
}
