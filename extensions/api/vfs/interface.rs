use core::fmt;

use alloc::{string::String, sync::Arc};

use super::Node;

#[derive(Clone)]
pub struct PathComponent {
	/// The parent directory. `None` if this is the root directory.
	pub parent_dir: Option<Arc<PathComponent>>,
	/// THe component name (e.g. `tmp` or `foo.txt` in `/tmp/foo.txt`).
	pub name: String,
	/// The referenced node.
	pub node: Node,
}

impl fmt::Debug for PathComponent {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("PathComponent")
			.field("name", &self.name)
			.field("node", &self.node)
			.finish()
	}
}
