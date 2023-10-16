use api::{ctypes::c_int, io::OpenFlags, vfs::Fd, Error, ErrorKind, Result};

use crate::process::current_process;

use super::SyscallHandler;

const _F_DUPFD: c_int = 0;
const _F_GETFD: c_int = 1;
const F_SETFD: c_int = 2;
const _F_GETFL: c_int = 3;
const F_SETFL: c_int = 4;

impl<'a> SyscallHandler<'a> {
  pub fn sys_fcntl(&mut self, fd: Fd, cmd: c_int, arg: usize) -> Result<isize> {
    let current = current_process();
    let mut opened_files = current.opened_files().lock();

    match cmd {
      F_SETFD => {
        opened_files.get(fd)?.set_cloexec(arg == 1);
        Ok(0)
      }
      F_SETFL => {
        opened_files
          .get(fd)?
          .set_flags(OpenFlags::from_bits_truncate(arg as i32))?;
        Ok(0)
      }
      _ => Err(ErrorKind::ENOSYS.into()),
    }
  }
}
