use core::arch::global_asm;

use environment::{address::UserVAddr, arch::PAGE_SIZE};

global_asm!(include_str!("usermode.S"));

pub const KERNEL_STACK_SIZE: usize = PAGE_SIZE * 256;
pub const USER_VALLOC_END: UserVAddr = unsafe { UserVAddr::new_unchecked(0x0000_0fff_0000_0000) };
pub const USER_VALLOC_BASE: UserVAddr = unsafe { UserVAddr::new_unchecked(0x0000_000a_0000_0000) };
pub const USER_STACK_TOP: UserVAddr = USER_VALLOC_BASE;

mod process;

pub use process::{switch_thread, Process};
