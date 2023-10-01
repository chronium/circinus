use api::Result;
use environment::address::UserVAddr;

use crate::process::current_process;

use super::SyscallHandler;

impl<'a> SyscallHandler<'a> {
	pub fn sys_brk(&mut self, new_heap_end: Option<UserVAddr>) -> Result<isize> {
		let current = current_process();
		let vm_ref = current.vm();
		let mut vm = vm_ref.as_ref().unwrap().lock();
		if let Some(new_heap_end) = new_heap_end {
			vm.expand_heap_to(new_heap_end)?;
		}
		trace!("brk {:x}", vm.heap_end().value());
		Ok(vm.heap_end().value() as isize)
	}
}
