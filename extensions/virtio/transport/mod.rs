use api::address::PAddr;

use crate::device::IsrStatus;

pub trait VirtioTransport: Send + Sync {
	fn read_device_config8(&self, offset: u16) -> u8;
	fn read_device_config16(&self, offset: u16) -> u16;
	fn read_device_config32(&self, offset: u16) -> u32;
	fn read_device_config64(&self, offset: u16) -> u64;
	fn read_isr_status(&self) -> IsrStatus;
	fn read_device_status(&self) -> u8;
	fn write_device_status(&self, value: u8);
	fn read_device_features(&self) -> u64;
	fn write_driver_features(&self, value: u64);
	fn select_queue(&self, index: u16);
	fn queue_max_size(&self) -> u16;
	fn set_queue_size(&self, queue_size: u16);
	fn notify_queue(&self, index: u16);
	fn enable_queue(&self);
	fn set_queue_desc_paddr(&self, paddr: PAddr);
	fn set_queue_driver_paddr(&self, paddr: PAddr);
	fn set_queue_device_paddr(&self, paddr: PAddr);
}

pub mod virtio_pci;
