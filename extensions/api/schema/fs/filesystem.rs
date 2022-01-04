use alloc::sync::Arc;

use crate::{
	io::{self, IoError},
	schema::unix::PathBuf,
};

use super::{directory::Directory, file::File};

pub trait Filesystem: Send + Sync {
	fn root(&self) -> Arc<dyn Directory>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
	Directory,
	RegularFile,
}

pub enum Node {
	Directory(Arc<dyn Directory>),
	RegularFile(Arc<dyn File>),
}

impl Node {
	pub fn as_dir(&self) -> io::Result<&Arc<dyn Directory>> {
		match self {
			Self::Directory(dir) => Ok(dir),
			_ => Err(IoError::NotADirectory),
		}
	}

	pub fn as_file(&self) -> io::Result<&Arc<dyn File>> {
		match self {
			Self::RegularFile(file) => Ok(file),
			_ => Err(IoError::NotAFile),
		}
	}
}

pub struct DirEntry {
	pub path: PathBuf,
	pub ftype: FileType,
}
