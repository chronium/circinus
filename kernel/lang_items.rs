use core::sync::atomic::AtomicBool;

pub static PANICKED: AtomicBool = AtomicBool::new(false);

/// This function is called on panic.
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
	use core::sync::atomic::Ordering;

	if PANICKED.load(Ordering::SeqCst) {
		environment::print::get_debug_printer().print_bytes(b"\ndouble panic!\n");
		environment::arch::halt();
	}

	PANICKED.store(true, Ordering::SeqCst);
	error!("{}", info);
	environment::backtrace::backtrace();
	environment::arch::halt();
}
