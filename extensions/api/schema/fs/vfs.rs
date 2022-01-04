use alloc::{boxed::Box, sync::Arc};
use environment::spinlock::SpinLock;

use crate::schema::unix::PathBuf;

use super::filesystem::Filesystem;

#[derive(Clone)]
pub struct Mountpoint {
	path: PathBuf,
}

pub struct Vfs {
	root: Option<Arc<SpinLock<Box<dyn Filesystem>>>>,
}

impl Vfs {
	pub(crate) fn new() -> Self {
		Self { root: None }
	}

	pub fn mount_root(
		&mut self,
		root: Arc<SpinLock<Box<dyn Filesystem>>>,
	) -> Mountpoint {
		self.root.replace(root);
		Mountpoint { path: "/".into() }
	}

	pub fn filesystem(
		&self,
		mountpoint: &Mountpoint,
	) -> Arc<SpinLock<Box<dyn Filesystem>>> {
		if mountpoint.path == "/".into() {
			return self.root.clone().unwrap();
		}

		todo!()
	}
}
