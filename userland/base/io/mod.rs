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

mod read;
mod write;
