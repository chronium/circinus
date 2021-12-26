use alloc::sync::Arc;
use environment::spinlock::SpinLock;
use utils::{lazy::Lazy, once::Once};

use self::scheduler::Scheduler;

pub use process::{Pid, Process, ProcessState};
pub use switch::switch;

cpu_local! {
	static ref CURRENT: Lazy<Arc<Process>> = Lazy::new();
}

cpu_local! {
	// TODO: Should be pub(super)
	pub static ref IDLE_THREAD: Lazy<Arc<Process>> = Lazy::new();
}

static SCHEDULER: Once<SpinLock<Scheduler>> = Once::new();

pub fn current_process() -> &'static Arc<Process> {
	CURRENT.get()
}

pub fn init() {
	SCHEDULER.init(|| SpinLock::new(Scheduler::new()));
	let idle_thread = Process::new_idle_thread(); // TODO: .unwrap();
	IDLE_THREAD.as_mut().set(idle_thread.clone());
	CURRENT.as_mut().set(idle_thread);
}

pub mod process;
pub mod scheduler;
pub mod switch;
