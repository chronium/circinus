use crate::{
	header::{ctype, stdio::flush_io_streams},
	platform::{self, sys::Sys, types::*},
};

pub const EXIT_SUCCESS: c_int = 0;
pub const EXIT_FAILURE: c_int = 1;

static mut ATEXIT_FUNCS: [Option<extern "C" fn()>; 32] = [None; 32];

#[no_mangle]
pub unsafe extern "C" fn strtod(s: *const c_char, endptr: *mut *mut c_char) -> c_double {
	strto_float_impl!(c_double, s, endptr)
}

#[no_mangle]
pub unsafe extern "C" fn malloc(size: size_t) -> *mut c_void {
	let ptr = platform::alloc(size);
	if ptr.is_null() {
		// TODO: errno = ENOMEM
		panic!("malloc failed");
	}
	ptr
}

#[no_mangle]
pub unsafe extern "C" fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void {
	let ptr = platform::realloc(ptr, size);
	if ptr.is_null() {
		// TODO: errno = ENOMEM
		panic!("realloc failed");
	}
	ptr
}

#[no_mangle]
pub unsafe extern "C" fn free(ptr: *mut c_void) {
	platform::free(ptr);
}

#[no_mangle]
#[linkage = "weak"]
pub unsafe extern "C" fn _fini() {}

#[no_mangle]
pub unsafe extern "C" fn exit(status: c_int) {
	extern "C" {
		static __fini_array_start: extern "C" fn();
		static __fini_array_end: extern "C" fn();

		fn _fini();
	}

	for i in (0..ATEXIT_FUNCS.len()).rev() {
		if let Some(func) = ATEXIT_FUNCS[i].take() {
			(func)();
		}
	}

	let mut f = &__fini_array_end as *const _;
	#[allow(clippy::op_ref)]
	while f > &__fini_array_start {
		f = f.offset(-1);
		(*f)();
	}

	_fini();

	flush_io_streams();

	Sys::exit(status);
}
