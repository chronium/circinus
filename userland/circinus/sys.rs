use core::{arch::asm, mem};

use alloc::vec::Vec;
use base::io::{FileMode, OpenFlags};

use crate::fs::{path::Path, stat::Stat, Fd};

#[cfg(feature = "lunix")]
#[repr(usize)]
pub enum Syscall {
	Read = 0,
	Write = 1,
	Open = 2,
	Stat = 4,
	Brk = 12,
	Exit = 60,
}

#[cfg(not(feature = "lunix"))]
#[repr(usize)]
pub enum Syscall {
	Write = 1,
	Read = 2,
	Stat = 3,
	Open = 4,
	Brk = 128,
	#[allow(clippy::enum_clike_unportable_variant)]
	Exit = -1isize as usize,
}

fn sys1(sys: Syscall, arg1: usize) -> usize {
	let mut ret;
	unsafe {
		asm!("syscall",
    in("rdi") arg1,
    in("rax") sys as usize,
    lateout("rax")  ret);
	}
	ret
}

fn sys2(sys: Syscall, arg1: usize, arg2: usize) -> usize {
	let mut ret;
	unsafe {
		asm!("syscall",
    in("rdi") arg1,
    in("rsi") arg2,
    in("rax") sys as usize,
    lateout("rax")  ret);
	}
	ret
}

fn sys3(sys: Syscall, arg1: usize, arg2: usize, arg3: usize) -> usize {
	let mut ret;
	unsafe {
		asm!("syscall",
    in("rdi") arg1,
    in("rsi") arg2,
    in("rdx") arg3,
    in("rax") sys as usize,
    lateout("rax")  ret);
	}
	ret
}

pub fn exit(status: i32) -> ! {
	sys1(Syscall::Exit, status as usize);
	unreachable!()
}

pub fn write(fd: i32, buf: &[u8]) -> usize {
	sys3(
		Syscall::Write,
		fd as usize,
		buf.as_ptr() as usize,
		buf.len(),
	)
}

pub fn write_vec<T>(fd: i32, buf: &Vec<T>) -> usize {
	sys3(
		Syscall::Write,
		fd as usize,
		buf.as_ptr() as usize,
		buf.len() * mem::size_of::<T>(),
	)
}

pub fn stat(path: &Path, buf: &mut Stat) -> usize {
	sys2(
		Syscall::Stat,
		path.as_ptr() as usize,
		buf as *mut Stat as usize,
	)
}

pub fn read(fd: i32, buf: &[u8]) -> usize {
	sys3(Syscall::Read, fd as usize, buf.as_ptr() as usize, buf.len())
}

pub fn read_obj<T>(fd: Fd, buf: &mut T) -> usize {
	sys3(
		Syscall::Read,
		fd.as_usize(),
		buf as *mut T as usize,
		mem::size_of::<T>(),
	)
}

pub fn read_ptr(fd: i32, buf: *mut u8, len: usize) -> usize {
	sys3(Syscall::Read, fd as usize, buf as usize, len)
}

pub fn brk<S: Into<Option<usize>>>(new_heap_end: S) -> usize {
	sys1(Syscall::Brk, new_heap_end.into().unwrap_or(0))
}

pub fn open(path: &Path, flags: OpenFlags, mode: FileMode) -> Fd {
	Fd::new(sys3(
		Syscall::Open,
		path.as_ptr() as usize,
		flags.bits() as usize,
		mode.0 as usize,
	) as i32)
}
