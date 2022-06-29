use alloc::{sync::Arc, vec::Vec};

use atomic_refcell::AtomicRefCell;
use crossbeam::atomic::AtomicCell;

use crate::{
	io,
	user_buffer::{UserBuffer, UserBufferMut},
	vfs::{interface::PathComponent, Node},
	Error, ErrorKind, Result,
};

use super::{Directory, Fd, File, FD_MAX};

pub struct OpenedFile {
	path: Arc<PathComponent>,
	pos: AtomicCell<usize>,
	options: AtomicRefCell<io::OpenOptions>,
}

impl OpenedFile {
	pub fn new(
		path: Arc<PathComponent>,
		options: io::OpenOptions,
		pos: usize,
	) -> Self {
		Self {
			path,
			pos: AtomicCell::new(pos),
			options: AtomicRefCell::new(options),
		}
	}

	pub fn as_file(&self) -> Result<&Arc<dyn File>> {
		self.path.node.as_file()
	}

	pub fn as_dir(&self) -> Result<&Arc<dyn Directory>> {
		self.path.node.as_dir()
	}

	pub fn pos(&self) -> usize {
		self.pos.load()
	}

	pub fn options(&self) -> io::OpenOptions {
		*self.options.borrow()
	}

	pub fn node(&self) -> &Node {
		&self.path.node
	}

	pub fn read(&self, buf: UserBufferMut<'_>) -> Result<usize> {
		// Avoid holding self.options and self.pos locks by copying.
		let options = self.options();
		let pos = self.pos();

		let written_len = self.as_file()?.read(pos, buf, &options)?;
		self.pos.fetch_add(written_len);
		Ok(written_len)
	}

	pub fn write(&self, buf: UserBuffer<'_>) -> Result<usize> {
		// Avoid holding self.options and self.pos locks by copying.
		let options = self.options();
		let pos = self.pos();

		let written_len = self.as_file()?.write(pos, buf, &options)?;
		self.pos.fetch_add(written_len);
		Ok(written_len)
	}
}

#[derive(Clone)]
struct LocalOpenedFile {
	opened_file: Arc<OpenedFile>,
	close_on_exec: bool,
}

#[derive(Clone)]
pub struct OpenedFileTable {
	files: Vec<Option<LocalOpenedFile>>,
	prev_fd: i32,
}

impl OpenedFileTable {
	pub fn new() -> OpenedFileTable {
		OpenedFileTable {
			files: Vec::new(),
			prev_fd: 1,
		}
	}

	pub fn get(&self, fd: Fd) -> Result<&Arc<OpenedFile>> {
		match self.files.get(fd.as_usize()) {
			Some(Some(LocalOpenedFile { opened_file, .. })) => Ok(opened_file),
			_ => Err(ErrorKind::BadFile.into()),
		}
	}

	pub fn open_with_fixed_fd(
		&mut self,
		fd: Fd,
		mut opened_file: Arc<OpenedFile>,
		options: io::OpenOptions,
	) -> Result<()> {
		if let Node::File(file) = &opened_file.path.node {
			if let Some(new_node) = file.open(&options)? {
				opened_file = Arc::new(OpenedFile {
					pos: AtomicCell::new(0),
					options: AtomicRefCell::new(options),
					path: Arc::new(PathComponent {
						name: opened_file.path.name.clone(),
						parent_dir: opened_file.path.parent_dir.clone(),
						node: new_node.into(),
					}),
				})
			}
		}

		match self.files.get_mut(fd.as_usize()) {
			Some(Some(_)) => {
				return Err(Error::with_message(
					ErrorKind::BadFile.into(),
					"already opened at the fd",
				));
			}
			Some(entry @ None) => {
				*entry = Some(LocalOpenedFile {
					opened_file,
					close_on_exec: options.close_on_exec,
				});
			}
			None if fd.as_int() >= FD_MAX => {
				return Err(ErrorKind::BadFile.into());
			}
			None => {
				self.files.resize(fd.as_usize() + 1, None);
				self.files[fd.as_usize()] = Some(LocalOpenedFile {
					opened_file,
					close_on_exec: options.close_on_exec,
				});
			}
		}

		Ok(())
	}
}

impl Default for OpenedFileTable {
	fn default() -> OpenedFileTable {
		OpenedFileTable::new()
	}
}
