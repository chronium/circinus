use core::sync::atomic::{AtomicUsize, Ordering};

use crate::process;

const PREEMPT_PER_TICKS: usize = 30;
static MONOTONIC_TICKS: AtomicUsize = AtomicUsize::new(0);

pub fn handle_timer_irq() {
	let ticks = MONOTONIC_TICKS.fetch_add(1, Ordering::Relaxed);

	if ticks % PREEMPT_PER_TICKS == 0 {
		process::switch();
	}
}
