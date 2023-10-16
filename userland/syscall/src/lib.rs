#![no_std]
#![feature(linkage)]
#![allow(non_camel_case_types)]

use core::ffi::{c_char, c_int, c_long, c_longlong};

use syscall::*;

pub mod syscall;

pub fn exit(status: i32) -> ! {
  sys1(Syscall::Exit, status as usize);
  unreachable!()
}

pub fn open(path: *const c_char, flags: c_int, mode: c_int) -> usize {
  sys3(Syscall::Open, path as usize, flags as usize, mode as usize)
}

pub fn close(fd: i32) -> usize {
  sys1(Syscall::Close, fd as usize)
}

pub fn brk(new_heap_end: usize) -> usize {
  sys1(Syscall::Brk, new_heap_end)
}

pub fn write(fd: i32, buf: &[u8]) -> usize {
  sys3(
    Syscall::Write,
    fd as usize,
    buf.as_ptr() as usize,
    buf.len(),
  )
}

pub fn execve(path: *const c_char, argv: *const *mut c_char, envp: *const *mut c_char) -> usize {
  sys3(Syscall::Execve, path as usize, argv as usize, envp as usize)
}

pub fn read(fd: i32, buf: &[u8]) -> usize {
  sys3(Syscall::Read, fd as usize, buf.as_ptr() as usize, buf.len())
}

pub fn getcwd(path: *mut c_char, size: usize) -> usize {
  sys2(Syscall::GetCwd, path as usize, size)
}

pub fn chdir(path: *const c_char) -> usize {
  sys1(Syscall::Chdir, path as usize)
}

pub fn getdents(fd: i32, buf: usize, bytes: usize) -> usize {
  sys3(Syscall::GetDents64, fd as usize, buf, bytes)
}

pub fn fork() -> usize {
  sys(Syscall::Fork)
}

pub fn wait4(pid: i32, status: *mut i32, options: i32, rusage: *mut u8) -> usize {
  sys4(
    Syscall::Wait4,
    pid as usize,
    status as usize,
    options as usize,
    rusage as usize,
  )
}

pub fn fcntl(fd: i32, cmd: i32, arg: usize) -> usize {
  sys3(Syscall::Fcntl, fd as usize, cmd as usize, arg)
}

pub fn lseek(fd: i32, offset: c_longlong, whence: i32) -> usize {
  sys3(
    Syscall::LSeek,
    fd as usize,
    offset as usize,
    whence as usize,
  )
}

#[linkage = "weak"]
#[no_mangle]
extern "C" fn cilibc_panic(_pi: &::core::panic::PanicInfo) -> ! {
  loop {}
}
