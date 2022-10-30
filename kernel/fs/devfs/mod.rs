use alloc::sync::Arc;
use api::{
	vfs::{self, File},
	Result,
};
use tempfs::Tempfs;
use utils::once::Once;

use self::{devconsole::DevConsole, fb0::Framebuffer};

pub static DEVFS: Once<Arc<Devfs>> = Once::new();
pub static SERIAL_TTY: Once<Arc<DevConsole>> = Once::new();
pub static FRAMEBUFFER_FILE: Once<Arc<Framebuffer>> = Once::new();

pub struct Devfs(Tempfs);

impl Devfs {
	pub fn new() -> Self {
		let tempfs = Tempfs::new();
		let root_dir = tempfs.root();

		SERIAL_TTY.init(|| Arc::new(DevConsole::new(Tempfs::alloc_inode_no())));
		FRAMEBUFFER_FILE.init(|| Arc::new(Framebuffer::new(Tempfs::alloc_inode_no())));

		root_dir.add_file("devcon", SERIAL_TTY.clone() as Arc<dyn File>);
		root_dir.add_file("Framebuffer", FRAMEBUFFER_FILE.clone() as Arc<dyn File>);

		Self(tempfs)
	}
}

impl vfs::Filesystem for Devfs {
	fn root(&self) -> Result<Arc<dyn vfs::Directory>> {
		vfs::Filesystem::root(&self.0)
	}
}

pub fn init() {
	DEVFS.init(|| Arc::new(Devfs::new()));
}

pub mod devconsole;
pub mod fb0;
