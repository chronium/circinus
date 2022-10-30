use core::arch::asm;

use crate::{
	allocator::Allocator,
	header::stdio,
	platform::types::{c_char, c_int},
};

#[repr(C)]
pub struct Stack {
	pub argc: isize,
	pub argv0: *const c_char,
}

#[global_allocator]
pub static mut ALLOCATOR: Allocator = Allocator::new();

#[link_section = ".init_array"]
#[used]
static INIT_ARRAY: [extern "C" fn(); 1] = [init_array];
static mut init_complete: bool = false;

fn alloc_init() {
	unsafe {
		if init_complete {
			return;
		}
	}

	unsafe {
		ALLOCATOR.init(1024 * 1024 * 16);
	}
}

extern "C" fn init_array() {
	unsafe {
		if init_complete {
			return;
		}
	}

	alloc_init();
	io_init();
}

fn io_init() {
	unsafe {
		stdio::stdin = stdio::default_stdin.get();
		stdio::stdout = stdio::default_stdout.get();
		stdio::stderr = stdio::default_stdout.get();
	}
}

fn exit(code: i32) -> ! {
	unsafe {
		asm!("syscall", in("rdi") code as usize, in("rax") -1isize as usize);
	}
	unreachable!()
}

#[no_mangle]
#[linkage = "weak"]
pub unsafe extern "C" fn main(_argc: isize, _argv: *const *const i8) -> usize {
	// LD
	0x1D
}

#[no_mangle]
#[linkage = "weak"]
pub unsafe extern "C" fn _init() {}

#[inline(never)]
#[no_mangle]
pub unsafe extern "C" fn cilibc_start(_sp: &'static Stack) -> ! {
	extern "C" {
		static __preinit_array_start: extern "C" fn();
		static __preinit_array_end: extern "C" fn();
		static __init_array_start: extern "C" fn();
		static __init_array_end: extern "C" fn();

		fn _init();
		fn main(argc: isize, argv: *mut *mut c_char, envp: *mut *mut c_char) -> c_int;
	}

	alloc_init();

	init_array();

	{
		let mut f = &__preinit_array_start as *const _;
		#[allow(clippy::op_ref)]
		while f < &__preinit_array_end {
			(*f)();
			f = f.offset(1);
		}
	}

	_init();

	{
		let mut f = &__init_array_start as *const _;
		#[allow(clippy::op_ref)]
		while f < &__init_array_end {
			(*f)();
			f = f.offset(1);
		}
	}

	exit(main(0, core::ptr::null_mut(), core::ptr::null_mut()));
}
