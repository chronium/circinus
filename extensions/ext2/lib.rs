#![no_std]
#![feature(box_syntax)]

extern crate alloc;

use alloc::sync::Arc;
use api::schema::fs::{self, register_partition_prober, PartitionProber};

pub struct Ext2Prober;

impl PartitionProber for Ext2Prober {
	fn probe(&self, partition: Arc<dyn fs::Partition>) {
		// TODO
	}
}

pub fn init() {
	register_partition_prober(box Ext2Prober)
}
