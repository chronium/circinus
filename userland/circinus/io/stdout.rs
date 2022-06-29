use core::fmt;

use base::io::{self, Write};

pub struct Stdout;

pub fn stdout() -> Stdout {
	Stdout
}

impl Write for Stdout {
	fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
		Ok(crate::sys::write(super::STDOUT as i32, buf))
	}

	fn flush(&mut self) -> io::Result<()> {
		Ok(())
	}
}

impl fmt::Write for Stdout {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		self.write(s.as_bytes()).ok();
		Ok(())
	}
}
