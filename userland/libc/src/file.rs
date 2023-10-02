use core::ops::Deref;

use crate::{
  c_str::CStr,
  io,
  platform::{sys::Sys, types::*},
};

pub struct File {
  pub fd: c_int,
  pub reference: bool,
}

impl File {
  pub fn new(fd: c_int) -> Self {
    Self {
      fd,
      reference: false,
    }
  }

  pub fn open(path: &CStr, oflag: c_int) -> core_io::Result<Self> {
    match Sys::open(path, oflag, 0) {
      -1 => todo!(), // TODO: Err(core_io::Error::last_os_error()),
      fd => Ok(Self::new(fd)),
    }
  }

  pub unsafe fn get_ref(&self) -> Self {
    Self {
      fd: self.fd,
      reference: true,
    }
  }
}

impl io::Read for File {
  fn read(&mut self, buf: &mut [u8]) -> core_io::Result<usize> {
    Ok(syscall::read(self.fd, buf))
  }
}

impl io::Write for File {
  fn write(&mut self, buf: &[u8]) -> core_io::Result<usize> {
    Ok(syscall::write(self.fd, buf))
  }

  fn flush(&mut self) -> core_io::Result<()> {
    Ok(())
  }
}

impl Deref for File {
  type Target = c_int;

  fn deref(&self) -> &Self::Target {
    &self.fd
  }
}
