use alloc::sync::{Arc, Weak};
use core::fmt;

use api::{
	io::{self, OpenOptions},
	print::get_debug_printer,
	sync::SpinLock,
	user_buffer::{UserBufReader, UserBuffer},
	vfs::{self, File, NodeId, Stat},
	Result,
};

use crate::{
	process::process_group::ProcessGroup,
	tty::line_reader::{LineControl, LineReader},
};

pub struct DevConsole {
	line_reader: LineReader,
	stat: Stat,
}

impl DevConsole {
	pub fn new(node_id: NodeId) -> Self {
		Self {
			line_reader: LineReader::new(),
			stat: Stat {
				node_id,
				size: 0,
				kind: vfs::FileKind::CharDevice,
			},
		}
	}

	pub fn input_char(&self, ch: u8) {
		self.line_reader
			.write(([ch].as_slice()).into(), |ctrl| match ctrl {
				LineControl::Echo(ch) => {
					self.write(0, [ch].as_slice().into(), &OpenOptions::readwrite())
						.ok();
				}
				LineControl::Backspace => {
					get_debug_printer().print_bytes(b"\x08 \x08");
				}
			})
			.ok();
	}

	pub fn set_foreground_process_group(&self, pg: Weak<SpinLock<ProcessGroup>>) {
		self.line_reader.set_foreground_process_group(pg);
	}
}

impl fmt::Debug for DevConsole {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("devcon").finish()
	}
}

impl vfs::File for DevConsole {
	fn open(&self, _options: &io::OpenOptions) -> Result<Option<Arc<dyn vfs::File>>> {
		Ok(None)
	}

	fn read(
		&self,
		_offset: usize,
		dst: api::user_buffer::UserBufferMut<'_>,
		_options: &api::io::OpenOptions,
	) -> Result<usize> {
		self.line_reader.read(dst)
	}

	fn write(
		&self,
		_offset: usize,
		buf: UserBuffer<'_>,
		_options: &api::io::OpenOptions,
	) -> Result<usize> {
		let mut reader = UserBufReader::from(buf);
		let len = reader.buffer_len();
		while reader.remaining_len() > 0 {
			let mut tmp = [0; 128];
			let copied_len = reader.read_bytes(&mut tmp).expect("could not read");

			for ch in &tmp.as_slice()[..copied_len] {
				match ch {
					_ => {
						print!("{}", *ch as char)
					}
				}
			}
		}

		Ok(len)
	}

	fn stat(&self) -> Result<vfs::Stat> {
		Ok(self.stat)
	}
}
