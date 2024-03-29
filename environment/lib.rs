#![no_std]

#[allow(unused_imports)]
#[macro_use]
extern crate alloc;

#[macro_use]
extern crate log;
use address::UserVAddr;
use ps2_mouse::MouseState;
use utils::static_cell::StaticCell;

#[macro_use]
pub mod print;

pub trait System: Sync {
  fn on_console_rx(&self, char: u8);
  #[cfg(target_arch = "x86_64")]
  fn on_mouse_event(&self, mouse_state: MouseState);
  fn on_irq(&self, irq: u8);
  fn on_timer_irq(&self);
  fn on_page_fault(
    &self,
    unaligned_vaddr: Option<UserVAddr>,
    ip: usize,
    _reason: arch::PageFaultReason,
  );

  #[allow(clippy::too_many_arguments)]
  fn on_syscall(
    &self,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    a6: usize,
    n: usize,
    frame: *mut arch::PtRegs,
  ) -> isize;

  #[cfg(debug_assertions)]
  fn usercopy_hook(&self);
}

static SYSTEM: StaticCell<&dyn System> = StaticCell::new(&NopSystem);
struct NopSystem;

impl System for NopSystem {
  fn on_console_rx(&self, _char: u8) {}

  #[cfg(target_arch = "x86_64")]
  fn on_mouse_event(&self, _mouse_state: MouseState) {}

  fn on_irq(&self, _irq: u8) {}

  fn on_timer_irq(&self) {}

  fn on_page_fault(
    &self,
    _unaligned_vaddr: Option<UserVAddr>,
    _ip: usize,
    _reason: arch::PageFaultReason,
  ) {
  }

  fn on_syscall(
    &self,
    _a1: usize,
    _a2: usize,
    _a3: usize,
    _a4: usize,
    _a5: usize,
    _a6: usize,
    _n: usize,
    _frame: *mut arch::PtRegs,
  ) -> isize {
    0
  }

  #[cfg(debug_assertions)]
  fn usercopy_hook(&self) {}
}

fn system() -> &'static dyn System {
  SYSTEM.load()
}

pub fn set_system(system: &'static dyn System) {
  SYSTEM.store(system);
}

mod x64;

pub mod arch {
  pub use crate::x64::{
    backtrace::Backtrace,
    cpu_local::cpu_local_head,
    idle::{halt, idle},
    interrupt::SavedInterruptStatus,
    ioapic::enable_irq,
    paging::PageTable,
    profile::read_clock_counter,
    serial::SERIAL0,
    syscall::PtRegs,
    tss::TSS,
    PageFaultReason, KERNEL_BASE_ADDR, KERNEL_STRAIGHT_MAP_PADDR_END, PAGE_SIZE, TICK_HZ,
  };

  pub mod x64 {
    pub use crate::x64::gdt::{USER_CS64, USER_DS, USER_RPL};
  }
}

pub mod address;
pub mod backtrace;
pub mod bootinfo;
pub mod global_allocator;
pub mod logger;
pub mod page_allocator;
pub mod profile;
pub mod spinlock;
