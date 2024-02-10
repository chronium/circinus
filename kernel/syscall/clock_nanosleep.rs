use api::{
  ctypes::{c_clockid, c_int},
  ErrorKind, Result,
};
use environment::address::UserVAddr;

use crate::timer;

use super::SyscallHandler;

impl<'a> SyscallHandler<'a> {
  pub fn sys_clock_nanosleep(
    &mut self,
    clock: c_clockid,
    flags: c_int,
    rqtp: UserVAddr,
    rmtp: Option<UserVAddr>,
  ) -> Result<isize> {
    if flags != 0 {
      return Err(ErrorKind::ENOSYS.into());
    }

    if rmtp.is_some() {
      return Err(ErrorKind::ENOSYS.into());
    }

    match clock {
      CLOCK_REALTIME => {
        let rqtp = rqtp.read::<timer::Timespec>()?;
        timer::sleep_timespec(rqtp);

        // TODO: write remaining time to rmtp
        Ok(0)
      }
      _ => {
        debug_warn!("clock_nanosleep: unsupported clock id: {}", clock);
        Err(ErrorKind::ENOSYS.into())
      }
    }
  }
}
