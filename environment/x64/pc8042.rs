use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use spin::Mutex;
use x86::io::inb;

use lazy_static::lazy_static;

use crate::{arch::enable_irq, system};

pub const PS2KBD_IRQ: u8 = 1;
const DATA_PORT: u16 = 0x60;

pub struct PS2Kbd {
	data_port: u16,
	irq: u8,
}

pub static PS2KBD: PS2Kbd = PS2Kbd::new(DATA_PORT, PS2KBD_IRQ);

impl PS2Kbd {
	const fn new(data_port: u16, irq: u8) -> Self {
		Self { data_port, irq }
	}

	pub fn read_scancode(&self) -> u8 {
		unsafe { inb(self.data_port) }
	}

	pub fn irq(&self) -> u8 {
		self.irq
	}
}

lazy_static! {
	static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
		Mutex::new(Keyboard::new(HandleControl::Ignore));
}

pub(crate) fn ps2kbd_irq_handler() {
	let scancode = PS2KBD.read_scancode();

	let mut keyboard = KEYBOARD.lock();

	if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
		if let Some(key) = keyboard.process_keyevent(key_event) {
			match key {
				DecodedKey::Unicode(character) => {
					system().on_console_rx(character as u8);
				}
				DecodedKey::RawKey(_key) => {} // system().on_console_rx(key),
			}
		}
	}
}

pub fn init() {
	enable_irq(PS2KBD.irq());
}
