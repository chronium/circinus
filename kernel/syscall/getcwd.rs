use api::{ctypes::c_size, user_buffer::UserBufWriter, ErrorKind};
use environment::address::UserVAddr;

use super::SyscallHandler;

impl<'a> SyscallHandler<'a> {
  pub fn sys_getcwd(&mut self, buf: UserVAddr, len: c_size) -> api::Result<isize> {
    let cwd = api::Process::rootfs()
      .lock()
      .cwd_path()
      .resolve_absolute_path();

    if (len as usize) < cwd.as_str().len() {
      return Err(ErrorKind::TooShort.into());
    }

    let mut writer = UserBufWriter::from_uaddr(buf, len as usize);
    writer.write_bytes(cwd.as_str().as_bytes())?;
    writer.write(0u8)?;

    Ok(buf.as_isize())
  }
}
