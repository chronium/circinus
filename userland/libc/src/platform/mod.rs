use core::fmt;

use self::{sys::Sys, types::c_int};

pub mod types;

#[path = "circinus/mod.rs"]
pub(crate) mod sys;

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
