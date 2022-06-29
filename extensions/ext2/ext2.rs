use core::sync::atomic::AtomicUsize;

use alloc::{sync::Arc, vec::Vec};
use api::{info, owo_colors::OwoColorize, schema::fs, sync::SpinLock};
use utils::{alignment::align_up, bytes_parser::BytesParser, once::Once};

use crate::{
	dirent::Dirent,
	inode::Inode,
	structure::{
		BlockGroupDescriptor, ReadOnlyFeatures, RequiredFeatures, Superblock,
	},
	BlockPointer,
};

pub struct Ext2 {
	physical_partition: Arc<SpinLock<dyn fs::Partition>>,
	superblock: Arc<Superblock>,
	block_group_descriptors: Arc<Once<Vec<BlockGroupDescriptor>>>,
	block_size: usize,
	inode_size: usize,
	dirent_has_type: bool,
	pub large_file_size: bool,
	next_fid: Arc<AtomicUsize>,
}

impl Clone for Ext2 {
	fn clone(&self) -> Self {
		Self {
			physical_partition: self.physical_partition.clone(),
			superblock: self.superblock.clone(),
			block_group_descriptors: self.block_group_descriptors.clone(),
			block_size: self.block_size.clone(),
			inode_size: self.inode_size.clone(),
			dirent_has_type: self.dirent_has_type.clone(),
			large_file_size: self.large_file_size.clone(),
			next_fid: self.next_fid.clone(),
		}
	}
}

impl Ext2 {
	pub fn new(
		physical_partition: Arc<SpinLock<dyn fs::Partition>>,
		superblock: Superblock,
	) -> Self {
		let block_size = superblock.block_size as usize;
		let inode_size = superblock
			.extended
			.as_ref()
			.map(|ext| ext.inode_size_in_bytes)
			.unwrap_or(128) as usize;

		let dirent_has_type = superblock
			.extended
			.as_ref()
			.map(|ext| {
				ext.required_features
					.contains(RequiredFeatures::DIRENT_TYPE)
			})
			.unwrap_or(false);

		let large_file_size = superblock
			.extended
			.as_ref()
			.map(|ext| {
				ext.readonly_features
					.contains(ReadOnlyFeatures::LARGE_FILE_SIZE)
			})
			.unwrap_or(false);

		Self {
			physical_partition,
			superblock: Arc::new(superblock),
			block_group_descriptors: Arc::new(Once::new()),
			block_size,
			inode_size,
			dirent_has_type,
			large_file_size,
			next_fid: Arc::new(AtomicUsize::new(0)),
		}
	}

	pub fn bgd_block(&self) -> BlockPointer {
		BlockPointer(
			if self.superblock.block_size == 1024 {
				2
			} else {
				1
			},
		)
	}

	pub fn inode_bgd(&self, inode: usize) -> BlockGroupDescriptor {
		self.block_group_descriptors
			[(inode - 1) / self.superblock.inodes_per_group as usize]
			.clone()
	}

	pub fn read_dirents(&self, block: BlockPointer, res: &mut Vec<Dirent>) {
		let mut buf = self.read_block_alloc(block);
		let mut parser = BytesParser::new(&mut buf);

		loop {
			if let Some(dirent) =
				Dirent::parse(&mut parser, self.dirent_has_type)
			{
				res.push(dirent);
			}

			if parser.remaining_len() < 8 {
				break;
			}
		}
	}

	pub fn parse_bgd_table(&mut self) {
		if self.block_group_descriptors.is_init() {
			panic!(
				"{} {}",
				"Block Group Descriptor table already read for partition".red(),
				self.physical_partition.lock().name().blue()
			)
		}

		let bgd_count = self.superblock.bgd_count();
		let blocks_count =
			align_up(bgd_count, self.block_size) / self.block_size;

		let mut buf = vec![0u8; blocks_count * self.block_size];
		self.read_block(self.bgd_block(), &mut buf);
		let mut parser = BytesParser::new(&buf);

		info!("Parsing {} Block Group Descriptors", bgd_count);
		let mut res = vec![];
		for _ in 0..bgd_count {
			res.push(BlockGroupDescriptor::parse(&mut parser))
		}
		self.block_group_descriptors.init(|| res);
	}

	pub fn read_block(&self, block: BlockPointer, buf: &mut [u8]) {
		assert!(buf.len() >= self.block_size);

		let partition = self.physical_partition.lock();

		let sectors_per_block =
			align_up(self.block_size, partition.block_size())
				/ partition.block_size();

		let start_sector = *block as usize * sectors_per_block;
		let end_sector = start_sector + sectors_per_block;

		partition.read_sectors(start_sector..end_sector, buf)
	}

	pub fn read_block_alloc(&self, block: BlockPointer) -> Vec<u8> {
		let mut buf = vec![0u8; self.block_size];

		self.read_block(block, &mut buf);

		buf
	}

	pub fn read_inode(&self, inode: usize) -> Inode {
		let bgd = self.inode_bgd(inode);

		let index = (inode - 1) % self.superblock.inodes_per_group as usize;
		let containing_block = (index * self.inode_size) / self.block_size;

		let buf = self.read_block_alloc(BlockPointer(
			bgd.inode_table + containing_block as u32,
		));

		Inode::parse(&mut BytesParser::new(&buf[index * self.inode_size..]))
	}

	pub fn read_dirent(&self, inode: Inode) -> Vec<Dirent> {
		let mut res = vec![];

		assert!(inode.direct_pointers.count() == 1, "not yet implemented");
		self.read_dirents(inode.direct_pointers.at(0), &mut res);

		res
	}
}
