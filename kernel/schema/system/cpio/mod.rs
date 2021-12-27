use core::{fmt, str::from_utf8_unchecked};

use alloc::sync::Arc;
use api::schema::{posix, unix};
use hashbrown::HashMap;

fn parse_str_field(bytes: &[u8]) -> &str {
	unsafe { from_utf8_unchecked(bytes) }
}

fn parse_hex_field(bytes: &[u8]) -> usize {
	usize::from_str_radix(parse_str_field(bytes), 16).unwrap()
}

macro_rules! consume_hex {
	($archive:ident, $cnt:expr) => {{
		use $crate::schema::system::cpio;
		cpio::parse_hex_field($archive.consume_bytes($cnt).unwrap())
	}};
}

macro_rules! consume_str {
	($archive:ident, $cnt:expr) => {{
		use $crate::schema::system::cpio;
		cpio::parse_str_field($archive.consume_bytes($cnt).unwrap())
	}};
}

#[allow(unused)]
pub struct CpioFile<'a> {
	filename: &'a str,
	data: &'a [u8],
	stat: posix::Stat,
}

impl fmt::Debug for CpioFile<'_> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("InitramFsFile")
			.field("name", &self.filename)
			.finish()
	}
}

pub enum CpioNode<'a> {
	File(Arc<CpioFile<'a>>),
	Directory(Arc<CpioDir<'a>>),
	Symlink(Arc<CpioSymlink<'a>>),
}

#[allow(unused)]
pub struct CpioDir<'a> {
	filename: &'a str,
	stat: posix::Stat,
	files: HashMap<&'a str, CpioNode<'a>>,
}

#[allow(unused)]
pub struct CpioSymlink<'a> {
	filename: &'a str,
	stat: posix::Stat,
	dst: unix::PathBuf,
}

#[allow(unused)]
pub struct CpioArchive<'a> {
	root_dir: Arc<CpioDir<'a>>,
}

pub mod archive;
