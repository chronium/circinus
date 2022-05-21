use core::{mem::size_of, slice::from_raw_parts};

use api::{ErrorKind, Result};
use environment::address::UserVAddr;
use goblin::elf64::{
	header::{Header, ELFMAG, EM_X86_64, ET_EXEC},
	program_header::ProgramHeader,
};

pub struct Elf<'a> {
	header: &'a Header,
	program_headers: &'a [ProgramHeader],
}

impl<'a> Elf<'a> {
	pub fn parse(buf: &'a [u8]) -> Result<Self> {
		if buf.len() < size_of::<Header>() {
			debug_warn!("ELF header buffer is too short");
			return Err(ErrorKind::NotExecutable.into());
		}

		let header = unsafe { &*(buf.as_ptr() as *const Header) };
		if &header.e_ident[..4] != ELFMAG {
			debug_warn!("invalid ELF magic");
			return Err(ErrorKind::NotExecutable.into());
		}

		if header.e_machine != EM_X86_64 {
			debug_warn!("invalid ELF e_machine");
			return Err(ErrorKind::NotExecutable.into());
		}

		if header.e_type != ET_EXEC {
			debug_warn!("ELF is not executable");
			return Err(ErrorKind::NotExecutable.into());
		}

		let program_headers = unsafe {
			from_raw_parts(
				&buf[header.e_phoff as usize] as *const _
					as *const ProgramHeader,
				header.e_phnum as usize,
			)
		};

		Ok(Self {
			header,
			program_headers,
		})
	}

	pub fn entry(&self) -> Result<UserVAddr> {
		println!("entry {:012x}", self.header.e_entry);
		UserVAddr::new_nonnull(self.header.e_entry as usize).map_err(Into::into)
	}

	pub fn header(&self) -> &Header {
		self.header
	}

	pub fn program_headers(&self) -> &[ProgramHeader] {
		self.program_headers
	}
}
