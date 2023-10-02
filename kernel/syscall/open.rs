use api::{
  io::OpenFlags,
  posix::CwdOrFd,
  schema::{
    posix::{FileMode, O_RDWR, O_WRONLY},
    unix::Path,
  },
  ErrorKind, Process, ProcessOps,
};

use crate::process::current_process;

use super::SyscallHandler;

impl<'a> SyscallHandler<'a> {
  pub fn sys_open(&mut self, path: &Path, flags: OpenFlags, mode: FileMode) -> api::Result<isize> {
    trace!(
      "[{}:{}] open(\"{}\")",
      api::Process::pid().as_i32(),
      api::Process::argv0(),
      path.as_str()
    );

    if flags.contains(OpenFlags::O_CREAT) {
      unimplemented!()
    }

    let path_comp = Process::rootfs().lock().lookup_path_at(
      &*Process::opened_files().lock(),
      &CwdOrFd::AtCwd,
      &path,
      true,
    )?;
    if flags.contains(OpenFlags::O_DIRECTORY) && !path_comp.node.is_dir() {
      return Err(ErrorKind::NotADirectory.into());
    }

    let access_mode = mode.access_mode();
    if path_comp.node.is_dir() && (access_mode == O_WRONLY || access_mode == O_RDWR) {
      return Err(ErrorKind::IsADirectory.into());
    }

    let fd = Process::opened_files()
      .lock()
      .open(path_comp, flags.into())?;
    Ok(fd.as_usize() as isize)
  }
}
