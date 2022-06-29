impl<'a> super::SyscallHandler<'a> {
	pub fn sys_exit(&mut self, status: api::ctypes::c_int) -> ! {
		api::Process::exit(status)
	}
}
