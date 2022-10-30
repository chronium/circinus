pub mod constants;
pub mod default;

use alloc::{boxed::Box, vec::Vec};
use core::{
	ops::{Deref, DerefMut},
	slice,
};
use core_io::{BufWriter, LineWriter, Write};
use spin::{Mutex, MutexGuard};

use crate::{file::File, platform::types::*};

pub use self::{constants::*, default::*};

use super::string::strlen;

enum Buffer<'a> {
	Borrowed(&'a mut [u8]),
	Owned(Vec<u8>),
}

pub trait Pending {
	fn pending(&self) -> size_t;
}

impl<W: core_io::Write> Pending for BufWriter<W> {
	fn pending(&self) -> size_t {
		self.buf.len() as size_t
	}
}

impl<W: core_io::Write> Pending for LineWriter<W> {
	fn pending(&self) -> size_t {
		self.inner.buf.len() as size_t
	}
}

pub trait Writer: Write + Pending {}

impl<W: core_io::Write> Writer for BufWriter<W> {}
impl<W: core_io::Write> Writer for LineWriter<W> {}

pub struct FILE {
	lock: Mutex<()>,

	file: File,
	pub(crate) flags: c_int,
	read_buf: Buffer<'static>,
	read_pos: usize,
	read_size: usize,
	unget: Vec<u8>,
	pub(crate) writer: Box<dyn Writer + Send>,

	pid: Option<c_int>,

	pub(crate) orientation: c_int,
}

impl Write for FILE {
	fn write(&mut self, buf: &[u8]) -> core_io::Result<usize> {
		self.writer.write(buf) // TODO: Error
	}

	fn flush(&mut self) -> core_io::Result<()> {
		self.writer.flush() // TODO: Error
	}
}

impl FILE {
	pub fn lock(&mut self) -> LockGuard {
		unsafe { flockfile(self) }
		LockGuard(self)
	}

	pub fn try_set_orientation(&mut self, mode: c_int) -> c_int {
		let stream = self.lock();
		stream.0.try_set_orientation_unlocked(mode)
	}

	pub fn try_set_orientation_unlocked(&mut self, mode: c_int) -> c_int {
		if self.orientation == 0 {
			self.orientation = match mode {
				1..=i32::MAX => 1,
				i32::MIN..=-1 => -1,
				0 => self.orientation,
			};
		}
		self.orientation
	}

	pub fn try_set_byte_orientation_unlocked(&mut self) -> core::result::Result<(), c_int> {
		match self.try_set_orientation_unlocked(-1) {
			i32::MIN..=-1 => Ok(()),
			x => Err(x),
		}
	}
}

#[no_mangle]
pub unsafe extern "C" fn flockfile(file: *mut FILE) {
	MutexGuard::leak((*file).lock.lock());
}

#[no_mangle]
pub unsafe extern "C" fn funlockfile(file: *mut FILE) {
	(*file).lock.force_unlock();
}

#[no_mangle]
pub unsafe extern "C" fn puts(s: *const c_char) -> c_int {
	let mut stream = (&mut *stdout).lock();
	if let Err(_) = (*stream).try_set_byte_orientation_unlocked() {
		return -1;
	}

	let buf = slice::from_raw_parts(s as *mut u8, strlen(s));

	if stream.write_all(&buf).is_err() {
		return -1;
	}
	if stream.write(&[b'\n']).is_err() {
		return -1;
	}
	0
}

pub struct LockGuard<'a>(&'a mut FILE);

impl<'a> Deref for LockGuard<'a> {
	type Target = FILE;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<'a> DerefMut for LockGuard<'a> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.0
	}
}

impl<'a> Drop for LockGuard<'a> {
	fn drop(&mut self) {
		unsafe {
			funlockfile(self.0);
		}
	}
}
