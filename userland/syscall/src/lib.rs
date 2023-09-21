#![no_std]
#![feature(linkage)]
#![allow(non_camel_case_types)]

use syscall::{sys1, sys3, Syscall};

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

pub fn read(fd: i32, buf: &[u8]) -> usize {
	sys3(Syscall::Read, fd as usize, buf.as_ptr() as usize, buf.len())
}

#[linkage = "weak"]
#[no_mangle]
extern "C" fn cilibc_panic(_pi: &::core::panic::PanicInfo) -> ! {
	loop {}
}
