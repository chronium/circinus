use core::ptr;

use crate::{
  c_str::CStr,
  platform::{self, sys::Sys, types::*},
};

use super::limits;

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

#[no_mangle]
pub unsafe extern "C" fn getcwd(mut buf: *mut c_char, mut size: size_t) -> *mut c_char {
  let alloc = buf.is_null();
  let mut stack_buf = [0; limits::PATH_MAX];
  if alloc {
    buf = stack_buf.as_mut_ptr();
    size = stack_buf.len();
  }

  let ret = Sys::getcwd(buf, size);
  if ret.is_null() {
    return ptr::null_mut();
  }

  if alloc {
    let len = stack_buf
      .iter()
      .position(|b| *b == 0)
      .expect("no nul-byte in getcwd string")
      + 1;
    let heap_buf = unsafe { platform::alloc(len) as *mut c_char };
    for i in 0..len {
      unsafe {
        *heap_buf.add(i) = stack_buf[i];
      }
    }
    heap_buf
  } else {
    ret
  }
}

#[no_mangle]
pub unsafe extern "C" fn chdir(path: *const c_char) -> c_int {
  let path = CStr::from_ptr(path);
  Sys::chdir(path)
}
