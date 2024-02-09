use core::arch::global_asm;

global_asm!(include_str!("boot.S"));
global_asm!(include_str!("trap.S"));
global_asm!(include_str!("usercopy.S"));

#[macro_use]
pub mod cpu_local;

pub mod apic;
pub mod backtrace;
pub mod boot;
pub mod bootinfo;
pub mod gdt;
pub mod idle;
pub mod idt;
pub mod interrupt;
pub mod ioapic;
pub mod mouse;
pub mod paging;
pub mod pc8042;
pub mod pit;
pub mod profile;
pub mod serial;
pub mod syscall;
pub mod tss;
pub mod vga;

pub use paging::PageFaultReason;

pub const PAGE_SIZE: usize = 4096;
pub const TICK_HZ: usize = 1000;

/// The base virtual address of straight mapping.
pub const KERNEL_BASE_ADDR: usize = 0xffff_8000_0000_0000;

/// The end of straight mapping. Any physical address `P` is mapped into the
/// kernel's virtual memory address `KERNEL_BASE_ADDR + P`.
pub const KERNEL_STRAIGHT_MAP_PADDR_END: usize = 0x1_0000_0000;
