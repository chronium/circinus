#![no_std]
#![feature(box_syntax)]

extern crate log;

#[macro_use]
extern crate alloc;

use core::mem::size_of;

use alloc::sync::Arc;
use ctypes::c_int;
use environment::spinlock::SpinLock;
use kernel::kernel_ops;
use process::{Pid, ProcessState};
use vfs::mount::Rootfs;

pub use environment::{debug_warn, print, println, warn_if_err, warn_once};
pub use log::{debug, error, info, trace, warn};
pub use process::ProcessOps;
pub use result::{Error, ErrorKind, Result};

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

pub mod bitflags {
	pub use bitflags::*;
}

pub mod hashbrown {
	pub use hashbrown::*;
}

pub mod memoffset {
	pub use memoffset::*;
}

macro_rules! proc {
	($k:expr) => {{
		$k.current_process().unwrap()
	}};
}

pub struct Process;

impl Process {
	pub fn rootfs() -> Arc<SpinLock<Rootfs>> {
		proc!(kernel_ops()).rootfs().clone()
	}

	pub fn exit(status: c_int) -> ! {
		proc!(kernel_ops()).exit(status)
	}

	pub fn get_open_file_by_fid(
		fd: vfs::Fd,
	) -> result::Result<Arc<vfs::opened_file::OpenedFile>> {
		proc!(kernel_ops()).get_open_file_by_fid(fd)
	}

	pub fn set_state(new_state: ProcessState) {
		proc!(kernel_ops()).set_state(new_state)
	}

	pub fn has_pending_signals() -> bool {
		proc!(kernel_ops()).has_pending_signals()
	}

	pub fn resume() {
		proc!(kernel_ops()).resume()
	}

	pub fn pid() -> Pid {
		proc!(kernel_ops()).pid()
	}
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

/// Prints and returns the value of a given expression for quick and dirty
/// debugging.
///
/// An example:
///
/// ```rust
/// let a = 2;
/// let b = dbg!(a * 2) + 1;
/// //      ^-- prints: [src/main.rs:2] a * 2 = 4
/// assert_eq!(b, 5);
/// ```
///
/// The macro works by using the `Debug` implementation of the type of
/// the given expression to print the value to [stderr] along with the
/// source location of the macro invocation as well as the source code
/// of the expression.
///
/// Invoking the macro on an expression moves and takes ownership of it
/// before returning the evaluated expression unchanged. If the type
/// of the expression does not implement `Copy` and you don't want
/// to give up ownership, you can instead borrow with `dbg!(&expr)`
/// for some expression `expr`.
///
/// The `dbg!` macro works exactly the same in release builds.
/// This is useful when debugging issues that only occur in release
/// builds or when debugging in release mode is significantly faster.
///
/// Note that the macro is intended as a debugging tool and therefore you
/// should avoid having uses of it in version control for long periods
/// (other than in tests and similar).
/// Debug output from production code is better done with other facilities
/// such as the [`debug!`] macro from the [`log`] crate.
///
/// # Stability
///
/// The exact output printed by this macro should not be relied upon
/// and is subject to future changes.
///
/// # Panics
///
/// Panics if writing to `io::stderr` fails.
///
/// # Further examples
///
/// With a method call:
///
/// ```rust
/// fn foo(n: usize) {
/// 	if let Some(_) = dbg!(n.checked_sub(4)) {
/// 		// ...
/// 	}
/// }
///
/// foo(3)
/// ```
///
/// This prints to [stderr]:
///
/// ```text,ignore
/// [src/main.rs:4] n.checked_sub(4) = None
/// ```
///
/// Naive factorial implementation:
///
/// ```rust
/// fn factorial(n: u32) -> u32 {
/// 	if dbg!(n <= 1) {
/// 		dbg!(1)
/// 	} else {
/// 		dbg!(n * factorial(n - 1))
/// 	}
/// }
///
/// dbg!(factorial(4));
/// ```
///
/// This prints to [stderr]:
///
/// ```text,ignore
/// [src/main.rs:3] n <= 1 = false
/// [src/main.rs:3] n <= 1 = false
/// [src/main.rs:3] n <= 1 = false
/// [src/main.rs:3] n <= 1 = true
/// [src/main.rs:4] 1 = 1
/// [src/main.rs:5] n * factorial(n - 1) = 2
/// [src/main.rs:5] n * factorial(n - 1) = 6
/// [src/main.rs:5] n * factorial(n - 1) = 24
/// [src/main.rs:11] factorial(4) = 24
/// ```
///
/// The `dbg!(..)` macro moves the input:
///
/// ```compile_fail
/// /// A wrapper around `usize` which importantly is not Copyable.
/// #[derive(Debug)]
/// struct NoCopy(usize);
///
/// let a = NoCopy(42);
/// let _ = dbg!(a); // <-- `a` is moved here.
/// let _ = dbg!(a); // <-- `a` is moved again; error!
/// ```
///
/// You can also use `dbg!()` without a value to just print the
/// file and line whenever it's reached.
///
/// Finally, if you want to `dbg!(..)` multiple values, it will treat them as
/// a tuple (and return it, too):
///
/// ```
/// assert_eq!(dbg!(1usize, 2u32), (1, 2));
/// ```
///
/// However, a single argument with a trailing comma will still not be treated
/// as a tuple, following the convention of ignoring trailing commas in macro
/// invocations. You can use a 1-tuple directly if you need one:
///
/// ```
/// assert_eq!(1, dbg!(1u32,)); // trailing comma ignored
/// assert_eq!((1,), dbg!((1u32,))); // 1-tuple
/// ```
///
/// [stderr]: https://en.wikipedia.org/wiki/Standard_streams#Standard_error_(stderr)
/// [`debug!`]: https://docs.rs/log/*/log/macro.debug.html
/// [`log`]: https://crates.io/crates/log
#[macro_export]
macro_rules! dbg {
    // NOTE: We cannot use `concat!` to make a static string as a format argument
    // of `eprintln!` because `file!` could contain a `{` or
    // `$val` expression could be a block (`{ .. }`), in which case the `eprintln!`
    // will be malformed.
    () => {
        $crate::info!("[{}:{}]", $crate::file!(), $crate::line!())
    };
    ($val:expr $(,)?) => {
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {
              use $crate::owo_colors::OwoColorize;
              $crate::info!("\x1b[0m[{}:{}] {} = {:#?}",
                    core::file!().yellow(), core::line!().yellow(), core::stringify!($val).green(), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::dbg!($val)),+,)
    };
}

pub mod ctypes;
pub mod driver;
pub mod guid;
pub mod io;
pub mod kernel;
pub mod posix;
pub mod process;
pub mod result;
pub mod schema;
pub mod user_buffer;
pub mod uuid;
pub mod vfs;
