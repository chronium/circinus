use api::{
	schema::{posix, unix::Path},
	Process,
};
use environment::address::UserVAddr;

use super::SyscallHandler;

impl<'a> SyscallHandler<'a> {
	pub fn sys_stat(&mut self, path: &Path, buf: UserVAddr) -> api::Result<isize> {
		let stat = Process::rootfs().lock().lookup(path)?.stat()?;
		let posix: posix::Stat = stat.into();

		buf.write(&posix)?;

		Ok(0)
	}
}
