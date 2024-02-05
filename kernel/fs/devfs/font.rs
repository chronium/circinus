use core::mem::size_of;

use alloc::fmt;
use api::vfs::{self, NodeId, Stat};

impl fmt::Debug for Font {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Font")
      .field("width", &self.width)
      .field("height", &self.height)
      .field("max_glyph", &self.max_glyph)
      .finish()
  }
}

pub struct Font {
  stat: Stat,
  width: usize,
  height: usize,
  stride: usize,
  max_glyph: usize,
  data: &'static [u8],
}

impl Font {
  pub fn new(
    node_id: NodeId,
    width: usize,
    height: usize,
    stride: usize,
    max_glyph: usize,
    data: &'static [u8],
  ) -> Self {
    Font {
      stat: Stat {
        node_id,
        size: size_of::<usize>() * 4 + data.len(),
        kind: vfs::FileKind::BlockDevice,
      },
      width,
      height,
      stride,
      max_glyph,
      data,
    }
  }
}

impl vfs::File for Font {
  fn open(
    &self,
    options: &api::io::OpenOptions,
  ) -> api::Result<Option<alloc::sync::Arc<dyn vfs::File>>> {
    Ok(None)
  }

  fn read(
    &self,
    offset: usize,
    dst: api::user_buffer::UserBufferMut<'_>,
    options: &api::io::OpenOptions,
  ) -> api::Result<usize> {
    let mut writer = api::user_buffer::UserBufWriter::from(dst);

    writer.write(self.width);
    writer.write(self.height);
    writer.write(self.stride);
    writer.write(self.max_glyph);
    writer.write_bytes(self.data);

    Ok(self.stat.size)
  }

  fn write(
    &self,
    offset: usize,
    buf: api::user_buffer::UserBuffer<'_>,
    options: &api::io::OpenOptions,
  ) -> api::Result<usize> {
    Ok(0)
  }

  fn stat(&self) -> api::Result<vfs::Stat> {
    Ok(self.stat)
  }
}
