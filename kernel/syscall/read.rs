impl<'a> super::SyscallHandler<'a> {
	pub fn sys_read(
		&mut self,
		fd: api::vfs::Fd,
		uaddr: environment::address::UserVAddr,
		len: usize,
	) -> api::Result<isize> {
		let len = core::cmp::min(len, super::MAX_READ_WRITE_LEN);

		let opened_file = api::Process::get_open_file_by_fid(fd)?;
		// trace!(
		//         "[{}:{}] read(file={:?}, len={})",
		//         current_process().pid().as_i32(),
		//         current_process().cmdline().argv0(),
		//         opened_file.inode(),
		//         len
		//     );
		trace!(
			"[{}] read(file={:?}, len={})",
			api::Process::pid().as_i32(),
			opened_file.node(),
			len
		);

		let read_len = opened_file
			.read(api::user_buffer::UserBufferMut::from_uaddr(uaddr, len))?;

		Ok(read_len as isize)
	}
}
