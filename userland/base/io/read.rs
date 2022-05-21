use alloc::string::String;

use crate::io;

pub trait Read {
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>;
}

pub trait BufRead {
	fn read_line(&mut self, buf: &mut String) -> io::Result<usize>;
}
