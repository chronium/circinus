use crate::bitflags::bitflags;

bitflags! {
  pub struct OpenFlags: i32 {
	  const O_RDONLY = 0o0;
	  const O_WRONLY = 0o1;
	  const O_RDWR = 0o2;
	  const O_CREAT = 0o100;
	  const O_EXCL = 0o200;
	  const O_NOCTTY = 0o400;
	  const O_TRUNC = 0o1000;
	  const O_APPEND = 0o2000;
	  const O_NONBLOCK = 0o4000;
	  const O_DIRECTORY = 0o200000;
	  const O_CLOEXEC  = 0o2000000;
  }
}

#[derive(Debug, Copy, Clone)]
pub struct OpenOptions {
	pub nonblock: bool,
	pub close_on_exec: bool,
}

impl OpenOptions {
	pub fn new(nonblock: bool, cloexec: bool) -> OpenOptions {
		OpenOptions {
			nonblock,
			close_on_exec: cloexec,
		}
	}

	pub fn empty() -> OpenOptions {
		OpenOptions {
			nonblock: false,
			close_on_exec: false,
		}
	}

	pub fn readwrite() -> OpenOptions {
		OpenOptions {
			nonblock: false,
			close_on_exec: false,
		}
	}
}

impl From<OpenFlags> for OpenOptions {
	fn from(flags: OpenFlags) -> OpenOptions {
		OpenOptions {
			nonblock: flags.contains(OpenFlags::O_NONBLOCK),
			close_on_exec: flags.contains(OpenFlags::O_CLOEXEC),
		}
	}
}
