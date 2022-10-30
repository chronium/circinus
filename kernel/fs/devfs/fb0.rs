use core::{fmt, mem};

use alloc::sync::Arc;
use api::{
	io,
	user_buffer::{UserBufReader, UserBufWriter},
	vfs::{self, NodeId, Stat},
	Result,
};

use crate::framebuffer::FRAMEBUFFER;

pub struct Framebuffer {
	stat: Stat,
}

impl fmt::Debug for Framebuffer {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Framebuffer").finish()
	}
}

impl Framebuffer {
	pub fn new(node_id: NodeId) -> Self {
		let (width, height) = FRAMEBUFFER.lock().size();
		Framebuffer {
			stat: Stat {
				node_id,
				size: width * height,
				kind: vfs::FileKind::BlockDevice,
			},
		}
	}
}

impl vfs::File for Framebuffer {
	fn open(&self, options: &io::OpenOptions) -> Result<Option<Arc<dyn vfs::File>>> {
		Ok(None)
	}

	fn read(
		&self,
		offset: usize,
		dst: api::user_buffer::UserBufferMut<'_>,
		options: &io::OpenOptions,
	) -> Result<usize> {
		#[derive(Debug, Copy, Clone)]
		struct FramebufferInfo {
			pub width: usize,
			pub height: usize,
		}

		let mut writer = UserBufWriter::from(dst);

		let framebuffer = FRAMEBUFFER.lock();
		let (width, height) = framebuffer.size();

		let info = FramebufferInfo { width, height };
		writer.write(info)?;

		Ok(mem::size_of::<FramebufferInfo>())
	}

	fn write(
		&self,
		offset: usize,
		buf: api::user_buffer::UserBuffer<'_>,
		options: &api::io::OpenOptions,
	) -> Result<usize> {
		let mut reader = UserBufReader::from(buf);

		let mut framebuffer = FRAMEBUFFER.lock();
		let (width, height) = framebuffer.size();

		let mut buf = vec![0u8; width * height * 4];
		reader.read_bytes(buf.as_mut_slice()).unwrap();

		framebuffer.write(&buf);

		Ok(reader.buffer_len())
	}

	fn stat(&self) -> Result<Stat> {
		Ok(self.stat)
	}
}
