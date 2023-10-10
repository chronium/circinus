use core::{mem, ptr};

use crate::{
  c_str::CStr,
  header::errno,
  platform::{self, sys::Sys, types::*},
};

use super::{limits, stdlib::getenv};

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

#[no_mangle]
pub extern "C" fn fork() -> pid_t {
  Sys::fork()
}

#[no_mangle]
pub unsafe extern "C" fn execvp(file: *const c_char, argv: *const *mut c_char) -> c_int {
  let file = CStr::from_ptr(file);

  if file.to_bytes().contains(&b'/') {
    execv(file.as_ptr(), argv)
  } else {
    let mut error = errno::ENOENT;

    let path_env = getenv(c_str!("PATH\0").as_ptr());
    if !path_env.is_null() {
      let path_env = CStr::from_ptr(path_env);
      for path in path_env.to_bytes().split(|&b| b == b':') {
        let mut program = path.to_vec();
        program.push(b'/');
        program.extend_from_slice(file.to_bytes());
        program.push(b'\0');

        let program_c = CStr::from_bytes_with_nul(&program).unwrap();
        execv(program_c.as_ptr(), argv);

        match platform::errno {
          errno::ENOENT => (),
          other => error = other,
        }
      }
    }

    platform::errno = error;
    -1
  }
}
