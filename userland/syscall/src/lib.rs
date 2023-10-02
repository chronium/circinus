#![no_std]
#![feature(linkage)]
#![allow(non_camel_case_types)]

use core::ffi::c_char;

use syscall::{sys1, sys2, sys3, Syscall};

pub mod syscall;

pub fn exit(status: i32) -> ! {
  sys1(Syscall::Exit, status as usize);
  unreachable!()
}

pub fn open<T: AsRef<str>>(path: T, flags: usize, mode: usize) -> usize {
  sys3(Syscall::Open, path.as_ref().as_ptr() as usize, flags, mode)
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

#[linkage = "weak"]
#[no_mangle]
extern "C" fn cilibc_panic(_pi: &::core::panic::PanicInfo) -> ! {
  loop {}
}
