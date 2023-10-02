use crate::{c_str::CStr, header::dirent::dirent};

use super::types::*;

pub struct Sys;

impl Sys {
  pub fn brk(caddr: *mut c_void) -> *mut c_void {
    syscall::brk(caddr as usize) as *mut c_void
  }

  pub fn open(path: &CStr, oflag: c_int, mode: mode_t) -> c_int {
    syscall::open(path.as_ptr(), oflag, mode) as c_int
  }

  pub fn close(fd: c_int) -> c_int {
    syscall::close(fd) as c_int
  }

  pub fn write(fd: c_int, buf: &[u8]) -> ssize_t {
    syscall::write(fd, buf) as ssize_t
  }

  pub fn exit(status: c_int) -> ! {
    syscall::exit(status)
  }

  pub unsafe fn execve(path: &CStr, argv: *const *mut c_char, envp: *const *mut c_char) -> c_int {
    syscall::execve(path.as_ptr(), argv, envp) as c_int
  }

  pub unsafe fn getcwd(buf: *mut c_char, size: size_t) -> *mut c_char {
    syscall::getcwd(buf, size) as *mut c_char
  }

  pub unsafe fn chdir(path: &CStr) -> c_int {
    syscall::chdir(path.as_ptr()) as c_int
  }

  pub unsafe fn getdents(fd: c_int, dirents: *mut dirent, bytes: usize) -> c_int {
    syscall::getdents(fd, dirents as usize, bytes) as c_int
  }
}
