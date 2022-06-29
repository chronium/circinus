use core::convert::TryInto;

use alloc::{string::String, vec::Vec};

fn align_down(value: usize, align: usize) -> usize {
	value & !(align - 1)
}

fn align_up(value: usize, align: usize) -> usize {
	align_down(value + align - 1, align)
}

#[derive(Debug, PartialEq)]
pub enum BytesParserError {
	TooShort,
	Utf8Parse,
}

pub struct BytesParser<'a> {
	buffer: &'a [u8],
	current: usize,
}

impl<'a> BytesParser<'a> {
	pub fn new(buffer: &'a [u8]) -> Self {
		Self { buffer, current: 0 }
	}

	pub fn remaining(&self) -> &[u8] {
		&self.buffer[self.current..]
	}

	pub fn remaining_len(&self) -> usize {
		self.buffer.len() - self.current
	}

	pub fn remaining_len_at(&self, offset: usize) -> usize {
		self.buffer.len() - self.current - offset
	}

	pub fn skip(&mut self, len: usize) -> Result<(), BytesParserError> {
		if self.current + len > self.buffer.len() {
			return Err(BytesParserError::TooShort);
		}

		self.current += len;
		Ok(())
	}

	pub fn skip_until_alignment(
		&mut self,
		align: usize,
	) -> Result<(), BytesParserError> {
		let next = align_up(self.current, align);
		if next > self.buffer.len() {
			return Err(BytesParserError::TooShort);
		}

		self.current = next;
		Ok(())
	}

	pub fn consume_u8(&mut self) -> Result<u8, BytesParserError> {
		let res = self.peek_bytes(1)?;

		self.current += 1;
		Ok(res[0])
	}

	pub fn consume_bytes(
		&mut self,
		len: usize,
	) -> Result<&'a [u8], BytesParserError> {
		let res = self.peek_bytes(len)?;

		self.current += len;
		Ok(res)
	}

	pub fn peek_bytes(&self, len: usize) -> Result<&'a [u8], BytesParserError> {
		self.peek_bytes_at(len, 0)
	}

	pub fn peek_bytes_at(
		&self,
		len: usize,
		offset: usize,
	) -> Result<&'a [u8], BytesParserError> {
		let at = self.current + offset;

		if at + len > self.buffer.len() {
			return Err(BytesParserError::TooShort);
		}

		Ok(&self.buffer[at..at + len])
	}

	pub fn consume_le_u16(&mut self) -> Result<u16, BytesParserError> {
		let value = self.peek_le_u16_at(0)?;
		self.current += 2;
		Ok(value)
	}

	pub fn peek_le_u16_at(
		&self,
		offset: usize,
	) -> Result<u16, BytesParserError> {
		let at = self.current + offset;

		if self.remaining_len_at(offset) < 2 {
			return Err(BytesParserError::TooShort);
		}

		Ok(u16::from_le_bytes(
			self.buffer[at..at + 2].try_into().unwrap(),
		))
	}

	pub fn consume_be_u16(&mut self) -> Result<u16, BytesParserError> {
		let value = self.peek_be_u16_at(0)?;
		self.current += 2;
		Ok(value)
	}

	pub fn peek_be_u16_at(
		&self,
		offset: usize,
	) -> Result<u16, BytesParserError> {
		let at = self.current + offset;

		if self.remaining_len_at(offset) < 2 {
			return Err(BytesParserError::TooShort);
		}

		Ok(u16::from_be_bytes(
			self.buffer[at..at + 2].try_into().unwrap(),
		))
	}

	pub fn consume_le_u32(&mut self) -> Result<u32, BytesParserError> {
		if self.remaining_len() < 4 {
			return Err(BytesParserError::TooShort);
		}

		let value = u32::from_le_bytes(
			self.buffer[self.current..self.current + 4]
				.try_into()
				.unwrap(),
		);
		self.current += 4;
		Ok(value)
	}

	pub fn consume_be_u32(&mut self) -> Result<u32, BytesParserError> {
		if self.remaining_len() < 4 {
			return Err(BytesParserError::TooShort);
		}

		let value = u32::from_be_bytes(
			self.buffer[self.current..self.current + 4]
				.try_into()
				.unwrap(),
		);
		self.current += 4;
		Ok(value)
	}

	pub fn consume_le_u64(&mut self) -> Result<u64, BytesParserError> {
		if self.remaining_len() < 8 {
			return Err(BytesParserError::TooShort);
		}

		let value = u64::from_le_bytes(
			self.buffer[self.current..self.current + 8]
				.try_into()
				.unwrap(),
		);
		self.current += 8;
		Ok(value)
	}

	pub fn consume_le_i32(&mut self) -> Result<i32, BytesParserError> {
		if self.remaining_len() < 4 {
			return Err(BytesParserError::TooShort);
		}

		let value = i32::from_le_bytes(
			self.buffer[self.current..self.current + 4]
				.try_into()
				.unwrap(),
		);
		self.current += 4;
		Ok(value)
	}

	pub fn consume_cstr(
		&mut self,
		len: usize,
	) -> Result<String, BytesParserError> {
		let bytes = self.consume_bytes(len)?;

		String::from_utf8(
			bytes
				.iter()
				.filter(|&&b| b != 0)
				.map(|b| *b)
				.collect::<Vec<_>>(),
		)
		.map_err(|_| BytesParserError::Utf8Parse)
	}
}
