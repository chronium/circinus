use crate::{address::UserVAddr, system, x64::mouse::PS2MOUSE_IRQ};

use core::fmt;

use super::{
  apic::ack_interrupt, ioapic::VECTOR_IRQ_BASE, pc8042::PS2KBD_IRQ, serial::SERIAL0_IRQ,
  PageFaultReason,
};
use x86::{
  controlregs::cr2,
  current::rflags::{self, RFlags},
  irq::*,
};

/// The interrupt stack frame.
#[derive(Copy, Clone)]
#[repr(C, packed)]
struct InterruptFrame {
  rax: u64,
  rbx: u64,
  rcx: u64,
  rdx: u64,
  rsi: u64,
  rbp: u64,
  r8: u64,
  r9: u64,
  r10: u64,
  r11: u64,
  r12: u64,
  r13: u64,
  r14: u64,
  r15: u64,
  rdi: u64,
  error: u64,
  rip: u64,
  cs: u64,
  rflags: u64,
  rsp: u64,
  ss: u64,
}

impl fmt::Debug for InterruptFrame {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let rip = self.rip;
    let rsp = self.rsp;
    let cs = self.cs;
    let error = self.error;
    write!(f, "RIP={rip:x}, RSP={rsp:x}, CS={cs:x}, ERR={error:x}")
  }
}

extern "C" {
  fn usercopy1();
  fn usercopy2();
  fn usercopy3();
}

#[no_mangle]
unsafe extern "C" fn x64_handle_interrupt(vec: u8, frame: *const InterruptFrame) {
  let frame = &*frame;

  // FIXME: Check "Legacy replacement" mapping
  const TIMER_IRQ: u8 = 0;
  const TIMER_IRQ2: u8 = 2;
  if vec != VECTOR_IRQ_BASE + TIMER_IRQ
    && vec != VECTOR_IRQ_BASE + TIMER_IRQ2
    && vec != 14
    && vec != 36
  {
    let rip = frame.rip;
    let rsp = frame.rsp;
    let error = frame.error;
    let cr2 = cr2();
    trace!("interrupt({vec}): rip={rip:x}, rsp={rsp:x}, err={error:x}, cr2={cr2:x}",);
  }

  match vec {
    _ if vec >= VECTOR_IRQ_BASE => {
      ack_interrupt();

      let irq = vec - VECTOR_IRQ_BASE;
      match irq {
        TIMER_IRQ | TIMER_IRQ2 => {
          system().on_timer_irq();
        }
        PS2KBD_IRQ => {
          super::pc8042::ps2kbd_irq_handler();
        }
        PS2MOUSE_IRQ => {
          super::mouse::ps2mouse_irq_handler();
        }
        SERIAL0_IRQ => {
          super::serial::serial0_irq_handler();
        }
        _ => {
          system().on_irq(irq);
        }
      }
    }
    DIVIDE_ERROR_VECTOR => {
      // TODO:
      panic!("unsupported exception: DIVIDE_ERROR\n{:?}", frame);
    }
    DEBUG_VECTOR => {
      // TODO:
      panic!("unsupported exception: DEBUG\n{:?}", frame);
    }
    NONMASKABLE_INTERRUPT_VECTOR => {
      // TODO:
      panic!("unsupported exception: NONMASKABLE_INTERRUPT\n{:?}", frame);
    }
    BREAKPOINT_VECTOR => {
      // TODO:
      panic!("unsupported exception: BREAKPOINT\n{:?}", frame);
    }
    OVERFLOW_VECTOR => {
      // TODO:
      panic!("unsupported exception: OVERFLOW\n{:?}", frame);
    }
    BOUND_RANGE_EXCEEDED_VECTOR => {
      // TODO:
      panic!("unsupported exception: BOUND_RANGE_EXCEEDED\n{:?}", frame);
    }
    INVALID_OPCODE_VECTOR => {
      // TODO:
      panic!("unsupported exception: INVALID_OPCODE\n{:?}", frame);
    }
    DEVICE_NOT_AVAILABLE_VECTOR => {
      // TODO:
      panic!("unsupported exception: DEVICE_NOT_AVAILABLE\n{:?}", frame);
    }
    DOUBLE_FAULT_VECTOR => {
      // TODO:
      panic!("unsupported exception: DOUBLE_FAULT\n{:?}", frame);
    }
    COPROCESSOR_SEGMENT_OVERRUN_VECTOR => {
      // TODO:
      panic!(
        "unsupported exception: COPROCESSOR_SEGMENT_OVERRUN\n{:?}",
        frame
      );
    }
    INVALID_TSS_VECTOR => {
      // TODO:
      panic!("unsupported exception: INVALID_TSS\n{:?}", frame);
    }
    SEGMENT_NOT_PRESENT_VECTOR => {
      // TODO:
      panic!("unsupported exception: SEGMENT_NOT_PRESENT\n{:?}", frame);
    }
    STACK_SEGEMENT_FAULT_VECTOR => {
      // TODO:
      panic!("unsupported exception: STACK_SEGEMENT_FAULT\n{:?}", frame);
    }
    GENERAL_PROTECTION_FAULT_VECTOR => {
      // TODO:
      panic!(
        "unsupported exception: GENERAL_PROTECTION_FAULT\n{:?}",
        frame
      );
    }
    PAGE_FAULT_VECTOR => {
      let reason = PageFaultReason::from_bits_truncate(frame.error as u32);

      // Panic if it's occurred in the kernel space.
      let occurred_in_user = reason.contains(PageFaultReason::CAUSED_BY_USER)
        || frame.rip == usercopy1 as *const u8 as u64
        || frame.rip == usercopy2 as *const u8 as u64
        || frame.rip == usercopy3 as *const u8 as u64;
      if !occurred_in_user {
        let rip = frame.rip;
        let rsp = frame.rsp;
        let cr2 = cr2();
        panic!("page fault occurred in the kernel: rip={rip:x}, rsp={rsp:x}, vaddr={cr2:x}",);
      }

      // Abort if the virtual address points to out of the user's address space.
      let unaligned_vaddr = UserVAddr::new(cr2() as usize);
      system().on_page_fault(unaligned_vaddr, frame.rip as usize, reason);
    }
    X87_FPU_VECTOR => {
      // TODO:
      panic!("unsupported exception: X87_FPU\n{:?}", frame);
    }
    ALIGNMENT_CHECK_VECTOR => {
      // TODO:
      panic!("unsupported exception: ALIGNMENT_CHECK\n{:?}", frame);
    }
    MACHINE_CHECK_VECTOR => {
      // TODO:
      panic!("unsupported exception: MACHINE_CHECK\n{:?}", frame);
    }
    SIMD_FLOATING_POINT_VECTOR => {
      // TODO:
      panic!("unsupported exception: SIMD_FLOATING_POINT\n{:?}", frame);
    }
    VIRTUALIZATION_VECTOR => {
      // TODO:
      panic!("unsupported exception: VIRTUALIZATION\n{:?}", frame);
    }
    _ => {
      panic!("unexpected interrupt: vec={}", vec);
    }
  }
}

pub struct SavedInterruptStatus {
  rflags: RFlags,
}

impl SavedInterruptStatus {
  pub fn save() -> SavedInterruptStatus {
    SavedInterruptStatus {
      rflags: rflags::read(),
    }
  }
}

impl Drop for SavedInterruptStatus {
  fn drop(&mut self) {
    rflags::set(rflags::read() | (self.rflags & rflags::RFlags::FLAGS_IF));
  }
}
