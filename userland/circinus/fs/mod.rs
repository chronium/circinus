use base::ctypes::c_int;

pub mod path;
pub mod stat;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Fd(c_int);

impl Fd {
	pub const fn new(value: i32) -> Fd {
		Fd(value)
	}

	pub const fn as_int(self) -> c_int {
		self.0
	}

	pub const fn as_usize(self) -> usize {
		self.0 as usize
	}
}
