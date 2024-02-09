use core::{mem::size_of, sync::atomic::AtomicI16};

use alloc::{collections::VecDeque, fmt, vec::Vec};
use api::{
  bitflags::bitflags,
  vfs::{self, File, Stat},
};
use ps2_mouse::MouseState;
use spin::Mutex;

use ringbuf::{Consumer, HeapRb, LocalRb, Producer, Rb, StaticRb};

pub struct Mouse {
  stat: Stat,
  dx: AtomicI16,
  dy: AtomicI16,
}

impl fmt::Debug for Mouse {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Mouse").finish()
  }
}

bitflags! {
  #[repr(transparent)]
  #[derive(Debug, Clone, Copy)]
  pub struct MouseButtons: u8 {
    const LEFT = 0b001;
    const RIGHT = 0b010;
    const MIDDLE = 0b100;
  }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MousePacket {
  pub x: i16,
  pub y: i16,
  pub buttons: MouseButtons,
}

impl From<MouseState> for MousePacket {
  fn from(state: MouseState) -> Self {
    let buttons = if state.left_button_down() {
      MouseButtons::LEFT
    } else {
      MouseButtons::empty()
    } | if state.right_button_down() {
      MouseButtons::RIGHT
    } else {
      MouseButtons::empty()
    };

    MousePacket {
      x: state.get_x(),
      y: state.get_y(),
      buttons,
    }
  }
}

impl Mouse {
  pub fn new(node_id: api::vfs::NodeId) -> Self {
    Mouse {
      stat: Stat {
        node_id,
        size: size_of::<MousePacket>(),
        kind: api::vfs::FileKind::BlockDevice,
      },
      dx: AtomicI16::new(0),
      dy: AtomicI16::new(0),
    }
  }

  pub fn push(&self, packet: MouseState) {
    self
      .dx
      .fetch_add(packet.get_x(), core::sync::atomic::Ordering::Relaxed);
    self
      .dy
      .fetch_add(packet.get_y(), core::sync::atomic::Ordering::Relaxed);
  }
}

impl vfs::File for Mouse {
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
    let mut reader = api::user_buffer::UserBufWriter::from(dst);

    reader.write(MousePacket {
      x: self.dx.swap(0, core::sync::atomic::Ordering::Relaxed),
      y: self.dy.swap(0, core::sync::atomic::Ordering::Relaxed),
      buttons: MouseButtons::empty(),
    })?;
    Ok(size_of::<MousePacket>())
  }

  fn write(
    &self,
    offset: usize,
    src: api::user_buffer::UserBuffer<'_>,
    options: &api::io::OpenOptions,
  ) -> api::Result<usize> {
    Ok(0)
  }

  fn stat(&self) -> api::Result<Stat> {
    Ok(self.stat)
  }
}
