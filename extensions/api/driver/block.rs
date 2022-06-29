use core::ops::Range;

use alloc::boxed::Box;

use crate::kernel::kernel_ops;

use super::Driver;

#[derive(PartialEq, Eq)]
pub enum BlockOp {
	Read,
	Write,
}

pub trait BlockDriver: Driver {
	fn read_sector(&self, sector: usize, buf: &mut [u8]);
	fn read_sectors(&self, sectors: Range<usize>, buf: &mut [u8]);

	fn sector_size(&self) -> usize;
}

pub fn register_block_driver(driver: Box<dyn BlockDriver>) {
	kernel_ops().register_block_driver(driver)
}
