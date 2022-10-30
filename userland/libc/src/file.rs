use crate::{io, platform::types::*};

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
