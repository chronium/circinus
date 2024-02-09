use ps2_mouse::Mouse;
use spin::{Lazy, Mutex};

use crate::system;

pub const PS2MOUSE_IRQ: u8 = 12;
const DATA_PORT: u16 = 0x60;

pub struct PS2Mouse {
  data_port: u16,
  irq: u8,
}

pub static PS2MOUSE: PS2Mouse = PS2Mouse::new(DATA_PORT, PS2MOUSE_IRQ);

impl PS2Mouse {
  const fn new(data_port: u16, irq: u8) -> Self {
    Self { data_port, irq }
  }

  pub fn read_data(&self) -> u8 {
    unsafe { x86::io::inb(self.data_port) }
  }

  pub fn irq(&self) -> u8 {
    self.irq
  }
}

pub static MOUSE: Lazy<Mutex<Mouse>> = Lazy::new(|| Mutex::new(Mouse::new()));

pub fn init() {
  MOUSE.lock().init();
  MOUSE
    .lock()
    .set_on_complete(|state| system().on_mouse_event(state));
  crate::arch::enable_irq(PS2MOUSE.irq());
}

pub(crate) fn ps2mouse_irq_handler() {
  let packet = PS2MOUSE.read_data();
  MOUSE.lock().process_packet(packet);
}
