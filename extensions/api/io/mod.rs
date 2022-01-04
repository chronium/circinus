pub type Result<T> = core::result::Result<T, IoError>;

#[derive(Debug)]
pub enum IoError {
	NotFound,
	NotADirectory,
	NotAFile,
}

pub enum OpenOptions {
	Read,
	Write,
	ReadWrite,
}
