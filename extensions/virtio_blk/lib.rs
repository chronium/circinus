#![no_std]

extern crate alloc;

use core::{hint::spin_loop, mem::size_of};

use alloc::{boxed::Box, sync::Arc};
use api::{
	address::VAddr,
	driver::{
		attach_irq,
		block::{register_block_driver, BlockDriver, BlockOp},
		register_driver_prober, DeviceProber, Driver,
	},
	memoffset::offset_of,
	owo_colors::OwoColorize,
	sync::SpinLock,
	trace, warn, AsBuf,
};
use utils::byte_size::ByteSize;
use vcell::VolatileCell;
use virtio::{
	device::{Virtio, VirtqDescBuffer},
	transport::{
		virtio_pci::{VirtioAttachError, VirtioPci},
		VirtioTransport,
	},
};

const VIRTIO_BLK_F_SIZE: u64 = 1 << 6;

#[repr(u32)]
#[allow(dead_code)]
pub enum VirtioBlockCommand {
	In = 0,
	Out = 1,
	Flush = 4,
	Discard = 11,
	WriteZeroes = 13,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum VirtioBlockStatus {
	Ok = 0,
	IoError = 1,
	Unsupported = 2,
	_NotReady = 0xff,
}

#[derive(Debug, Copy, Clone)]
#[repr(C, packed)]
pub struct VirtioBlockGeometry {
	cylinders: u16,
	heads: u8,
	sectors: u8,
}

#[derive(Debug, Copy, Clone)]
#[repr(C, packed)]
pub struct VirtioBlockTopology {
	// # of logical blocks per physical block (log2)
	physical_block_exp: u8,
	// offset of first aligned logical block
	alignment_offset: u8,
	// suggested minimum I/O size in blocks
	min_io_size: u16,
	// optimal (suggested maximum) I/O size in blocks
	opt_io_size: u32,
}

#[derive(Debug, Copy, Clone)]
#[repr(C, packed)]
pub struct VirtioBlockConfig {
	capacity: u64,
	size_max: u32,
	seg_max: u32,
	geometry: VirtioBlockGeometry,
	blk_size: u32,
	topology: VirtioBlockTopology,
	writeback: u8,
	unused0: [u8; 3],
	max_discard_sectors: u32,
	max_discard_seg: u32,
	discard_sector_alignment: u32,
	max_write_zeroes_sectors: u32,
	max_write_zeroes_seg: u32,
	write_zeroes_may_unmap: u8,
	unused1: [u8; 3],
}

#[repr(C, packed)]
pub struct VirtioBlockRequest {
	_type: u32,
	_reserved: u32,
	sector: u64,
}

unsafe impl AsBuf for VirtioBlockRequest {}

pub struct VirtioBlock {
	virtio: Virtio,
	block_size: usize,
	#[allow(dead_code)]
	capacity_blocks: usize,
}

impl VirtioBlock {
	pub fn new(transport: Arc<dyn VirtioTransport>) -> Result<Self, VirtioAttachError> {
		let mut virtio = Virtio::new(transport);
		virtio.initialize(VIRTIO_BLK_F_SIZE, 1)?;

		let capacity_blocks =
			virtio.read_device_config64(offset_of!(VirtioBlockConfig, capacity) as u16) as usize;
		let block_size =
			virtio.read_device_config16(offset_of!(VirtioBlockConfig, blk_size) as u16) as usize;

		let capacity = ByteSize::new(capacity_blocks * block_size);

		virtio::info!(virtio::Kind::BlockDevice, "Capacity is", capacity);
		virtio::info!(virtio::Kind::BlockDevice, "Block size is", block_size);

		Ok(Self {
			virtio,
			block_size,
			capacity_blocks,
		})
	}

	pub fn operate(&mut self, op: BlockOp, sector: u64, buf: &mut [u8]) -> VirtioBlockStatus {
		assert!(buf.len() == self.block_size);
		trace!(
			"{}: {} sector {}",
			if op == BlockOp::Read {
				"reading"
			} else {
				"writing"
			},
			virtio::Kind::BlockDevice,
			sector.red()
		);

		let status_buffer = VolatileCell::new(VirtioBlockStatus::_NotReady);
		let status_addr = VAddr::new(status_buffer.as_ptr() as usize).as_paddr();

		let req = VirtioBlockRequest {
			_type: match op {
				BlockOp::Read => VirtioBlockCommand::In,
				BlockOp::Write => VirtioBlockCommand::Out,
			} as u32,
			sector,
			_reserved: 0,
		};
		let req_addr = VAddr::new(req.as_buf().as_ptr() as usize).as_paddr();
		let req_len = size_of::<VirtioBlockRequest>();

		let chain = &[
			VirtqDescBuffer::ReadOnlyFromDevice {
				addr: req_addr,
				len: req_len,
			},
			match op {
				BlockOp::Read => VirtqDescBuffer::WritableFromDevice {
					addr: VAddr::new(buf.as_ptr() as usize).as_paddr(),
					len: self.block_size,
				},
				BlockOp::Write => VirtqDescBuffer::ReadOnlyFromDevice {
					addr: VAddr::new(buf.as_ptr() as usize).as_paddr(),
					len: self.block_size,
				},
			},
			VirtqDescBuffer::WritableFromDevice {
				addr: status_addr,
				len: 1,
			},
		];

		let tx_virtq = self.virtio.virtq_mut(0);
		tx_virtq.enqueue(chain);
		tx_virtq.notify();

		loop {
			let status = status_buffer.get();
			if status != VirtioBlockStatus::_NotReady {
				break status;
			}
			spin_loop();
		}
	}

	pub fn handle_irq(&self) {}
}

struct VirtioBlockDriver {
	device: Arc<SpinLock<VirtioBlock>>,
}

impl VirtioBlockDriver {
	fn new(device: Arc<SpinLock<VirtioBlock>>) -> Self {
		Self { device }
	}
}

impl Driver for VirtioBlockDriver {
	fn name(&self) -> &str {
		"virtio-blk"
	}
}

impl BlockDriver for VirtioBlockDriver {
	fn read_sector(&self, sector: usize, buf: &mut [u8]) {
		self.device
			.lock()
			.operate(BlockOp::Read, sector as u64, buf);
	}

	fn read_sectors(&self, sectors: core::ops::Range<usize>, buf: &mut [u8]) {
		let mut device = self.device.lock();
		let block_size = device.block_size;
		assert!(buf.len() >= sectors.clone().count() * block_size);

		for (i, sector) in sectors.enumerate() {
			device.operate(
				BlockOp::Read,
				sector as u64,
				&mut buf[i * block_size..(i + 1) * block_size],
			);
		}
	}

	fn sector_size(&self) -> usize {
		self.device.lock().block_size
	}
}

pub struct VirtioBlockProber;

impl DeviceProber for VirtioBlockProber {
	fn probe_pci(&self, pci_device: &api::driver::pci::PciDevice) {
		if !pci_device.is_virtio_kind(virtio::Kind::BlockDevice) {
			return;
		}

		virtio::info!(virtio::Kind::BlockDevice, "Found device", "(over PCI)");
		let device = match VirtioPci::probe_pci(pci_device, VirtioBlock::new) {
			Ok(device) => Arc::new(SpinLock::new(device)),
			Err(VirtioAttachError::InvalidVendorId) => {
				return;
			}
			Err(err) => {
				warn!("failed to attach a virtio-blk: {:?}", err);
				return;
			}
		};

		register_block_driver(Box::new(VirtioBlockDriver::new(device.clone())));
		attach_irq(pci_device.config().interrupt_line(), move || {
			device.lock().handle_irq();
		});
	}

	fn probe_virtio_mmio(&self, mmio_device: &api::driver::VirtioMmioDevice) {
		let mmio = mmio_device.mmio_base.as_vaddr();
		let magic = unsafe { *mmio.as_ptr::<u32>() };
		let virtio_version = unsafe { *mmio.add(4).as_ptr::<u32>() };
		let device_id = unsafe { *mmio.add(8).as_ptr::<u32>() };

		if magic != 0x74726976 {
			return;
		}

		if virtio_version != 2 {
			warn!("unsupported virtio device version: {}", virtio_version);
			return;
		}

		// Device is a block device
		if device_id != 2 {
			return;
		}

		trace!("virtio-block: found the device (over MMIO)");
	}
}

pub fn init() {
	register_driver_prober(Box::new(VirtioBlockProber))
}
