use alloc::sync::Arc;
use api::vfs;

use crate::ext2::Ext2;

pub struct Ext2Filesystem(pub(crate) Arc<Ext2>, pub(crate) usize);

impl Ext2Filesystem {
	pub fn ext2(&self) -> &Ext2 {
		&self.0
	}
}

impl Ext2Filesystem {
	pub fn root(&self) -> Arc<dyn vfs::Directory> {
		Ext2::root(self.0.clone())
	}
}

impl vfs::Filesystem for Ext2Filesystem {
	fn root(&self) -> api::Result<Arc<dyn vfs::Directory>> {
		Ok(self.root())
	}
}
