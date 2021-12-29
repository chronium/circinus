#![no_std]
#![feature(box_syntax)]

#[macro_use]
extern crate alloc;

use core::ops::{Deref, DerefMut};

use alloc::sync::Arc;
use api::{
	info,
	owo_colors::OwoColorize,
	println,
	schema::fs::{self, register_partition_prober, PartitionProber},
};
use utils::bytes_parser::BytesParser;

use crate::{dirent::DirentType, filesystem::Ext2, structure::Superblock};

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
	fn probe(&self, partition: Arc<dyn fs::Partition>) {
		let superblock_sectors = partition.in_sectors(1024);
		let mut buf = vec![0u8; 1024];
		partition.read_sectors(
			superblock_sectors..superblock_sectors + superblock_sectors,
			&mut buf,
		);
		let mut parser = BytesParser::new(&buf);

		if parser.peek_le_u16_at(56).unwrap() != 0xef53 {
			info!("Partition {:?} is not ext2", partition.name());
			return;
		}

		info!("Found ext2 partition {:?}", partition.name());
		let superblock = Superblock::parse(&mut parser);
		info!("{:#?}", superblock);

		let mut ext2 = Ext2::new(partition, superblock);
		ext2.parse_bgd_table();

		let root_ino = ext2.read_inode(2);
		info!("{:#?}", root_ino);

		println!("Contents of /");
		let root_dirs = ext2.read_dirent(root_ino);
		for dir in root_dirs {
			if dir.dirent_type == DirentType::Directory {
				println!("{}", dir.name.blue())
			}
			if dir.dirent_type == DirentType::Regular {
				println!("{}", dir.name.green())
			}
		}
	}
}

pub fn init() {
	register_partition_prober(box Ext2Prober)
}

pub mod dirent;
pub mod filesystem;
pub mod inode;
pub mod structure;
