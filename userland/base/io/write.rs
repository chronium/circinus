use crate::io;

pub trait Write {
	fn write(&mut self, buf: &[u8]) -> io::Result<usize>;
	fn flush(&mut self) -> io::Result<()>;
}
