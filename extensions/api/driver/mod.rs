use alloc::{boxed::Box, fmt, vec::Vec};
use environment::spinlock::SpinLock;
use log::trace;

pub use environment::bootinfo::VirtioMmioDevice;

use crate::kernel::kernel_ops;

use self::pci::PciDevice;

static DEVICE_PROBERS: SpinLock<Vec<Box<dyn DeviceProber>>> =
	SpinLock::new(vec![]);

pub trait Driver: Send + Sync {
	fn name(&self) -> &str;
}

pub trait DeviceProber: Send + Sync {
	fn probe_pci(&self, pci_device: &PciDevice);
	fn probe_virtio_mmio(&self, mmio_device: &VirtioMmioDevice);
}

pub fn register_driver_prober(driver: Box<dyn DeviceProber>) {
	DEVICE_PROBERS.lock().push(driver);
}

pub fn attach_irq<F: FnMut() + Send + Sync + 'static>(irq: u8, f: F) {
	kernel_ops().attach_irq(irq, box f)
}

pub fn init(pci_enabled: bool, mmio_devices: &[VirtioMmioDevice]) {
	// Scan PCI devices.
	if pci_enabled {
		for device in pci::enumerate_pci_devices() {
			trace!(
				"pci: found a device: id={:04x}:{:04x}, bar0={:016x?}, irq={}",
				device.config().vendor_id(),
				device.config().device_id(),
				device.config().bar(0),
				device.config().interrupt_line()
			);

			for prober in DEVICE_PROBERS.lock().iter() {
				prober.probe_pci(&device);
			}
		}
	}

	// Register Virtio devices connected over MMIO.
	for device in mmio_devices {
		for prober in DEVICE_PROBERS.lock().iter() {
			prober.probe_virtio_mmio(device);
		}
	}
}

#[derive(Debug)]
#[repr(u16)]
pub enum VirtioKind {
	NetworkCard = 0x1041,
	BlockDevice = 0x1042,
	Console = 0x1043,
	EntropySource = 0x1044,
	MemoryBallooning = 0x1045,
	SCSIHost = 0x1048,
	Filesystem = 0x1049,
	GPU = 0x1050,
	Input = 0x1052,
	Socket = 0x1053,
}

impl fmt::Display for VirtioKind {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			VirtioKind::NetworkCard => write!(f, "virtio-net"),
			VirtioKind::BlockDevice => write!(f, "virtio-blk"),
			_ => unimplemented!("{:?}", self),
		}
	}
}

pub mod block;
pub mod net;
pub mod pci;
