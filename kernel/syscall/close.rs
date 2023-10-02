use api::{vfs::Fd, Process};

use super::SyscallHandler;

impl<'a> SyscallHandler<'a> {
  pub fn sys_close(&mut self, fd: Fd) -> api::Result<isize> {
    Process::opened_files().lock().close(fd)?;
    Ok(0)
  }
}
