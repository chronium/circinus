use api::{
  ctypes::{c_clockid, c_int, c_size},
  io::OpenFlags,
  kernel::KernelOps,
  process::Pid,
  schema::{
    posix::FileMode,
    unix::{Path, PathBuf},
  },
  user_buffer::UserCStr,
  vfs::Fd,
  Error, ErrorKind, ProcessOps, Result,
};
use environment::{address::UserVAddr, arch::PtRegs};

use crate::process::current_process;

use self::wait4::WaitOptions;

const SYS_WRITE: usize = 1;
const SYS_READ: usize = 2;
const SYS_STAT: usize = 3;
const SYS_OPEN: usize = 4;
const SYS_EXECVE: usize = 5;
const SYS_GETCWD: usize = 6;
const SYS_CHDIR: usize = 7;
const SYS_CLOSE: usize = 8;
const SYS_GETDENTS64: usize = 9;
const SYS_FCNTL: usize = 10;
const SYS_LSEEK: usize = 11;
const SYS_CLOCK_GETTIME: usize = 12;
const SYS_CLOCK_NANOSLEEP: usize = 13;
const SYS_WAIT4: usize = 126;
const SYS_FORK: usize = 127;
const SYS_BRK: usize = 128;
const SYS_EXIT: usize = -1isize as usize;

pub(self) const MAX_READ_WRITE_LEN: usize = core::isize::MAX as usize;

fn resolve_path(uaddr: usize) -> Result<PathBuf> {
  const PATH_MAX: usize = 512;
  Ok(Path::new(UserCStr::new(UserVAddr::new_nonnull(uaddr)?, PATH_MAX)?.as_str()).to_path_buf())
}

pub struct SyscallHandler<'a> {
  pub frame: &'a mut PtRegs,
}

impl<'a> SyscallHandler<'a> {
  pub fn new(frame: &'a mut PtRegs) -> SyscallHandler<'a> {
    SyscallHandler { frame }
  }

  pub fn dispatch(
    &mut self,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    a6: usize,
    n: usize,
  ) -> Result<isize> {
    let ret = self.do_dispatch(a1, a2, a3, a4, a5, a6, n).map_err(|err| {
      debug_warn!("{}: error: {:?}", syscall_name_by_number(n), err);
      err
    });

    // TODO:
    // if let Err(err) = Process::try_delivering_signal(self.frame) {
    // debug_warn!("failed to setup the signal stack: {:?}", err);
    // }

    ret
  }

  pub fn do_dispatch(
    &mut self,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    a6: usize,
    n: usize,
  ) -> Result<isize> {
    match n {
      SYS_WRITE => self.sys_write(Fd::new(a1 as i32), UserVAddr::new_nonnull(a2)?, a3),
      SYS_READ => self.sys_read(Fd::new(a1 as i32), UserVAddr::new_nonnull(a2)?, a3),
      SYS_STAT => self.sys_stat(&resolve_path(a1)?, UserVAddr::new_nonnull(a2)?),
      SYS_OPEN => self.sys_open(
        &resolve_path(a1)?,
        bitflags_from_user!(OpenFlags, a2 as i32)?,
        FileMode::new(a3 as u32),
      ),
      SYS_EXIT => self.sys_exit(a1 as c_int),
      SYS_EXECVE => self.sys_execve(
        &resolve_path(a1)?,
        UserVAddr::new_nonnull(a2)?,
        UserVAddr::new_nonnull(a3)?,
      ),
      SYS_GETCWD => self.sys_getcwd(UserVAddr::new_nonnull(a1)?, a2 as c_size),
      SYS_BRK => self.sys_brk(UserVAddr::new(a1)),
      SYS_CHDIR => self.sys_chdir(&resolve_path(a1)?),
      SYS_CLOSE => self.sys_close(Fd::new(a1 as i32)),
      SYS_GETDENTS64 => self.sys_getdents64(Fd::new(a1 as i32), UserVAddr::new_nonnull(a2)?, a3),
      SYS_FCNTL => self.sys_fcntl(Fd::new(a1 as i32), a2 as c_int, a3),
      SYS_LSEEK => self.sys_lseek(Fd::new(a1 as i32), a2 as isize, a3 as isize),
      SYS_CLOCK_GETTIME => self.sys_clock_gettime(a1 as c_clockid, UserVAddr::new_nonnull(a2)?),
      SYS_CLOCK_NANOSLEEP => self.sys_clock_nanosleep(
        a1 as c_clockid,
        a2 as c_int,
        UserVAddr::new_nonnull(a3)?,
        UserVAddr::new(a4),
      ),
      SYS_WAIT4 => self.sys_wait4(
        Pid::new(a1 as i32),
        UserVAddr::new(a2),
        bitflags_from_user!(WaitOptions, a3 as c_int)?,
        UserVAddr::new(a4),
      ),
      SYS_FORK => self.sys_fork(),
      _ => {
        debug_warn!(
          "unimplemented system call: {} (n={})",
          syscall_name_by_number(n),
          n,
        );
        Err(Error::new(ErrorKind::NoSyscall))
      }
    }
  }
}

fn syscall_name_by_number(n: usize) -> &'static str {
  match n {
    1 => "write",
    2 => "read",
    3 => "stat",
    4 => "open",
    5 => "exec",
    6 => "getcwd",
    7 => "chdir",
    8 => "close",
    9 => "getdents64",
    10 => "fcntl",
    11 => "lseek",
    12 => "clock_gettime",
    13 => "clock_nanosleep",
    126 => "wait4",
    127 => "fork",
    128 => "brk",
    SYS_EXIT => "exit",
    _ => "(unknown)",
  }
}

pub(self) mod brk;
pub(self) mod chdir;
pub(self) mod clock_gettime;
pub(self) mod clock_nanosleep;
pub(self) mod close;
pub(self) mod execve;
pub(self) mod exit;
pub(self) mod fcntl;
pub(self) mod fork;
pub(self) mod getcwd;
pub(self) mod getdents64;
pub(self) mod lseek;
pub(self) mod open;
pub(self) mod read;
pub(self) mod stat;
pub(self) mod wait4;
pub(self) mod write;
