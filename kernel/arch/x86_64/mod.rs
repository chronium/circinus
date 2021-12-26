use environment::arch::PAGE_SIZE;

global_asm!(include_str!("usermode.S"));

pub const KERNEL_STACK_SIZE: usize = PAGE_SIZE * 16;

mod process;

pub use process::{switch_thread, Process};
