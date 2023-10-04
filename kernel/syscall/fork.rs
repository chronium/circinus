use crate::process::{current_process, Process};

use super::SyscallHandler;

impl<'a> SyscallHandler<'a> {
  pub fn sys_fork(&mut self) -> api::Result<isize> {
    trace!("sys_fork: current process: {:?}", current_process().pid());
    Process::fork(current_process(), self.frame).map(|child| child.pid().as_i32() as isize)
  }
}
