use core::fmt;

use environment::{
	address::PAddr,
	arch::SERIAL0,
	bootinfo::BootInfo,
	print::{set_debug_printer, Printer},
};
use utils::once::Once;

use crate::font::{Font, BIZCAT};

#[derive(Default, Copy, Clone)]
#[repr(C)]
pub struct Pixel {
	b: u8,
	g: u8,
	r: u8,
	a: u8,
}

impl Pixel {
	pub fn new(r: u8, g: u8, b: u8) -> Self {
		Self { r, g, b, a: 255 }
	}

	pub fn hex(mut hex: &str) -> Self {
		if hex.starts_with("0x") {
			hex = &hex[2..];
		} else if hex.starts_with("#") {
			hex = &hex[1..];
		}

		assert!(hex.len() == 6);

		let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
		let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
		let b = u8::from_str_radix(&hex[4..6], 16).unwrap();

		Self { r, g, b, a: 255 }
	}

	pub fn set(&mut self, to: impl Into<Pixel>) {
		let to: Pixel = to.into();
		self.r = to.r;
		self.g = to.g;
		self.b = to.b;
		self.a = to.a;
	}
}

pub struct Framebuffer {
	ptr: PAddr,
	_pitch: usize,
	width: usize,
	height: usize,
	size: usize,
	_bpp: u8,
	row: usize,
	col: usize,
	font: &'static Font,
	pub fg: Pixel,
	pub bg: Pixel,
	colors: [Pixel; 16],
}

impl Framebuffer {
	pub fn new(bootinfo: &BootInfo, colors: [Pixel; 16]) -> Self {
		Self {
			ptr: bootinfo.framebuffer.addr,
			_pitch: bootinfo.framebuffer.pitch as usize,
			width: bootinfo.framebuffer.width as usize,
			height: bootinfo.framebuffer.height as usize,
			size: (bootinfo.framebuffer.width * bootinfo.framebuffer.height)
				as usize,
			_bpp: bootinfo.framebuffer.bpp,
			row: 0,
			col: 0,
			font: &BIZCAT,
			fg: colors[Colors::LightGray as usize],
			bg: colors[Colors::Black as usize],
			colors,
		}
	}

	pub fn clear(&mut self) {
		for i in 0..self.size {
			unsafe {
				*self.ptr.as_mut_ptr::<Pixel>().add(i) = self.bg;
			}
		}
	}

	pub fn reset(&mut self) {
		self.reset_fg();
		self.reset_bg();
	}

	#[inline(always)]
	pub fn reset_fg(&mut self) {
		self.fg = self.colors[Colors::LightGray as usize];
	}

	#[inline(always)]
	pub fn reset_bg(&mut self) {
		self.bg = self.colors[Colors::Black as usize];
	}

	pub fn draw(&mut self, c: char) {
		match c {
			'\n' => {
				self.row += self.font.stride as usize;
				self.col = 0;
			}
			_ => {
				if self.row + 1 > self.height {
					self.col = 0;
					self.row = self.height - self.font.height as usize;

					unsafe {
						core::ptr::copy(
							self.ptr
								.as_ptr::<Pixel>()
								.add(self.width * self.font.height as usize),
							self.ptr.as_mut_ptr::<Pixel>(),
							self.width
								* (self.height - self.font.height as usize),
						)
					}

					self.clear_row(self.cols() - 1);
				}

				let offset = c as usize * self.font.stride as usize;
				for y in 0..self.font.height as usize {
					for x in 0..self.font.width as usize {
						let cur_x = self.col + x;
						let cur_y = self.row + y;

						unsafe {
							let px = self
								.ptr
								.as_mut_ptr::<Pixel>()
								.add(cur_x + cur_y * self.width);

							if self.font.data[y as usize + offset] >> x & 1 == 1
							{
								*px = self.fg;
							} else {
								*px = self.bg;
							}
						}
					}
				}

				if self.col + self.font.width as usize >= self.width {
					self.draw('\n');
				} else {
					self.col += self.font.width as usize;
				}
			}
		}
	}

	fn clear_row(&self, row: usize) {
		let fh = self.font.height as usize;
		for y in (row * fh)..(row + 1) * fh {
			for x in 0..self.width {
				unsafe {
					*self.ptr.as_mut_ptr::<Pixel>().add(x + y * self.width) =
						self.bg;
				}
			}
		}
	}

	#[allow(unused)]
	fn rows(&self) -> usize {
		self.width / self.font.width as usize
	}

	fn cols(&self) -> usize {
		self.height / self.font.height as usize
	}
}

impl vte::Perform for Framebuffer {
	fn execute(&mut self, byte: u8) {
		self.draw(byte as char);
	}

	fn print(&mut self, c: char) {
		self.draw(c);
	}

	fn osc_dispatch(&mut self, _params: &[&[u8]], _bell_terminated: bool) {
		trace!("osc");
	}

	fn csi_dispatch(
		&mut self,
		params: &vte::Params,
		_intermediates: &[u8],
		_ignore: bool,
		action: char,
	) {
		use core::fmt::Write;

		struct Debug;

		impl Write for Debug {
			fn write_str(&mut self, s: &str) -> fmt::Result {
				for b in s.bytes() {
					SERIAL0.print_char(b);
				}

				Ok(())
			}
		}

		let mut light = false;

		match action {
			'm' => {
				for param in params {
					for sub in param {
						match sub {
							0 => self.reset(),
							1 => light = true,
							30..=37 => {
								if light {
									self.fg =
										self.colors[(sub - 22) as usize].into();
								} else {
									self.fg =
										self.colors[(sub - 30) as usize].into();
								}
							}
							39 => self.reset_fg(),
							40..=47 => {
								if light {
									self.bg =
										self.colors[(sub - 32) as usize].into();
								} else {
									self.bg =
										self.colors[(sub - 40) as usize].into();
								}
							}
							49 => self.reset_bg(),
							_ => {
								let _ = write!(Debug {}, "{:?}", params);
							}
						}
					}
				}
			}
			_ => self.reset(),
		}
	}

	fn esc_dispatch(
		&mut self,
		_intermediates: &[u8],
		_ignore: bool,
		_byte: u8,
	) {
		trace!("esc: {:?} {} {}", _intermediates, _ignore, _byte);
	}
}
static TERMINAL_PARSER: Once<spin::Mutex<vte::Parser>> = Once::new();
static FRAMEBUFFER: Once<spin::Mutex<Framebuffer>> = Once::new();

struct FramebufferPrinter;

impl Printer for FramebufferPrinter {
	fn print_bytes(&self, s: &[u8]) {
		for b in s {
			SERIAL0.send_char(*b);
			TERMINAL_PARSER.lock().advance(&mut *FRAMEBUFFER.lock(), *b);
		}
	}
}

pub fn init(bootinfo: &BootInfo) {
	#[allow(unused)]
	let campbell = [
		Pixel::hex("#0C0C0C"), // Black
		Pixel::hex("#C50F1F"), // Red
		Pixel::hex("#13A10E"), // Green
		Pixel::hex("#C19C00"), // Brown
		Pixel::hex("#0037DA"), // Blue
		Pixel::hex("#881798"), // Magenta
		Pixel::hex("#3A96DD"), // Cyan
		Pixel::hex("#CCCCCC"), // LightGray
		Pixel::hex("#767676"), // DarkGray
		Pixel::hex("#E74856"), // LightRed
		Pixel::hex("#16C60C"), // LightGreen
		Pixel::hex("#C19C00"), // Yellow
		Pixel::hex("#3B78FF"), // LightBlue
		Pixel::hex("#B4009E"), // Pink
		Pixel::hex("#61D6D6"), // LightCyan
		Pixel::hex("#F2F2F2"), // White
	];

	let nord = [
		Pixel::hex("#2E3440"), // Black
		Pixel::hex("#BF616A"), // Red
		Pixel::hex("#8FBCBB"), // Green
		Pixel::hex("#EBCB8B"), // Brown
		Pixel::hex("#81A1C1"), // Blue
		Pixel::hex("#B48EAD"), // Magenta
		Pixel::hex("#88C0D0"), // Cyan
		Pixel::hex("#E5E9F0"), // LightGray
		Pixel::hex("#3B4252"), // DarkGray
		Pixel::hex("#BF616A"), // LightRed
		Pixel::hex("#A3BE8C"), // LightGreen
		Pixel::hex("#EBCB8B"), // Yellow
		Pixel::hex("#5E81AC"), // LightBlue
		Pixel::hex("#B48EAD"), // Pink
		Pixel::hex("#88C0D0"), // LightCyan
		Pixel::hex("#4C566A"), // White
	];

	TERMINAL_PARSER.init(|| spin::Mutex::new(vte::Parser::new()));
	FRAMEBUFFER.init(|| {
		let mut framebuffer = Framebuffer::new(bootinfo, nord);
		framebuffer.clear();
		spin::Mutex::new(framebuffer)
	});

	set_debug_printer(&FramebufferPrinter);
}

#[repr(usize)]
pub enum Colors {
	Black = 0,
	Blue = 1,
	Green = 2,
	Cyan = 3,
	Red = 4,
	Magenta = 5,
	Brown = 6,
	LightGray = 7,
	DarkGray = 8,
	LightBlue = 9,
	LightGreen = 10,
	LightCyan = 11,
	LightRed = 12,
	Pink = 13,
	Yellow = 14,
	White = 15,
}
