use alloc::sync::Arc;

use crate::{
	ctypes::c_int,
	io,
	schema::unix::PathBuf,
	user_buffer::{UserBuffer, UserBufferMut},
	ErrorKind, Result,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct NodeId(usize);

impl NodeId {
	pub const fn new(no: usize) -> Self {
		Self(no)
	}

	pub const fn as_u64(self) -> u64 {
		self.0 as u64
	}
}

const FD_MAX: c_int = 1024;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Fd(c_int);

impl Fd {
	pub const fn new(value: i32) -> Fd {
		Fd(value)
	}

	pub const fn as_int(self) -> c_int {
		self.0
	}

	pub const fn as_usize(self) -> usize {
		self.0 as usize
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
	Directory,
	RegularFile,
}

pub trait Filesystem: Send + Sync {
	fn root(&self) -> Result<Arc<dyn Directory>>;
}

pub struct DirEntry {
	pub path: PathBuf,
	pub ftype: FileType,
}

pub trait Directory: Send + Sync + core::fmt::Debug {
	fn _lookup(&self, name: &str) -> Result<Node>;

	fn read_dir(&self, index: usize) -> Option<DirEntry>;

	fn stat(&self) -> Result<Stat>;
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

pub trait File: Send + Sync + core::fmt::Debug {
	fn open(&self, options: &io::OpenOptions) -> Result<Option<Arc<dyn File>>>;

	fn read(
		&self,
		offset: usize,
		dst: UserBufferMut<'_>,
		options: &io::OpenOptions,
	) -> Result<usize>;

	fn write(
		&self,
		offset: usize,
		buf: UserBuffer<'_>,
		options: &io::OpenOptions,
	) -> Result<usize>;
}

#[derive(Debug)]
pub enum Node {
	Directory(Arc<dyn Directory>),
	File(Arc<dyn File>),
}

impl Clone for Node {
	fn clone(&self) -> Self {
		match self {
			Self::Directory(dir) => Self::Directory(dir.clone()),
			Self::File(file) => Self::File(file.clone()),
		}
	}
}

impl Node {
	pub fn as_dir(&self) -> Result<&Arc<dyn Directory>> {
		match self {
			Node::Directory(dir) => Ok(dir),
			_ => Err(ErrorKind::NotADirectory.into()),
		}
	}

	pub fn as_file(&self) -> Result<&Arc<dyn File>> {
		match self {
			Node::File(file) => Ok(file),
			_ => Err(ErrorKind::NotAFile.into()),
		}
	}

	pub fn is_dir(&self) -> bool {
		matches!(self, Node::Directory(_))
	}

	pub fn is_file(&self) -> bool {
		matches!(self, Node::File(_))
	}
}

impl From<Arc<dyn File>> for Node {
	fn from(file: Arc<dyn File>) -> Self {
		Self::File(file)
	}
}

impl From<Arc<dyn Directory>> for Node {
	fn from(dir: Arc<dyn Directory>) -> Self {
		Self::Directory(dir)
	}
}

#[derive(Debug, Copy, Clone)]
pub struct Stat {
	pub node_id: NodeId,
}

pub mod interface;
pub mod mount;
pub mod opened_file;
