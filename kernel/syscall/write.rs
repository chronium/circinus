impl<'a> super::SyscallHandler<'a> {
	pub fn sys_write(
		&mut self,
		fd: api::vfs::Fd,
		uaddr: environment::address::UserVAddr,
		len: usize,
	) -> api::Result<isize> {
		let len = core::cmp::min(len, super::MAX_READ_WRITE_LEN);

		let opened_file = api::Process::get_open_file_by_fid(fd)?;
		// trace!(
		// "[{}:{}] write(file={:?}, len={})",
		// current_process().pid().as_i32(),
		// current_process().cmdline().argv0(),
		// opened_file.inode(),
		// len
		// );
		trace!(
			"[{}] write(file={:?}, len={})",
			api::Process::pid().as_i32(),
			opened_file.node(),
			len
		);

		let written_len = opened_file
			.write(api::user_buffer::UserBuffer::from_uaddr(uaddr, len))?;
		Ok(written_len as isize)
	}
}
