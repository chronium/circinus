#![no_std]
#![feature(box_syntax)]

#[macro_use]
extern crate alloc;

use core::ops::{Deref, DerefMut};

use alloc::sync::Arc;
use api::{
	info,
	schema::fs::{self, register_partition_prober, PartitionProber},
	sync::SpinLock,
};
use utils::bytes_parser::BytesParser;

use crate::{ext2::Ext2, structure::Superblock};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct BlockPointer(u32);

impl Deref for BlockPointer {
	type Target = u32;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for BlockPointer {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

pub enum Ext2Error {}

pub struct Ext2Prober;

impl PartitionProber for Ext2Prober {
	fn probe(&self, partition: Arc<SpinLock<dyn fs::Partition>>) {
		let part = partition.lock();
		let superblock_sectors = part.in_sectors(1024);
		let mut buf = vec![0u8; 1024];
		part.read_sectors(
			superblock_sectors..superblock_sectors + superblock_sectors,
			&mut buf,
		);
		let mut parser = BytesParser::new(&buf);

		if parser.peek_le_u16_at(56).unwrap() != 0xef53 {
			info!("Partition {:?} is not ext2", part.name());
			return;
		}

		info!("Found ext2 partition {:?}", part.name());
		let superblock = Superblock::parse(&mut parser);
		info!("{:#?}", superblock);

		drop(part);
		let mut ext2 = Ext2::new(partition, superblock);
		ext2.parse_bgd_table();
	}
}

pub fn init() {
	register_partition_prober(box Ext2Prober)
}

pub mod dirent;
pub mod ext2;
pub mod filesystem;
pub mod inode;
pub mod structure;
