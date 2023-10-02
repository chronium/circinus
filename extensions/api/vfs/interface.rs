use core::fmt;

use alloc::{borrow::ToOwned, string::String, sync::Arc};

use crate::schema::unix::PathBuf;

use super::Node;

#[derive(Clone)]
pub struct PathComponent {
	/// The parent directory. `None` if this is the root directory.
	pub parent_dir: Option<Arc<PathComponent>>,
	/// The component name (e.g. `tmp` or `foo.txt` in `/tmp/foo.txt`).
	pub name: String,
	/// The referenced node.
	pub node: Node,
}

impl PathComponent {
	pub fn resolve_absolute_path(&self) -> PathBuf {
		let path = if self.parent_dir.is_some() {
			let mut path = String::from(&self.name);
			let mut parent_dir = &self.parent_dir;

			while let Some(path_comp) = parent_dir {
				path = path_comp.name.clone() + "/" + &path;
				parent_dir = &path_comp.parent_dir;
			}

			debug_assert!(!path.starts_with('/'));
			path
		} else {
			"/".to_owned()
		};

		PathBuf::from(path)
	}
}

impl fmt::Debug for PathComponent {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("PathComponent")
			.field("name", &self.name)
			.field("node", &self.node)
			.finish()
	}
}
