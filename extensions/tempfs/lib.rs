#![no_std]

#[allow(unused_imports)]
#[macro_use]
extern crate alloc;

use core::sync::atomic::{AtomicUsize, Ordering};

use alloc::{borrow::ToOwned, fmt::Debug, string::String, sync::Arc, vec::Vec};
use api::{
	hashbrown::HashMap,
	io,
	sync::SpinLock,
	user_buffer::{UserBufReader, UserBufWriter},
	vfs::{self, NodeId, Stat},
	ErrorKind, Result,
};

#[derive(Debug)]
pub struct Tempfs {
	dir: Arc<TempfsDirectory>,
}

impl Tempfs {
	pub fn new() -> Self {
		Self {
			dir: Arc::new(TempfsDirectory::new(Self::alloc_inode_no())),
		}
	}

	pub fn new_root() -> Self {
		Self {
			dir: Arc::new(TempfsDirectory::new(NodeId::new(1))),
		}
	}

	pub fn root(&self) -> &Arc<TempfsDirectory> {
		&self.dir
	}

	pub fn alloc_inode_no() -> NodeId {
		// Inode #1 is reserved for the root dir.
		static NEXT_INODE_NO: AtomicUsize = AtomicUsize::new(2);

		NodeId::new(NEXT_INODE_NO.fetch_add(1, Ordering::Relaxed))
	}
}

impl vfs::Filesystem for Tempfs {
	fn root(&self) -> Result<Arc<dyn vfs::Directory>> {
		Ok(self.dir.clone() as Arc<dyn vfs::Directory>)
	}
}

#[derive(Debug)]
struct DirectoryInner {
	files: HashMap<String, TempfsNode>,
	stat: Stat,
}

#[derive(Debug)]
pub struct TempfsDirectory(SpinLock<DirectoryInner>);

impl TempfsDirectory {
	pub fn new(node: NodeId) -> Self {
		Self(SpinLock::new(DirectoryInner {
			files: HashMap::new(),
			stat: Stat {
				node_id: node,
				size: 0,
				kind: vfs::FileKind::Directory,
			},
		}))
	}

	pub fn add_dir(&self, name: &str) -> Arc<Self> {
		let dir = Arc::new(Self::new(Tempfs::alloc_inode_no()));
		self.0
			.lock()
			.files
			.insert(name.to_owned(), TempfsNode::Directory(dir.clone()));
		dir
	}

	pub fn add_file<S: AsRef<str>>(&self, name: S, file: Arc<dyn vfs::File>) {
		self.0
			.lock()
			.files
			.insert(name.as_ref().to_owned(), TempfsNode::File(file));
	}
}

impl vfs::Directory for TempfsDirectory {
	fn read_dir(&self, index: usize) -> Result<Option<vfs::DirEntry>> {
		let dir_lock = self.0.lock();

		let (name, node) = match dir_lock.files.iter().nth(index) {
			Some((name, node)) => (name, node),
			None => return Ok(None),
		};

		let entry = match node {
			TempfsNode::Directory(dir) => {
				let dir = dir.0.lock();
				vfs::DirEntry {
					node_id: dir.stat.node_id,
					file_type: vfs::FileType::Directory,
					name: name.clone(),
				}
			}
			TempfsNode::File(file) => vfs::DirEntry {
				node_id: file.stat()?.node_id,
				file_type: vfs::FileType::RegularFile,
				name: name.clone(),
			},
		};

		Ok(Some(entry))
	}

	fn _lookup(&self, name: &str) -> Result<vfs::Node> {
		self.0
			.lock()
			.files
			.get(name)
			.map(|tempfs_node| match tempfs_node {
				TempfsNode::File(file) => (file.clone() as Arc<dyn vfs::File>).into(),
				TempfsNode::Directory(dir) => (dir.clone() as Arc<dyn vfs::Directory>).into(),
			})
			.ok_or_else(|| ErrorKind::NoEntry.into())
	}

	fn stat(&self) -> Result<vfs::Stat> {
		Ok(self.0.lock().stat)
	}
}

#[derive(Debug)]
pub enum TempfsNode {
	File(Arc<dyn vfs::File>),
	Directory(Arc<TempfsDirectory>),
}

pub struct InMemoryFile {
	data: SpinLock<Vec<u8>>,
	stat: Stat,
}

#[derive(Debug)]
pub enum Snip {
	Snip,
}

impl Debug for InMemoryFile {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("InMemoryFile")
			.field("data", &self.data.lock()[..16].to_vec())
			.field("data_rest", &"<snip>")
			.finish()
	}
}

impl InMemoryFile {
	pub fn new(data: &[u8]) -> Self {
		Self {
			data: SpinLock::new(data.to_owned()),
			stat: Stat {
				node_id: Tempfs::alloc_inode_no(),
				size: data.len(),
				kind: vfs::FileKind::RegularFile,
			},
		}
	}
}

impl vfs::File for InMemoryFile {
	fn open(&self, _options: &io::OpenOptions) -> Result<Option<Arc<dyn vfs::File>>> {
		Ok(None)
	}

	fn read(
		&self,
		offset: usize,
		buf: api::user_buffer::UserBufferMut<'_>,
		_options: &io::OpenOptions,
	) -> Result<usize> {
		let data = self.data.lock();

		if offset > data.len() {
			return Ok(0);
		}

		let mut writer = UserBufWriter::from(buf);
		writer
			.write_bytes(&data[offset..])
			.map_err(|_| ErrorKind::BufferError.into())
	}

	fn write(
		&self,
		offset: usize,
		buf: api::user_buffer::UserBuffer<'_>,
		_options: &io::OpenOptions,
	) -> Result<usize> {
		let mut data = self.data.lock();
		let mut reader = UserBufReader::from(buf);
		data.resize(offset + reader.remaining_len(), 0);
		reader
			.read_bytes(&mut data[offset..])
			.map_err(|_| ErrorKind::BufferError.into())
	}

	fn stat(&self) -> Result<vfs::Stat> {
		Ok(self.stat)
	}
}

#[derive(Debug)]
pub struct InMemoryTextFile {
	data: SpinLock<String>,
	stat: Stat,
}

impl InMemoryTextFile {
	pub fn new<S: AsRef<str>>(data: S) -> Self {
		let sdata = data.as_ref().to_owned();
		Self {
			stat: Stat {
				node_id: Tempfs::alloc_inode_no(),
				size: sdata.len(),
				kind: vfs::FileKind::RegularFile,
			},
			data: SpinLock::new(sdata),
		}
	}
}

impl vfs::File for InMemoryTextFile {
	fn open(&self, _options: &io::OpenOptions) -> Result<Option<Arc<dyn vfs::File>>> {
		Ok(None)
	}

	fn read(
		&self,
		offset: usize,
		dst: api::user_buffer::UserBufferMut<'_>,
		_options: &io::OpenOptions,
	) -> Result<usize> {
		let data = self.data.lock();

		if offset > data.len() {
			return Ok(0);
		}

		let mut writer = UserBufWriter::from(dst);
		writer
			.write_bytes(&data[offset..].as_ref())
			.map_err(|_| ErrorKind::BufferError.into())
	}

	fn write(
		&self,
		offset: usize,
		buf: api::user_buffer::UserBuffer<'_>,
		_options: &io::OpenOptions,
	) -> Result<usize> {
		let mut data = self.data.lock();
		let mut reader = UserBufReader::from(buf);
		data.reserve_exact(offset + reader.remaining_len());
		reader
			.read_bytes(unsafe { &mut data.as_bytes_mut()[offset..] })
			.map_err(|_| ErrorKind::BufferError.into())
	}

	fn stat(&self) -> Result<vfs::Stat> {
		Ok(self.stat)
	}
}
