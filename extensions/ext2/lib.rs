#![no_std]
#![feature(box_syntax)]

#[macro_use]
extern crate alloc;

use alloc::sync::Arc;
use api::{
	info,
	schema::fs::{self, register_partition_prober, PartitionProber},
};
use utils::bytes_parser::BytesParser;

use crate::structure::Superblock;

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
	}
}

pub fn init() {
	register_partition_prober(box Ext2Prober)
}

pub mod structure;
