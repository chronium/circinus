use alloc::{borrow::ToOwned, string::String, sync::Arc};
use hashbrown::HashMap;

use crate::{schema::unix::Path, ErrorKind, Result};

use super::{interface::PathComponent, Directory, Filesystem, Node, NodeId};

const DEFAULT_SYMLINK_FOLLOW_MAX: usize = 8;

pub struct MountPoint {
	fs: Arc<dyn Filesystem>,
}

pub struct Rootfs {
	root_path: Arc<PathComponent>,
	cwd_path: Arc<PathComponent>,
	mount_points: HashMap<NodeId, MountPoint>,
	symlink_follow_limit: usize,
}

impl Rootfs {
	pub fn new(root: Arc<dyn Filesystem>) -> Result<Rootfs> {
		let root_path = Arc::new(PathComponent {
			parent_dir: None,
			name: String::new(),
			node: root.root()?.into(),
		});

		Ok(Rootfs {
			mount_points: HashMap::new(),
			root_path: root_path.clone(),
			cwd_path: root_path,
			symlink_follow_limit: DEFAULT_SYMLINK_FOLLOW_MAX,
		})
	}

	pub fn mount(
		&mut self,
		dir: Arc<dyn Directory>,
		fs: Arc<dyn Filesystem>,
	) -> Result<()> {
		self.mount_points
			.insert(dir.stat()?.node_id, MountPoint { fs });
		Ok(())
	}

	pub fn lookup_path<P: AsRef<Path>>(
		&self,
		path: P,
		follow_symlink: bool,
	) -> Result<Arc<PathComponent>> {
		let lookup_from = if path.as_ref().is_absolute() {
			self.root_path.clone()
		} else {
			self.cwd_path.clone()
		};

		self.do_lookup_path(
			&lookup_from,
			path,
			follow_symlink,
			self.symlink_follow_limit,
		)
	}

	fn do_lookup_path<P: AsRef<Path>>(
		&self,
		lookup_from: &Arc<PathComponent>,
		path: P,
		follow_symlink: bool,
		symlink_follow_limit: usize,
	) -> Result<Arc<PathComponent>> {
		let path = path.as_ref();

		if path.is_empty() {
			return Err(ErrorKind::NoEntry.into());
		}

		let mut parent_dir = lookup_from.clone();

		let mut components = path.components().peekable();
		while let Some(name) = components.next() {
			let path_comp = match name {
				"." => continue,
				".." => parent_dir
					.parent_dir
					.as_ref()
					.unwrap_or(&self.root_path)
					.clone(),
				_ => {
					let node = match parent_dir.node.as_dir()?._lookup(name)? {
						Node::Directory(dir) => match self
							.lookup_mount_point(&dir)?
						{
							Some(mount_point) => mount_point.fs.root()?.into(),
							None => dir.into(),
						},
						node => node,
					};

					Arc::new(PathComponent {
						parent_dir: Some(parent_dir.clone()),
						name: name.to_owned(),
						node,
					})
				}
			};

			if components.peek().is_some() {
				parent_dir = match &path_comp.node {
					Node::Directory(_) => path_comp,
					// TODO: Node::Symlink
					Node::File(_) => {
						return Err(ErrorKind::NotADirectory.into());
					}
				}
			} else {
				match &path_comp.node {
					// TODO: Node::Symlink
					_ => return Ok(path_comp),
				}
			}
		}

		Ok(parent_dir)
	}

	fn lookup_mount_point(
		&self,
		dir: &Arc<dyn Directory>,
	) -> Result<Option<&MountPoint>> {
		let stat = dir.stat()?;
		Ok(self.mount_points.get(&stat.node_id))
	}
}
