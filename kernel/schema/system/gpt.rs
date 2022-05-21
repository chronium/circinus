use alloc::{fmt, string::String, sync::Arc, vec::Vec};
use api::{guid::Guid, schema::fs::Partition, sync::SpinLock};
use atomic_refcell::AtomicRefCell;
use spin::Mutex;
use utils::{alignment::align_up, bytes_parser::BytesParser, once::Once};

use crate::schema::block::with_block_driver;

pub struct GptHeader {
	signature: [u8; 8],
	revision: u32,
	header_size: u32,
	header_crc: u32,
	_reserved: u32,
	this_lba: u64,
	header_mirror: u64,
	first_usable_block: u64,
	last_usable_block: u64,
	guid: Guid,
	partition_entry: u64,
	partition_count: u32,
	partition_entry_bytes: u32,
	partition_entry_crc: u32,
}

impl GptHeader {
	pub fn parse_bytes(bytes: &[u8]) -> Result<Self, GptError> {
		assert!(bytes.len() >= 0x5c);
		let mut bytes = BytesParser::new(&bytes);

		Self::parse(&mut bytes)
	}

	pub fn parse(bytes: &mut BytesParser) -> Result<Self, GptError> {
		let signature = bytes.consume_bytes(8).unwrap().try_into().unwrap();
		if &signature != b"EFI PART" {
			return Err(GptError::HeaderSignatureNotFound);
		}

		let revision = bytes.consume_le_u32().unwrap();
		let header_size = bytes.consume_le_u32().unwrap();
		let header_crc = bytes.consume_le_u32().unwrap();
		let _reserved = bytes.consume_le_u32().unwrap();
		let this_lba = bytes.consume_le_u64().unwrap();
		let header_mirror = bytes.consume_le_u64().unwrap();
		let first_usable_block = bytes.consume_le_u64().unwrap();
		let last_usable_block = bytes.consume_le_u64().unwrap();
		let guid = Guid::new(bytes.consume_bytes(16).unwrap());
		let partition_entry = bytes.consume_le_u64().unwrap();
		let num_partitions = bytes.consume_le_u32().unwrap();
		let partition_entry_bytes = bytes.consume_le_u32().unwrap();
		let partition_entry_crc = bytes.consume_le_u32().unwrap();

		Ok(GptHeader {
			signature,
			revision,
			header_size,
			header_crc,
			_reserved,
			this_lba,
			header_mirror,
			first_usable_block,
			last_usable_block,
			guid,
			partition_entry,
			partition_count: num_partitions,
			partition_entry_bytes,
			partition_entry_crc,
		})
	}
}

#[derive(Debug)]
#[allow(unused)]
pub struct PartitionEntry {
	type_guid: Guid,
	guid: Guid,
	start_lba: u64,
	end_lba: u64,
	attributes: u64,
	name: String,
	block_size: AtomicRefCell<Option<usize>>,
}

impl PartitionEntry {
	pub fn parse(
		parser: &mut BytesParser,
		len: usize,
	) -> Result<Option<Self>, GptError> {
		assert!(parser.remaining_len() >= len);

		let type_guid_bytes = parser.peek_bytes(16).unwrap();

		if type_guid_bytes.iter().all(|b| *b == 0) {
			let _ = parser.consume_bytes(len);
			return Ok(None);
		}

		let type_guid = Guid::new(parser.consume_bytes(16).unwrap());
		let guid = Guid::new(parser.consume_bytes(16).unwrap());
		let start_lba = parser.consume_le_u64().unwrap();
		let end_lba = parser.consume_le_u64().unwrap();
		let attributes = parser.consume_le_u64().unwrap();
		let (_, name_bytes, _) = unsafe {
			parser
				.consume_bytes(len as usize - 0x38)
				.unwrap()
				.align_to::<u16>()
		};

		let name = String::from_utf16(
			name_bytes
				.iter()
				.filter_map(|b| if *b != 0 { Some(*b) } else { None })
				.collect::<alloc::vec::Vec<_>>()
				.as_slice(),
		)
		.unwrap();

		Ok(Some(PartitionEntry {
			type_guid,
			guid,
			start_lba,
			end_lba,
			attributes,
			name,
			block_size: AtomicRefCell::new(None),
		}))
	}
}

impl Partition for PartitionEntry {
	fn read_sector(&self, sector: usize, buf: &mut [u8]) {
		with_block_driver(|block| {
			assert!(buf.len() >= block.sector_size());
			assert!((self.start_lba + sector as u64) < self.end_lba);
			block.read_sector(self.start_lba as usize + sector, buf)
		})
	}

	fn in_sectors(&self, size: usize) -> usize {
		with_block_driver(|block| {
			align_up(size, block.sector_size()) / block.sector_size()
		})
	}

	fn block_size(&self) -> usize {
		*self
			.block_size
			.borrow_mut()
			.get_or_insert(with_block_driver(|block| block.sector_size()))
	}

	fn name(&self) -> &str {
		&self.name
	}
}

#[derive(Debug)]
pub enum GptError {
	HeaderSignatureNotFound,
	ConsumeError,
	UnexpectedEndOfStream,
}

pub struct Gpt {
	header: GptHeader,
	partitions: Vec<Option<Arc<SpinLock<PartitionEntry>>>>,
}

impl Gpt {
	pub fn new(header: GptHeader) -> Self {
		Self {
			header,
			partitions: vec![],
		}
	}

	pub fn parse_partition_table(&mut self) -> Result<(), GptError> {
		with_block_driver(|block| {
			let partition_table_size = align_up(
				(self.header.partition_entry_bytes
					* self.header.partition_count) as usize,
				block.sector_size(),
			);
			let partition_table_sectors =
				partition_table_size / block.sector_size();

			let mut buf = vec![0u8; partition_table_size];
			let start = self.header.partition_entry as usize;
			let end = start + partition_table_sectors;
			block.read_sectors(start..end, &mut buf);

			let mut bytes = BytesParser::new(&mut buf);

			for _ in 0..self.header.partition_count {
				match PartitionEntry::parse(
					&mut bytes,
					self.header.partition_entry_bytes as usize,
				) {
					Err(e) => return Err(e),
					Ok(part) => self
						.partitions
						.push(part.map(SpinLock::new).map(Arc::new)),
				}
			}

			Ok(())
		})
	}
}

pub static GPT: Once<Mutex<Gpt>> = Once::new();

pub fn partitions<'a>() -> Vec<Arc<SpinLock<dyn Partition>>> {
	GPT.lock()
		.partitions
		.iter()
		.filter(|p| p.is_some())
		.map(|p| p.as_ref().unwrap().clone() as Arc<SpinLock<dyn Partition>>)
		.collect::<Vec<_>>()
}

pub fn init() {
	let header = with_block_driver(|block| {
		let mut header_bytes = vec![0u8; 512];
		block.read_sector(1, &mut header_bytes);

		GptHeader::parse_bytes(&mut header_bytes)
	});

	if let Ok(header) = header {
		GPT.init(|| {
			let mut gpt = Gpt::new(header);
			let _ = gpt.parse_partition_table();
			Mutex::new(gpt)
		});
	} else {
		warn!("No GPT found on current block device");
	}
}

impl fmt::Debug for GptHeader {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
		f.debug_struct("GptHeader")
			.field(
				"signature",
				&String::from_utf8(self.signature.to_vec()).unwrap(),
			)
			.field("revision", &self.revision)
			.field("header_size", &self.header_size)
			.field("header_crc", &self.header_crc)
			.field("this_lba", &self.this_lba)
			.field("header_mirror", &self.header_mirror)
			.field("first_usable_block", &self.first_usable_block)
			.field("last_usable_block", &self.last_usable_block)
			.field("guid", &self.guid)
			.field("partition_entry", &self.partition_entry)
			.field("num_partitions", &self.partition_count)
			.field("partition_entry_bytes", &self.partition_entry_bytes)
			.field("partition_entry_crc", &self.partition_entry_crc)
			.finish()
	}
}
