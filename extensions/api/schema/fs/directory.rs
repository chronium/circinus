use core::fmt::Debug;

use alloc::sync::Arc;

use crate::{schema::unix::PathBuf, Result};

use super::{filesystem::Node, DirEntry};

pub trait Directory: Debug + Send + Sync {
	fn _lookup(&self, path: PathBuf) -> Result<Node>;

	fn read_dir(&self, index: usize) -> Option<DirEntry>;

	fn arc(&self) -> Arc<&dyn Directory>;

	fn iter(&self) -> ReadDir {
		ReadDir {
			directory: self.arc(),
			current: 0,
		}
	}
}

impl dyn Directory {
	pub fn lookup<P: Into<PathBuf>>(&self, path: P) -> Result<Node> {
		self._lookup(path.into())
	}
}

pub struct ReadDir<'a> {
	pub directory: Arc<&'a dyn Directory>,
	pub current: usize,
}

impl Iterator for ReadDir<'_> {
	type Item = DirEntry;

	fn next(&mut self) -> Option<Self::Item> {
		let res = self.directory.read_dir(self.current);
		self.current += 1;
		res
	}
}
