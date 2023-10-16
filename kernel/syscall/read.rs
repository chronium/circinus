use crate::process::current_process;

impl<'a> super::SyscallHandler<'a> {
  pub fn sys_read(
    &mut self,
    fd: api::vfs::Fd,
    uaddr: environment::address::UserVAddr,
    len: usize,
  ) -> api::Result<isize> {
    let len = core::cmp::min(len, super::MAX_READ_WRITE_LEN);

    let file_table = current_process().opened_files().lock();
    let opened_file = file_table.get(fd)?;
    trace!(
      "[{}:{}] read(file={:?}, len={})",
      api::Process::pid().as_i32(),
      api::Process::argv0(),
      opened_file.node(),
      len
    );

    let read_len = opened_file.read(api::user_buffer::UserBufferMut::from_uaddr(uaddr, len))?;

    Ok(read_len as isize)
  }
}
