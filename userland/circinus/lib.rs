#![no_std]
#![feature(asm_const)]
#![feature(allocator_api)]

use allocator::Allocator;

extern crate alloc;

pub mod allocator;
pub mod io;
pub mod lang_items;
pub mod sys;

#[panic_handler]
pub fn panic(info: &core::panic::PanicInfo) -> ! {
	println!("{}", info);
	crate::sys::exit(!0)
}

#[macro_export]
macro_rules! start {
	() => {
		#[start]
		#[no_mangle]
		fn _start(_argc: isize, _argv: *const *const u8) -> isize {
			unsafe {
				$crate::setup_heap();
				main();
			}
			0
		}
	};
}

#[inline(never)]
pub unsafe fn setup_heap() {
	ALLOCATOR.init(1024 * 1024);
}

#[global_allocator]
pub static mut ALLOCATOR: Allocator = Allocator::new();

pub mod owo_colors {
	pub use owo_colors::*;
}
