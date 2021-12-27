use alloc::{boxed::Box, sync::Arc, vec::Vec};
use environment::bootinfo::VirtioMmioDevice;
use utils::static_cell::StaticCell;

use crate::{
	driver::{self, block::BlockDriver},
	schema::fs::Partition,
};

pub trait KernelOps: Sync {
	fn attach_irq(&self, irq: u8, f: Box<dyn FnMut() + Send + Sync + 'static>);

	fn register_block_driver(&self, driver: Box<dyn BlockDriver>);

	fn request_partitions(&self) -> Vec<Arc<dyn Partition>>;
}

static OPS: StaticCell<&dyn KernelOps> = StaticCell::new(&NopOps);

struct NopOps;

impl KernelOps for NopOps {
	fn attach_irq(
		&self,
		_irq: u8,
		_f: Box<dyn FnMut() + Send + Sync + 'static>,
	) {
	}

	fn register_block_driver(&self, _driver: Box<dyn BlockDriver>) {}

	fn request_partitions(&self) -> Vec<Arc<dyn Partition>> {
		vec![]
	}
}

pub(crate) fn kernel_ops() -> &'static dyn KernelOps {
	OPS.load()
}

pub fn set_kernel_ops(ops: &'static dyn KernelOps) {
	OPS.store(ops);
}

pub fn init(ops: &'static dyn KernelOps) {
	set_kernel_ops(ops);
}

pub fn init_drivers(pci_enabled: bool, mmio_devices: &[VirtioMmioDevice]) {
	driver::init(pci_enabled, mmio_devices);
}
