use api::{
  bitflags::bitflags,
  ctypes::c_int,
  process::{Pid, ProcessState},
};
use environment::address::UserVAddr;

use crate::process::{current_process, JOIN_WAIT_QUEUE};

use super::SyscallHandler;

bitflags! {
    pub struct WaitOptions: c_int {
        const WNOHANG   = 1;
        const WUNTRACED = 2;
    }
}

impl<'a> SyscallHandler<'a> {
  pub fn sys_wait4(
    &mut self,
    pid: Pid,
    status: Option<UserVAddr>,
    options: WaitOptions,
    _rusage: Option<UserVAddr>,
  ) -> api::Result<isize> {
    let (got_pid, status_value) = JOIN_WAIT_QUEUE.sleep_signalable_until(|| {
      let current = current_process();
      for child in current.children().iter() {
        if pid.as_i32() > 0 && child.pid() != pid {
          continue;
        }

        if pid.as_i32() == 0 {
          // TODO: Wait for any children in the same process group
          todo!("Wait for any children in the same process group")
        }

        if let ProcessState::Exited(status_value) = child.state() {
          return Ok(Some((child.pid(), status_value)));
        }
      }

      if options.contains(WaitOptions::WNOHANG) {
        return Ok(Some((Pid::new(0), 0)));
      }

      Ok(None)
    })?;

    current_process().children().retain(|p| p.pid() != got_pid);

    if let Some(status) = status {
      // FIXME: This is not the correct format of `status`
      status.write::<c_int>(&status_value)?;
    }

    Ok(got_pid.as_i32() as isize)
  }
}
