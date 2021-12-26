#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct MacAddress([u8; 6]);

impl MacAddress {
	pub fn new(addr: [u8; 6]) -> MacAddress {
		MacAddress(addr)
	}

	pub fn as_array(&self) -> [u8; 6] {
		self.0
	}
}
