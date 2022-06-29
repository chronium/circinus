#[derive(Debug, Eq, PartialEq, Hash)]
pub enum ErrorKind {
	ParseError,
	UnknownCommand,
	Unimplemented,
	InvalidArgument,
	NotADirectory,
}

pub type Result<T> = core::result::Result<T, ErrorKind>;
