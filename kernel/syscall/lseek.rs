use crate::process::current_process;

use super::SyscallHandler;
use api::{vfs::Fd, ErrorKind, Result};

pub const SEEK_SET: isize = 0;
pub const SEEK_CUR: isize = 1;
pub const SEEK_END: isize = 2;

impl<'a> SyscallHandler<'a> {
  pub fn sys_lseek(&mut self, fd: Fd, offset: isize, whence: isize) -> Result<isize> {
    let file_table = current_process().opened_files().lock();
    let file = file_table.get(fd)?;
    let new_pos = match whence {
      SEEK_SET => offset as usize,
      SEEK_CUR => file.pos() + offset as usize,
      SEEK_END => file.size()? + offset as usize,
      _ => return Err(ErrorKind::EINVAL.into()), // TODO: return_errno!(EINVAL, "invalid whence"),
    };
    file.seek(new_pos)?;
    Ok(new_pos as isize)
  }
}
