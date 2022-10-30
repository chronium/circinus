use super::types::*;

pub struct Sys;

impl Sys {
	pub fn brk(caddr: *mut c_void) -> *mut c_void {
		syscall::brk(caddr as usize) as *mut c_void
	}

	pub fn write(fd: c_int, buf: &[u8]) -> ssize_t {
		syscall::write(fd, buf) as ssize_t
	}

	pub fn exit(status: c_int) -> ! {
		syscall::exit(status)
	}
}
