use alloc::string::String;
use api::{posix, uuid::Uuid};
use bitflags::bitflags;
use utils::{alignment::align_up, bytes_parser::BytesParser};

#[derive(Debug)]
#[allow(unused)]
pub struct Superblock {
	total_inodes: u32,
	total_blocks: u32,
	superuser_reserved: u32,
	total_unallocated_blocks: u32,
	total_unallocated_inodes: u32,
	this_superblock: u32,
	pub block_size: u32,
	fragment_size: u32,
	blocks_per_group: u32,
	fragments_per_group: u32,
	pub inodes_per_group: u32,
	last_mount_time: posix::Timestamp,
	last_written_time: posix::Timestamp,
	times_mounted_since_fsck: u16,
	mounts_allowed_before_fsck: u16,
	signature: u16,
	state: FileSystemState,
	error_handling: ErrorHandling,
	minor_version: u16,
	last_fsck: posix::Timestamp,
	fsck_interval: posix::Timestamp,
	os_id: OsId,
	major_version: u32,
	uid_reserved_access: u16,
	gid_reserved_access: u16,

	pub extended: Option<ExtendedSuperblock>,
}

#[derive(Debug)]
#[repr(u16)]
pub enum FileSystemState {
	Clean = 1,
	HasErrors = 2,
}

impl From<u16> for FileSystemState {
	fn from(val: u16) -> Self {
		match val {
			1 => Self::Clean,
			2 => Self::HasErrors,
			_ => panic!("out of range"),
		}
	}
}

#[derive(Debug)]
#[repr(u16)]
pub enum ErrorHandling {
	Ignore = 1,
	RemountReadOnly = 2,
	KernelPanic = 3,
}

impl From<u16> for ErrorHandling {
	fn from(val: u16) -> Self {
		match val {
			1 => Self::Ignore,
			2 => Self::RemountReadOnly,
			3 => Self::KernelPanic,
			_ => panic!("out of range"),
		}
	}
}

#[derive(Debug)]
#[repr(u32)]
pub enum OsId {
	Linux = 0,
	GnuHurd = 1,
	MASIX = 2,
	FreeBSD = 3,
	Other = 4,
}

impl From<u32> for OsId {
	fn from(val: u32) -> Self {
		match val {
			0 => Self::Linux,
			1 => Self::GnuHurd,
			2 => Self::MASIX,
			3 => Self::FreeBSD,
			4 => Self::Other,
			_ => panic!("out of range"),
		}
	}
}

#[derive(Debug)]
#[allow(unused)]
pub struct ExtendedSuperblock {
	first_non_reserved: u32,
	pub inode_size_in_bytes: u16,
	this_superblock: u16,
	optional_features: OptionalFeatures,
	pub required_features: RequiredFeatures,
	readonly_features: ReadOnlyFeatures,
	fsid: Uuid,
	volume_name: String,
	last_mounted_path: String,
	compression_algo: u32,
	preallocate_files_blocks: u8,
	preallocate_dirs_blocks: u8,
	_reserved: u16,
	journal_id: Uuid,
	journal_inode: u32,
	journal_device: u32,
	head_orphan: u32,
}

bitflags! {
  struct OptionalFeatures: u32 {
	const PREALLOCATE_BLOCKS = 0x01;
	const AFS_INODE = 0x02;
	const HAS_JOURNAK = 0x04;
	const EXTENDED_ATTR = 0x08;
	const CAN_RESIZE = 0x10;
	const DIRS_USE_HASH_IDX = 0x20;
  }
}

bitflags! {
  pub struct RequiredFeatures: u32 {
	const COMPRESSION = 0x01;
	const DIRENT_TYPE = 0x02;
	const JOURNAL_REPLAY = 0x04;
	const JOURNAL_DEVICE = 0x08;
  }
}

bitflags! {
  struct ReadOnlyFeatures: u32 {
	const SPARESE_SUPERBLOCK = 0x01;
	const LARGE_FILE_SIZE = 0x02;
  const DIRENT_BTREE = 0x04;
  }
}

#[derive(Debug)]
#[allow(unused)]
pub struct BlockGroupDescriptor {
	block_usage_bitmap: u32,
	inode_usage_bitmap: u32,
	pub inode_table: u32,
	unallocated_blocks: u16,
	unallocated_inodes: u16,
	directories: u16,
	_reserved: [u8; 14],
}

impl BlockGroupDescriptor {
	pub fn parse(parser: &mut BytesParser) -> Self {
		let block_usage_bitmap = parser.consume_le_u32().unwrap();
		let inode_usage_bitmap = parser.consume_le_u32().unwrap();
		let inode_table = parser.consume_le_u32().unwrap();
		let unallocated_blocks = parser.consume_le_u16().unwrap();
		let unallocated_inodes = parser.consume_le_u16().unwrap();
		let directories = parser.consume_le_u16().unwrap();
		let _reserved = parser.consume_bytes(14).unwrap();

		Self {
			block_usage_bitmap,
			inode_usage_bitmap,
			inode_table,
			unallocated_blocks,
			unallocated_inodes,
			directories,
			_reserved: _reserved.try_into().unwrap(),
		}
	}
}

impl Superblock {
	pub fn parse(parser: &mut BytesParser) -> Self {
		let total_inodes = parser.consume_le_u32().unwrap();
		let total_blocks = parser.consume_le_u32().unwrap();
		let superuser_reserved = parser.consume_le_u32().unwrap();
		let total_unallocated_blocks = parser.consume_le_u32().unwrap();
		let total_unallocated_inodes = parser.consume_le_u32().unwrap();
		let this_superblock = parser.consume_le_u32().unwrap();
		let block_size = parser.consume_le_u32().unwrap();
		let fragment_size = parser.consume_le_u32().unwrap();
		let blocks_per_group = parser.consume_le_u32().unwrap();
		let fragments_per_group = parser.consume_le_u32().unwrap();
		let inodes_per_group = parser.consume_le_u32().unwrap();
		let last_mount_time =
			posix::Timestamp(parser.consume_le_u32().unwrap());
		let last_written_time =
			posix::Timestamp(parser.consume_le_u32().unwrap());
		let times_mounted_since_fsck = parser.consume_le_u16().unwrap();
		let mounts_allowed_before_fsck = parser.consume_le_u16().unwrap();
		let signature = parser.consume_le_u16().unwrap();
		let state = parser.consume_le_u16().unwrap();
		let error_handling = parser.consume_le_u16().unwrap();
		let minor_version = parser.consume_le_u16().unwrap();
		let last_fsck = posix::Timestamp(parser.consume_le_u32().unwrap());
		let fsck_interval = posix::Timestamp(parser.consume_le_u32().unwrap());
		let os_id = parser.consume_le_u32().unwrap();
		let major_version = parser.consume_le_u32().unwrap();
		let uid_reserved_access = parser.consume_le_u16().unwrap();
		let gid_reserved_access = parser.consume_le_u16().unwrap();

		let extended = if major_version >= 1 {
			Some(ExtendedSuperblock::parse(parser))
		} else {
			None
		};

		Self {
			total_inodes,
			total_blocks,
			superuser_reserved,
			total_unallocated_blocks,
			total_unallocated_inodes,
			this_superblock,
			block_size: 1024 << block_size,
			fragment_size: 1024 << fragment_size,
			blocks_per_group,
			fragments_per_group,
			inodes_per_group,
			last_mount_time,
			last_written_time,
			times_mounted_since_fsck,
			mounts_allowed_before_fsck,
			signature,
			state: state.into(),
			error_handling: error_handling.into(),
			minor_version,
			last_fsck,
			fsck_interval,
			os_id: os_id.into(),
			major_version,
			uid_reserved_access,
			gid_reserved_access,
			extended,
		}
	}

	pub fn bgd_count(&self) -> usize {
		let inode_count = self.total_inodes;
		let block_count = self.total_blocks;

		let blocks_per_group = self.blocks_per_group;
		let inodes_per_group = self.inodes_per_group;

		let blocks_bgd_count =
			align_up(block_count as usize, blocks_per_group as usize)
				/ blocks_per_group as usize;
		let inode_bgd_count =
			align_up(inode_count as usize, inodes_per_group as usize)
				/ inodes_per_group as usize;

		assert!(blocks_bgd_count == inode_bgd_count);

		blocks_bgd_count as usize
	}
}

impl ExtendedSuperblock {
	pub fn parse(parser: &mut BytesParser) -> Self {
		let first_non_reserved = parser.consume_le_u32().unwrap();
		let inode_size_in_bytes = parser.consume_le_u16().unwrap();
		let this_superblock = parser.consume_le_u16().unwrap();
		let optional_features = parser.consume_le_u32().unwrap();
		let required_features = parser.consume_le_u32().unwrap();
		let readonly_features = parser.consume_le_u32().unwrap();
		let fsid = Uuid::parse(parser);
		let volume_name = parser.consume_cstr(16).unwrap();
		let last_mounted_path = parser.consume_cstr(64).unwrap();
		let compression_algo = parser.consume_le_u32().unwrap();
		let preallocate_files_blocks = parser.consume_u8().unwrap();
		let preallocate_dirs_blocks = parser.consume_u8().unwrap();
		let _reserved = parser.consume_le_u16().unwrap();
		let journal_id = Uuid::parse(parser);
		let journal_inode = parser.consume_le_u32().unwrap();
		let journal_device = parser.consume_le_u32().unwrap();
		let head_orphan = parser.consume_le_u32().unwrap();

		Self {
			first_non_reserved,
			inode_size_in_bytes,
			this_superblock,
			optional_features: OptionalFeatures::from_bits(optional_features)
				.unwrap(),
			required_features: RequiredFeatures::from_bits(required_features)
				.unwrap(),
			readonly_features: ReadOnlyFeatures::from_bits(readonly_features)
				.unwrap(),
			fsid,
			volume_name,
			last_mounted_path,
			compression_algo,
			preallocate_files_blocks,
			preallocate_dirs_blocks,
			_reserved,
			journal_id,
			journal_inode,
			journal_device,
			head_orphan,
		}
	}
}
