use api::{schema::unix::Path, Process};

use super::SyscallHandler;

impl<'a> SyscallHandler<'a> {
  pub fn sys_chdir(&mut self, path: &Path) -> api::Result<isize> {
    Process::rootfs().lock().chdir(path)?;
    Ok(0)
  }
}
