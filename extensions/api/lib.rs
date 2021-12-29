#![no_std]
#![feature(box_syntax)]

extern crate log;

#[macro_use]
extern crate alloc;

use core::mem::size_of;

pub use environment::{debug_warn, print, println, warn_if_err, warn_once};
pub use log::{debug, error, info, trace, warn};

pub mod address {
	pub use environment::address::{PAddr, VAddr};
}

pub mod arch {
	pub use environment::arch::{idle, PAGE_SIZE};
}

pub mod mm {
	pub use environment::page_allocator::{
		alloc_pages, AllocPageFlags, PageAllocError,
	};
}

pub mod sync {
	pub use environment::spinlock::{SpinLock, SpinLockGuard};
}

pub mod owo_colors {
	pub use owo_colors::*;
}

pub unsafe trait AsBuf: Sized {
	fn as_buf(&self) -> &[u8] {
		unsafe {
			core::slice::from_raw_parts(
				self as *const _ as _,
				size_of::<Self>(),
			)
		}
	}
	fn as_buf_mut(&mut self) -> &mut [u8] {
		unsafe {
			core::slice::from_raw_parts_mut(
				self as *mut _ as _,
				size_of::<Self>(),
			)
		}
	}
}

pub mod driver;
pub mod guid;
pub mod kernel;
pub mod posix;
pub mod schema;
pub mod uuid;
