use core::fmt;

use itertools::join;
use utils::bytes_parser::BytesParser;

pub struct Uuid {
	first: u32,
	second: u16,
	third: u16,
	fourth: [u8; 2],
	fifth: [u8; 6],
}

impl Uuid {
	pub fn new(bytes: &[u8]) -> Self {
		assert!(bytes.len() == 16);
		let mut bytes = BytesParser::new(bytes);

		Self::parse(&mut bytes)
	}

	pub fn parse(parser: &mut BytesParser) -> Self {
		let first = parser.consume_be_u32().unwrap();
		let second = parser.consume_be_u16().unwrap();
		let third = parser.consume_be_u16().unwrap();
		let fourth = parser.consume_bytes(2).unwrap().try_into().unwrap();
		let fifth = parser.consume_bytes(6).unwrap().try_into().unwrap();

		Self {
			first,
			second,
			third,
			fourth,
			fifth,
		}
	}
}

impl fmt::Debug for Uuid {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
		f.debug_tuple("Uuid")
			.field(&format_args!(
				"{:08x}-{:04x}-{:04x}-{}-{}",
				self.first,
				self.second,
				self.third,
				join(self.fourth.iter().map(|b| format!("{:02x}", b)), ""),
				join(self.fifth.iter().map(|b| format!("{:02x}", b)), ""),
			))
			.finish()
	}
}
