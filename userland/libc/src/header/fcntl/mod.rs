use crate::{
  c_str::CStr,
  platform::{sys::Sys, types::*},
};

pub const F_DUPFD: c_int = 0;
pub const F_GETFD: c_int = 1;
pub const F_SETFD: c_int = 2;
pub const F_GETFL: c_int = 3;
pub const F_SETFL: c_int = 4;
pub const F_GETLK: c_int = 5;
pub const F_SETLK: c_int = 6;
pub const F_SETLKW: c_int = 7;

pub const F_RDLCK: c_int = 0;
pub const F_WRLCK: c_int = 1;
pub const F_UNLCK: c_int = 2;

pub const F_ULOCK: c_int = 0;
pub const F_LOCK: c_int = 1;
pub const F_TLOCK: c_int = 2;
pub const F_TEST: c_int = 3;

pub const O_RDONLY: c_int = 0o0;
pub const O_WRONLY: c_int = 0o1;
pub const O_RDWR: c_int = 0o2;
pub const O_CREAT: c_int = 0o100;
pub const O_EXCL: c_int = 0o200;
pub const O_NOCTTY: c_int = 0o400;
pub const O_TRUNC: c_int = 0o1000;
pub const O_APPEND: c_int = 0o2000;
pub const O_NONBLOCK: c_int = 0o4000;
pub const O_DIRECTORY: c_int = 0o200000;
pub const O_CLOEXEC: c_int = 0o2000000;

pub const FD_CLOEXEC: c_int = 0o2000000;

#[no_mangle]
pub unsafe extern "C" fn sys_open(path: *const c_char, oflag: c_int, mode: mode_t) -> c_int {
  let path = CStr::from_ptr(path);
  Sys::open(path, oflag, mode)
}

#[no_mangle]
pub unsafe extern "C" fn sys_fcntl(fd: c_int, cmd: c_int, arg: c_ulonglong) -> c_int {
  Sys::fcntl(fd, cmd, arg)
}
