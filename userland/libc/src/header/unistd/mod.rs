use core::ffi::{c_char, c_int};

use crate::{
	c_str::CStr,
	platform::{self, sys::Sys},
};

#[no_mangle]
pub unsafe extern "C" fn execv(path: *const c_char, argv: *const *mut c_char) -> c_int {
	execve(path, argv, platform::environ)
}

#[no_mangle]
pub unsafe extern "C" fn execve(
	path: *const c_char,
	argv: *const *mut c_char,
	envp: *const *mut c_char,
) -> c_int {
	let path = CStr::from_ptr(path);
	Sys::execve(path, argv, envp)
}
