#![no_std]

#[macro_use]
extern crate alloc;

pub mod device;
pub mod transport;

pub use api::driver::VirtioKind as Kind;

#[macro_export]
macro_rules! info {
	($kind:path, $first:expr, $second:expr) => {{
		use owo_colors::OwoColorize;
		api::info!("{}: {} {}", $kind.yellow(), $first.cyan(), $second.red());
	}};
}
