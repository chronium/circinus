pub use read::{BufRead, Read};
pub use write::Write;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
	kind: ErrorKind,
}

#[derive(Debug)]
pub enum ErrorKind {}

impl Into<Error> for ErrorKind {
	fn into(self) -> Error {
		Error { kind: self }
	}
}

use bitflags::bitflags;

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

#[allow(unused)]
pub const O_RDONLY: u32 = 0o0;
pub const O_WRONLY: u32 = 0o1;
pub const O_RDWR: u32 = 0o2;

#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct FileMode(pub u32);

impl FileMode {
	pub fn new(value: u32) -> FileMode {
		FileMode(value)
	}
}

mod read;
mod write;
