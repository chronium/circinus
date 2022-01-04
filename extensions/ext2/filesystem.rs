use core::fmt;

use alloc::{boxed::Box, sync::Arc, vec::Vec};
use api::{
	io,
	schema::{
		fs::{
			self,
			file::File,
			filesystem::{Filesystem, Node},
		},
		unix,
	},
};

use crate::{
	dirent::{Dirent, DirentType},
	ext2::Ext2,
};

struct Dir {
	fs: Ext2,
	dirents: Vec<Dirent>,
}

impl fmt::Debug for Dir {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("Dir")
			.field("dirents", &self.dirents)
			.finish()
	}
}

impl fs::Directory for Dir {
	fn read_dir(&self, index: usize) -> Option<fs::DirEntry> {
		self.dirents.get(index).map(|d| d.into())
	}

	fn arc(&self) -> Arc<&dyn fs::Directory> {
		Arc::new(self)
	}

	fn _lookup(&self, path: unix::PathBuf) -> io::Result<Node> {
		let dirent = self.dirents.iter().find(|&d| d.name == path.as_str());
		if dirent.is_none() {
			return Err(io::IoError::NotFound);
		}
		let dirent = dirent.unwrap();

		let inode = self.fs.read_inode(dirent.inode as usize);
		let dirents = self.fs.read_dirent(inode);

		match dirent.dirent_type {
			DirentType::Directory => Ok(Node::Directory(Arc::new(Dir {
				fs: self.fs.clone(),
				dirents,
			}))),
			DirentType::Regular => Ok(Node::RegularFile(Arc::new(ExtFile {
				fs: self.fs.clone(),
			}))),
			_ => unimplemented!(),
		}
	}
}

struct ExtFile {
	fs: Ext2,
}

impl fmt::Debug for ExtFile {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("ExtFile").finish()
	}
}

impl File for ExtFile {
	fn open(&self, options: &io::OpenOptions) -> io::Result<Arc<dyn File>> {
		todo!()
	}
}

impl Filesystem for Ext2 {
	fn root(&self) -> Arc<dyn fs::Directory> {
		let inode = self.read_inode(2);
		let dirents = self.read_dirent(inode);

		Arc::new(Dir {
			fs: self.clone(),
			dirents,
		})
	}
}
