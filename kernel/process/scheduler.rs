use alloc::collections::VecDeque;
use api::process::Pid;
use environment::spinlock::SpinLock;

pub struct Scheduler {
	run_queue: SpinLock<VecDeque<Pid>>,
}

impl Scheduler {
	/// Creates a scheduler.
	pub fn new() -> Self {
		Self {
			run_queue: SpinLock::new(VecDeque::new()),
		}
	}

	/// Enqueues a process into the runqueue.
	pub fn enqueue(&self, pid: Pid) {
		self.run_queue.lock().push_back(pid);
	}

	/// Returns the next process to run.
	///
	/// The process is removed from the runqueue so you need to enqueue it by
	/// [`Scheduler::enqueue`] again.
	pub fn pick_next(&self) -> Option<Pid> {
		self.run_queue.lock().pop_front()
	}

	/// Removes the process from the runqueue.
	pub fn remove(&self, pid: Pid) {
		self.run_queue.lock().retain(|p| *p != pid);
	}
}
