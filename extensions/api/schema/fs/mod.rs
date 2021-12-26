pub mod dirent;
pub mod readdir;

use alloc::{boxed::Box, sync::Arc, vec::Vec};
use environment::spinlock::SpinLock;
pub use readdir::ReadDir;

use crate::kernel::kernel_ops;

static PARTITION_PROBERS: SpinLock<Vec<Box<dyn PartitionProber>>> =
	SpinLock::new(Vec::new());

pub trait Partition: Send + Sync {}

pub trait PartitionProber: Send + Sync {
	fn probe(&self, partition: Arc<dyn Partition>);
}

pub fn register_partition_prober(prober: Box<dyn PartitionProber>) {
	PARTITION_PROBERS.lock().push(prober);
}

pub fn init() {
	for partition in kernel_ops().request_partitions() {
		for prober in PARTITION_PROBERS.lock().iter() {
			prober.probe(partition.clone());
		}
	}
}
