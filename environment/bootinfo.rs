use arrayvec::{ArrayString, ArrayVec};

use crate::address::PAddr;

pub struct RamArea {
	pub base: PAddr,
	pub len: usize,
}

pub struct VirtioMmioDevice {
	pub mmio_base: PAddr,
	pub irq: u8,
}

#[derive(Debug)]
pub struct Framebuffer {
	pub addr: PAddr,
	pub pitch: u32,
	pub width: u32,
	pub height: u32,
	pub bpp: u8,
}

pub struct BootInfo {
	pub ram_areas: ArrayVec<RamArea, 8>,
	pub virtio_mmio_devices: ArrayVec<VirtioMmioDevice, 4>,
	pub log_filter: ArrayString<64>,
	pub framebuffer: Framebuffer,
	pub pci_enabled: bool,
	pub use_second_serialport: bool,
}
