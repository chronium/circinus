#![no_std]
#![feature(box_syntax)]

#[macro_use]
extern crate alloc;

use core::ops::{Deref, DerefMut};

use alloc::{boxed::Box, sync::Arc};
use api::{
	info,
	io::OpenOptions,
	owo_colors::OwoColorize,
	println,
	schema::fs::{self, register_partition_prober, PartitionProber, VFS},
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
		let arc = Arc::new(SpinLock::new(box ext2 as Box<dyn Filesystem>));

		let mp = VFS.lock().mount_root(arc.clone());
		let ext2 = VFS.lock().filesystem(&mp);

		use fs::filesystem::Filesystem;
		let root = ext2.lock().root();

		println!("Contents of /");
		for dir in root.iter() {
			if dir.ftype == fs::FileType::Directory {
				println!("{}", dir.path.as_str().blue())
			}
			if dir.ftype == fs::FileType::RegularFile {
				println!("{}", dir.path.as_str().green())
			}
		}

		if let Ok(boot) = root.lookup("boot") {
			println!("Contents of /boot");

			for dir in boot.as_dir().expect("").iter() {
				if dir.ftype == fs::FileType::Directory {
					println!("{}", dir.path.as_str().blue())
				}
				if dir.ftype == fs::FileType::RegularFile {
					println!("{}", dir.path.as_str().green())
				}
			}
		};

		if let Ok(test) = root.lookup("test.txt") {
			println!("Contents of /test.txt");

			match test.as_file().expect("").open(&OpenOptions::Read) {
				Err(e) => info!("Could not open {:?}", e),
				Ok(file) => {}
			}
		}
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
