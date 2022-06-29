use alloc::sync::Arc;

use crate::{schema::unix::PathBuf, ErrorKind, Result};

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
	pub fn as_dir(&self) -> Result<&Arc<dyn Directory>> {
		match self {
			Self::Directory(dir) => Ok(dir),
			_ => Err(ErrorKind::NotADirectory.into()),
		}
	}

	pub fn as_file(&self) -> Result<&Arc<dyn File>> {
		match self {
			Self::RegularFile(file) => Ok(file),
			_ => Err(ErrorKind::NotAFile.into()),
		}
	}
}

pub struct DirEntry {
	pub path: PathBuf,
	pub ftype: FileType,
}
