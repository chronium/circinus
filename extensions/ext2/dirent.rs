use alloc::string::String;
use utils::bytes_parser::BytesParser;

#[derive(Debug)]
#[allow(unused)]
pub struct Dirent {
	pub inode: u32,
	total_size: u16,
	name_length_lsb: u8,
	pub dirent_type: DirentType,
	pub name: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum DirentType {
	Unknown = 0,
	Regular = 1,
	Directory = 2,
	CharDevice = 3,
	BlockDevice = 4,
	Fifo = 5,
	Socket = 6,
	Symlink = 7,
}

impl From<u8> for DirentType {
	fn from(val: u8) -> Self {
		match val {
			0 => Self::Unknown,
			1 => Self::Regular,
			2 => Self::Directory,
			3 => Self::CharDevice,
			4 => Self::BlockDevice,
			5 => Self::Fifo,
			6 => Self::Socket,
			7 => Self::Symlink,
			_ => Self::Unknown,
		}
	}
}

impl Dirent {
	pub fn parse(parser: &mut BytesParser, has_type: bool) -> Option<Self> {
		assert!(has_type, "directories without type unsupported");

		let inode = parser.consume_le_u32().unwrap();
		let total_size = parser.consume_le_u16().unwrap();
		let name_len = total_size - 8;

		if inode == 0 {
			let _ = parser.skip(total_size as usize - 6);
			return None;
		}

		let name_length_lsb = parser.consume_u8().unwrap();
		let dirent_type = parser.consume_u8().unwrap().into();

		let name = parser.consume_cstr(name_len as usize).unwrap();

		Some(Self {
			inode,
			total_size,
			name_length_lsb,
			dirent_type,
			name,
		})
	}
}
