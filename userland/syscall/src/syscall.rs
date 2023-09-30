use core::arch::asm;

#[allow(clippy::enum_clike_unportable_variant)]
#[repr(usize)]
pub(crate) enum Syscall {
	Write = 1,
	Read = 2,
	Stat = 3,
	Open = 4,
	Execve = 5,
	Brk = 128,
	Exit = -1isize as usize,
}

pub(crate) fn sys1(sys: Syscall, arg1: usize) -> usize {
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

pub(crate) fn sys3(sys: Syscall, arg1: usize, arg2: usize, arg3: usize) -> usize {
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
