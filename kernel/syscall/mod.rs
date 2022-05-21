use api::{ctypes::c_int, vfs::Fd, Error, ErrorKind, Result};
use environment::{address::UserVAddr, arch::PtRegs};

const SYS_WRITE: usize = 1;
const SYS_READ: usize = 2;
const SYS_BRK: usize = 128;
const SYS_EXIT: usize = -1isize as usize;

pub(self) const MAX_READ_WRITE_LEN: usize = core::isize::MAX as usize;

pub struct SyscallHandler<'a> {
	pub frame: &'a mut PtRegs,
}

impl<'a> SyscallHandler<'a> {
	pub fn new(frame: &'a mut PtRegs) -> SyscallHandler<'a> {
		SyscallHandler { frame }
	}

	pub fn dispatch(
		&mut self,
		a1: usize,
		a2: usize,
		a3: usize,
		a4: usize,
		a5: usize,
		a6: usize,
		n: usize,
	) -> Result<isize> {
		let ret = self.do_dispatch(a1, a2, a3, a4, a5, a6, n).map_err(|err| {
			debug_warn!("{}: error: {:?}", syscall_name_by_number(n), err);
			err
		});

		// TODO:
		// if let Err(err) = Process::try_delivering_signal(self.frame) {
		// debug_warn!("failed to setup the signal stack: {:?}", err);
		// }

		ret
	}

	pub fn do_dispatch(
		&mut self,
		a1: usize,
		a2: usize,
		a3: usize,
		a4: usize,
		a5: usize,
		a6: usize,
		n: usize,
	) -> Result<isize> {
		match n {
			SYS_WRITE => self.sys_write(Fd::new(a1 as i32), UserVAddr::new_nonnull(a2)?, a3),
			SYS_READ => self.sys_read(Fd::new(a1 as i32), UserVAddr::new_nonnull(a2)?, a3),
			SYS_EXIT => self.sys_exit(a1 as c_int),
			SYS_BRK => self.sys_brk(UserVAddr::new(a1)),
			_ => {
				debug_warn!(
					"unimplemented system call: {} (n={})",
					syscall_name_by_number(n),
					n,
				);
				Err(Error::new(ErrorKind::NoSyscall))
			}
		}
	}
}

fn syscall_name_by_number(n: usize) -> &'static str {
	match n {
		1 => "write",
		2 => "read",
		128 => "brk",
		SYS_EXIT => "exit",
		_ => "(unknown)",
	}
}

pub(self) mod brk;
pub(self) mod exit;
pub(self) mod read;
pub(self) mod write;