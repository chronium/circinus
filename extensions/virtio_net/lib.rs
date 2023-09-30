#![no_std]

extern crate alloc;

use alloc::{boxed::Box, format, sync::Arc};
use api::{
	driver::{net::MacAddress, register_driver_prober, DeviceProber, VirtioMmioDevice},
	memoffset::offset_of,
	sync::SpinLock,
	trace, warn,
};
use virtio::{
	device::Virtio,
	transport::{
		virtio_pci::{VirtioAttachError, VirtioPci},
		VirtioTransport,
	},
};

const VIRTIO_NET_F_MAC: u64 = 1 << 5;

#[derive(Debug, Copy, Clone)]
#[repr(C, packed)]
struct VirtioNetConfig {
	mac: [u8; 6],
	status: u16,
	max_virtqueue_pairs: u16,
	mtu: u16,
}

pub struct VirtioNet {
	#[allow(dead_code)]
	mac_addr: MacAddress,
	#[allow(dead_code)]
	virtio: Virtio,
}

impl VirtioNet {
	pub fn new(transport: Arc<dyn VirtioTransport>) -> Result<Self, VirtioAttachError> {
		let mut virtio = Virtio::new(transport);
		virtio.initialize(VIRTIO_NET_F_MAC, 2 /* RX and TX queues. */)?;

		let mut mac_addr = [0; 6];
		for (i, byte) in mac_addr.iter_mut().enumerate() {
			*byte = virtio.read_device_config8((offset_of!(VirtioNetConfig, mac) + i) as u16);
		}

		virtio::info!(
			virtio::Kind::NetworkCard,
			"MAC address is",
			mac_addr.map(|m| format!("{:02x}", m)).join(":")
		);

		Ok(Self {
			mac_addr: MacAddress::new(mac_addr),
			virtio,
		})
	}
}

pub struct VirtioNetProber;

impl DeviceProber for VirtioNetProber {
	fn probe_pci(&self, pci_device: &api::driver::pci::PciDevice) {
		if !pci_device.is_virtio_kind(virtio::Kind::NetworkCard) {
			return;
		}

		virtio::info!(virtio::Kind::NetworkCard, "Found device", "(over PCI)");

		// TODO
		let _device = match VirtioPci::probe_pci(pci_device, VirtioNet::new) {
			Ok(device) => Arc::new(SpinLock::new(device)),
			Err(VirtioAttachError::InvalidVendorId) => {
				return;
			}
			Err(err) => {
				warn!("failed to attach a virtio-net: {:?}", err);
				return;
			}
		};
	}

	fn probe_virtio_mmio(&self, mmio_device: &VirtioMmioDevice) {
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

		// It looks like a virtio device. Check if the device is a network card.
		if device_id != 1 {
			return;
		}

		trace!("virtio-net: found the device (over MMIO)");
	}
}

pub fn init() {
	register_driver_prober(Box::new(VirtioNetProber));
}
