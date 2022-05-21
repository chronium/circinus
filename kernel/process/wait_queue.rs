use alloc::{collections::VecDeque, sync::Arc};
use api::{process::ProcessState, sync::SpinLock, ErrorKind, Process, Result};

use super::{current_process, switch};

pub struct WaitQueue {
	queue: SpinLock<VecDeque<Arc<super::Process>>>,
}

impl WaitQueue {
	pub fn new() -> WaitQueue {
		WaitQueue {
			queue: SpinLock::new(VecDeque::new()),
		}
	}

	pub fn sleep_signalable_until<F, R>(
		&self,
		mut sleep_if_none: F,
	) -> Result<R>
	where
		F: FnMut() -> Result<Option<R>>,
	{
		loop {
			Process::set_state(ProcessState::BlockedSignalable);
			self.queue.lock().push_back(current_process().clone());

			if Process::has_pending_signals() {
				Process::resume();
				self.queue
					.lock()
					.retain(|proc| !Arc::ptr_eq(proc, current_process()));
				return Err(ErrorKind::Interrupted.into());
			}

			let ret_value = match sleep_if_none() {
				Ok(Some(ret_value)) => Some(Ok(ret_value)),
				Ok(None) => None,
				Err(err) => Some(Err(err)),
			};

			if let Some(ret_value) = ret_value {
				Process::resume();
				self.queue
					.lock()
					.retain(|proc| !Arc::ptr_eq(proc, current_process()));
				return ret_value;
			}

			switch();
		}
	}

	pub fn _wake_one(&self) {
		let mut queue = self.queue.lock();
		if let Some(process) = queue.pop_front() {
			process._resume();
		}
	}

	pub fn wake_all(&self) {
		let mut queue = self.queue.lock();
		while let Some(process) = queue.pop_front() {
			process._resume();
		}
	}
}
