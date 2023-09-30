use alloc::boxed::Box;
use api::driver::block::BlockDriver;
use atomic_refcell::AtomicRefCell;
use owo_colors::OwoColorize;

static BLOCK_DRIVER: AtomicRefCell<Option<Box<dyn BlockDriver>>> =
	AtomicRefCell::new(None);

pub fn register_block_driver(driver: Box<dyn BlockDriver>) {
	assert!(
		BLOCK_DRIVER.borrow().is_none(),
		"multiple block drivers are not supported"
	);
	info!("registered driver {}", driver.name().green());
	*BLOCK_DRIVER.borrow_mut() = Some(driver)
}

pub fn with_block_driver<F, R>(f: F) -> R
where
	F: FnOnce(&Box<dyn BlockDriver>) -> R,
{
	let driver = BLOCK_DRIVER.borrow();
	f(driver.as_ref().expect("no block driver"))
}
