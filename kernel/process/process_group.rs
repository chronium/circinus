use alloc::{
	collections::BTreeMap,
	sync::{Arc, Weak},
	vec::Vec,
};

use api::{process::PgId, sync::SpinLock};

use super::Process;

pub static PROCESS_GROUPS: SpinLock<BTreeMap<PgId, Arc<SpinLock<ProcessGroup>>>> =
	SpinLock::new(BTreeMap::new());

pub struct ProcessGroup {
	pgid: PgId,
	processes: Vec<Weak<Process>>,
}

impl ProcessGroup {
	/// Create a new process group.
	pub fn new(pgid: PgId) -> Arc<SpinLock<ProcessGroup>> {
		let pg = Arc::new(SpinLock::new(ProcessGroup {
			pgid,
			processes: Vec::new(),
		}));

		PROCESS_GROUPS.lock().insert(pgid, pg.clone());
		pg
	}

	pub fn add(&mut self, proc: Weak<Process>) {
		self.processes.push(proc);
	}

	pub fn remove_dropped_processes(&mut self) {
		self.processes.retain(|proc| proc.upgrade().is_some());
		if self.processes.is_empty() {
			PROCESS_GROUPS.lock().remove(&self.pgid);
		}
	}
}
