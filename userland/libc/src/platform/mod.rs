use core::{
	alloc::{GlobalAlloc, Layout},
	fmt, ptr,
};

use alloc::vec::Vec;

use self::{sys::Sys, types::*};

use crate::{
	io::{self, Write},
	start::ALLOCATOR,
};

pub mod types;

#[path = "circinus/mod.rs"]
pub(crate) mod sys;

#[allow(non_camel_case_types)]
#[no_mangle]
pub static mut environ: *mut *mut c_char = ptr::null_mut();

pub static mut OUR_ENVIRON: Vec<*mut c_char> = Vec::new();

pub unsafe fn alloc(size: usize) -> *mut c_void {
	let layout = Layout::new::<c_void>().repeat(size).unwrap();
	ALLOCATOR.alloc(layout.0) as *mut c_void
}

pub unsafe fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void {
	let layout = Layout::new::<c_void>().repeat(size).unwrap();
	ALLOCATOR.realloc(ptr as *mut u8, layout.0, size) as *mut c_void
}

pub unsafe fn free(ptr: *mut c_void) {
	ALLOCATOR.dealloc(ptr as *mut u8, Layout::new::<c_void>())
}

pub struct FileWriter(pub c_int);

impl FileWriter {
	pub fn write(&mut self, buf: &[u8]) -> isize {
		Sys::write(self.0, buf)
	}
}

impl fmt::Write for FileWriter {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		self.write(s.as_bytes());
		Ok(())
	}
}

pub struct StringWriter(pub *mut u8, pub usize);
impl Write for StringWriter {
	fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
		if self.1 > 1 {
			let copy_size = buf.len().min(self.1 - 1);
			unsafe {
				ptr::copy_nonoverlapping(buf.as_ptr(), self.0, copy_size);
				self.1 -= copy_size;

				self.0 = self.0.add(copy_size);
				*self.0 = 0;
			}
		}

		Ok(buf.len())
	}

	fn flush(&mut self) -> io::Result<()> {
		Ok(())
	}
}

impl fmt::Write for StringWriter {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		self.write(s.as_bytes()).unwrap();
		Ok(())
	}
}

impl WriteByte for StringWriter {
	fn write_u8(&mut self, byte: u8) -> fmt::Result {
		self.write(&[byte]).unwrap();
		Ok(())
	}
}

pub struct UnsafeStringWriter(pub *mut u8);

impl Write for UnsafeStringWriter {
	fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
		unsafe {
			ptr::copy_nonoverlapping(buf.as_ptr(), self.0, buf.len());
			self.0 = self.0.add(buf.len());
			*self.0 = b'\0';
		}
		Ok(buf.len())
	}

	fn flush(&mut self) -> io::Result<()> {
		Ok(())
	}
}

impl fmt::Write for UnsafeStringWriter {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		self.write(s.as_bytes()).unwrap();
		Ok(())
	}
}

impl WriteByte for UnsafeStringWriter {
	fn write_u8(&mut self, byte: u8) -> fmt::Result {
		self.write(&[byte]).unwrap();
		Ok(())
	}
}

pub struct CountingWriter<T> {
	pub inner: T,
	pub written: usize,
}

impl<T> CountingWriter<T> {
	pub fn new(writer: T) -> Self {
		Self {
			inner: writer,
			written: 0,
		}
	}
}

impl<T: fmt::Write> fmt::Write for CountingWriter<T> {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		self.written += s.len();
		self.inner.write_str(s)
	}
}

impl<T: WriteByte> WriteByte for CountingWriter<T> {
	fn write_u8(&mut self, byte: u8) -> fmt::Result {
		self.written += 1;
		self.inner.write_u8(byte)
	}
}

impl<T: Write> Write for CountingWriter<T> {
	fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
		let res = self.inner.write(buf);
		if let Ok(written) = res {
			self.written += written;
		}
		res
	}

	fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
		match self.inner.write_all(buf) {
			Ok(()) => (),
			Err(ref err) if err.kind() == io::ErrorKind::WriteZero => (),
			Err(err) => return Err(err),
		}
		self.written += buf.len();
		Ok(())
	}

	fn flush(&mut self) -> io::Result<()> {
		self.inner.flush()
	}
}

pub trait WriteByte: fmt::Write {
	fn write_u8(&mut self, byte: u8) -> fmt::Result;
}

impl<'a, W: WriteByte> WriteByte for &'a mut W {
	fn write_u8(&mut self, byte: u8) -> fmt::Result {
		(**self).write_u8(byte)
	}
}
