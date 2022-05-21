use base::io::{self, BufRead, Read};

pub struct Stdin;

pub fn stdin() -> Stdin {
	Stdin
}

impl Read for Stdin {
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
		Ok(crate::sys::read(super::STDIN as i32, buf))
	}
}

impl BufRead for Stdin {
	fn read_line(&mut self, buf: &mut alloc::string::String) -> io::Result<usize> {
		// TODO
		buf.reserve(512usize - buf.capacity());
		self.read(unsafe { buf.as_bytes_mut() })
	}
}
