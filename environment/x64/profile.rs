pub fn read_clock_counter() -> u64 {
	unsafe { x86::time::rdtscp().0 }
}
