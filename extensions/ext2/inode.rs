use core::mem;

use api::posix;
use bitflags::bitflags;
use utils::bytes_parser::BytesParser;

use crate::BlockPointer;

#[derive(Debug)]
#[repr(transparent)]
pub struct BlockPointers([BlockPointer; 12]);

impl BlockPointers {
	pub fn parse(parser: &mut BytesParser) -> Self {
		assert!(parser.remaining_len() >= 12 * mem::size_of::<u32>());

		let mut pointers = [BlockPointer::default(); 12];
		for pointer in pointers.iter_mut() {
			**pointer = parser.consume_le_u32().unwrap();
		}

		Self(pointers)
	}

	pub fn at(&self, at: usize) -> BlockPointer {
		self.0[at]
	}

	pub fn count(&self) -> usize {
		self.0.iter().filter(|&&b| *b != 0).count()
	}
}

bitflags! {
  struct TypeAndPermissions: u16 {
	// --- Type ---
	const FIFO = 0x1000;
  const CHARR_DEVICE = 0x2000;
  const DIRECTORY = 0x4000;
  const BLOCK_DEVICE = 0x6000;
  const REGULAR_FILE = 0x8000;
  const SYM_LINK = 0xa000;
  const UNIX_SOCKET = 0xc000;

  // --- Permissions ---

  // Other
  const OX = 0x001;
  const OW = 0x002;
  const OR = 0x004;

  // Group
  const GX = 0x008;
  const GW = 0x010;
  const GR = 0x020;

  // User
	const UX = 0x040;
	const UW = 0x080;
	const UR = 0x100;

  // --- Attributes ---

  const STICKY = 0x200;
  const SET_GID = 0x400;
  const SET_UID = 0x800;
  }
}

bitflags! {
  struct Flags: u32 {
	const SECURE_DELETION = 0x0000_0001;
	const COPY_ON_DELETE = 0x0000_0002;
  const FILE_COMPRESSION = 0x0000_0004;
  const SYNC_UPDATES = 0x0000_0008;
  const IMMUTABLE_FILE = 0x0000_0010;
  const APPEND_ONLY = 0x0000_0020;
  const NO_DUMP = 0x0000_0040;
  const NO_LAST_ACCESS_UPDATE = 0x0000_0080;

  const HASH_INDEX_DIRECTORY = 0x0001_0000;
  const AFS_DIRECTORY = 0x0002_0000;
  const JOURNAL_FILE_DATA = 0x0004_0000;
  }
}

#[derive(Debug)]
#[allow(unused)]
pub struct Inode {
	type_and_perms: TypeAndPermissions,
	uid: u16,
	pub lower_size: u32,
	last_access: posix::Timestamp,
	creation: posix::Timestamp,
	last_modification: posix::Timestamp,
	deletion: posix::Timestamp,
	gid: u16,
	hard_links: u16,
	drive_sectors: u32,
	flags: Flags,
	_osval1: [u8; 4],
	pub direct_pointers: BlockPointers,
	singly_pointer: BlockPointer,
	doubly_pointer: BlockPointer,
	triply_pointer: BlockPointer,
	gen_number: u32,
	extended_attrib_block: BlockPointer,
	pub extended_dir_block: BlockPointer,
	fragment_pointer: BlockPointer,
	_osval2: [u8; 12],
}

impl Inode {
	pub fn parse(parser: &mut BytesParser) -> Self {
		let type_and_perms = parser.consume_le_u16().unwrap();
		let uid = parser.consume_le_u16().unwrap();
		let lower_size = parser.consume_le_u32().unwrap();
		let last_access = posix::Timestamp(parser.consume_le_u32().unwrap());
		let creation = posix::Timestamp(parser.consume_le_u32().unwrap());
		let last_modification = posix::Timestamp(parser.consume_le_u32().unwrap());
		let deletion = posix::Timestamp(parser.consume_le_u32().unwrap());
		let gid = parser.consume_le_u16().unwrap();
		let hard_links = parser.consume_le_u16().unwrap();
		let drive_sectors = parser.consume_le_u32().unwrap();
		let flags = parser.consume_le_u32().unwrap();
		let _osval1 = parser.consume_bytes(4).unwrap().try_into().unwrap();
		let direct_pointers = BlockPointers::parse(parser);
		let singly_pointer = BlockPointer(parser.consume_le_u32().unwrap());
		let doubly_pointer = BlockPointer(parser.consume_le_u32().unwrap());
		let triply_pointer = BlockPointer(parser.consume_le_u32().unwrap());
		let gen_number = parser.consume_le_u32().unwrap();
		let extended_attrib_block = BlockPointer(parser.consume_le_u32().unwrap());
		let extended_dir_block = BlockPointer(parser.consume_le_u32().unwrap());
		let fragment_pointer = BlockPointer(parser.consume_le_u32().unwrap());
		let _osval2 = parser.consume_bytes(12).unwrap().try_into().unwrap();

		Self {
			type_and_perms: TypeAndPermissions::from_bits(type_and_perms).unwrap(),
			uid,
			lower_size,
			last_access,
			creation,
			last_modification,
			deletion,
			gid,
			hard_links,
			drive_sectors,
			flags: Flags::from_bits(flags).unwrap(),
			_osval1,
			direct_pointers,
			singly_pointer,
			doubly_pointer,
			triply_pointer,
			gen_number,
			extended_attrib_block,
			extended_dir_block,
			fragment_pointer,
			_osval2,
		}
	}
}
