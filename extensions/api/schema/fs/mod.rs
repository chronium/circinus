use core::ops::Range;

use alloc::{boxed::Box, sync::Arc, vec::Vec};
use environment::spinlock::SpinLock;

pub use directory::Directory;
pub use filesystem::{DirEntry, FileType};
use utils::once::Once;

use crate::kernel::kernel_ops;

use self::vfs::Vfs;

static PARTITION_PROBERS: SpinLock<Vec<Box<dyn PartitionProber>>> =
	SpinLock::new(vec![]);

pub static VFS: Once<SpinLock<Vfs>> = Once::new();

pub trait Partition: Send + Sync {
	fn read_sector(&self, sector: usize, buf: &mut [u8]);

	fn in_sectors(&self, size: usize) -> usize;
	fn block_size(&self) -> usize;

	fn name(&self) -> &str;

	fn read_sectors(&self, sectors: Range<usize>, buf: &mut [u8]) {
		let block_size = self.block_size();
		assert!(buf.len() >= sectors.clone().count() * block_size);

		for (i, sector) in sectors.enumerate() {
			self.read_sector(
				sector,
				&mut buf[i * block_size..(i + 1) * block_size],
			)
		}
	}
}

pub trait PartitionProber: Send + Sync {
	fn probe(&self, partition: Arc<dyn Partition>);
}

pub fn register_partition_prober(prober: Box<dyn PartitionProber>) {
	PARTITION_PROBERS.lock().push(prober);
}

pub fn init() {
	VFS.init(|| SpinLock::new(Vfs::new()));

	for partition in kernel_ops().request_partitions() {
		for prober in PARTITION_PROBERS.lock().iter() {
			prober.probe(partition.clone());
		}
	}
}

pub mod directory;
pub mod file;
pub mod filesystem;
pub mod vfs;
