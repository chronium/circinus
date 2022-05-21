use core::arch::asm;

#[repr(usize)]
pub enum Syscall {
	Write = 1,
	Read = 2,
	Brk = 128,
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

pub fn read(fd: i32, buf: &mut [u8]) -> usize {
	sys3(Syscall::Read, fd as usize, buf.as_ptr() as usize, buf.len())
}

pub fn brk<S: Into<Option<usize>>>(new_heap_end: S) -> usize {
	sys1(Syscall::Brk, new_heap_end.into().unwrap_or(0))
}