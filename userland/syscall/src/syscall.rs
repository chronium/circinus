use core::arch::asm;

#[allow(clippy::enum_clike_unportable_variant)]
#[repr(usize)]
pub(crate) enum Syscall {
  Write = 1,
  Read = 2,
  Stat = 3,
  Open = 4,
  Execve = 5,
  GetCwd = 6,
  Chdir = 7,
  Close = 8,
  GetDents64 = 9,
  Wait4 = 126,
  Fork = 127,
  Brk = 128,
  Exit = -1isize as usize,
}

pub(crate) fn sys(sys: Syscall) -> usize {
  let mut ret;
  unsafe {
    asm!("syscall",
    in("rax") sys as usize,
    lateout("rax")  ret);
  }
  ret
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

pub(crate) fn sys2(sys: Syscall, arg1: usize, arg2: usize) -> usize {
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

pub(crate) fn sys4(sys: Syscall, arg1: usize, arg2: usize, arg3: usize, arg4: usize) -> usize {
  let mut ret;
  unsafe {
    asm!("syscall",
    in("rdi") arg1,
    in("rsi") arg2,
    in("rdx") arg3,
    in("r10") arg4,
    in("rax") sys as usize,
    lateout("rax")  ret);
  }
  ret
}
