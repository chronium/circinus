use alloc::sync::Arc;
use atomic_refcell::AtomicRef;
use environment::spinlock::{SpinLock, SpinLockGuard};

use crate::{
  cmdline::Cmdline,
  ctypes::c_int,
  vfs::{
    mount::Rootfs,
    opened_file::{OpenedFile, OpenedFileTable},
    Fd,
  },
  Result,
};

/// Process states.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ProcessState {
  /// The process is runnable.
  Runnable,
  /// The process is sleeping. It can be resumed by signals.
  BlockedSignalable,
  /// The process has exited.
  Exited(c_int),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Pid(i32);

impl Pid {
  pub const fn new(pid: i32) -> Self {
    Self(pid)
  }

  pub const fn as_i32(self) -> i32 {
    self.0
  }
}

pub trait ProcessOps {
  fn rootfs(&self) -> &Arc<SpinLock<Rootfs>>;
  fn exit(&self, status: c_int) -> !;
  fn opened_files(&self) -> Arc<SpinLock<OpenedFileTable>>;
  fn set_state(&self, new_state: ProcessState);
  fn has_pending_signals(&self) -> bool;
  fn resume(&self);
  fn pid(&self) -> Pid;
  fn cmdline(&self) -> AtomicRef<'_, Cmdline>;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PgId(i32);

impl PgId {
  pub const fn new(pgid: i32) -> PgId {
    PgId(pgid)
  }

  pub const fn as_i32(self) -> i32 {
    self.0
  }
}
