use api::ProcessOps;

use crate::process::current_process;

impl<'a> super::SyscallHandler<'a> {
  pub fn sys_write(
    &mut self,
    fd: api::vfs::Fd,
    uaddr: environment::address::UserVAddr,
    len: usize,
  ) -> api::Result<isize> {
    let len = core::cmp::min(len, super::MAX_READ_WRITE_LEN);

    let file_table = current_process().opened_files().lock();
    let opened_file = file_table.get(fd)?;
    trace!(
      "[{}:{}] write(file={:?}, len={})",
      api::Process::pid().as_i32(),
      api::Process::argv0(),
      opened_file.node(),
      len
    );

    let written_len = opened_file.write(api::user_buffer::UserBuffer::from_uaddr(uaddr, len))?;
    Ok(written_len as isize)
  }
}
