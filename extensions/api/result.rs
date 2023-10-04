use core::fmt;

use environment::{
  address::{AccessError, NullUserPointerError},
  backtrace::CapturedBacktrace,
};

use crate::mm::PageAllocError;

#[derive(Debug, Clone, Copy)]
pub enum ErrorKind {
  AllocationError = 1,
  PidAllocFailed,
  PageFault,
  OutOfMemory,

  NotSupported,

  Invalid,

  TooBig,
  TooShort,

  NotExecutable,
  NoSyscall,
  Interrupted,

  NotFound,
  NotADirectory,
  NotAFile,
  NoEntry,
  Unsupported,

  IsADirectory,

  BadFile,

  BufferError,
}

pub type Result<T> = ::core::result::Result<T, Error>;

enum ErrorMessage {
  StaticStr(&'static str),
}

pub struct Error {
  kind: ErrorKind,
  message: Option<ErrorMessage>,
  #[cfg(debug_assertions)]
  backtrace: Option<CapturedBacktrace>,
}

impl Error {
  pub fn new(kind: ErrorKind) -> Error {
    Error {
      kind,
      message: None,
      #[cfg(debug_assertions)]
      backtrace: Some(CapturedBacktrace::capture()),
    }
  }

  pub fn with_message(kind: ErrorKind, message: &'static str) -> Error {
    Error {
      kind,
      message: Some(ErrorMessage::StaticStr(message)),
      #[cfg(debug_assertions)]
      backtrace: Some(CapturedBacktrace::capture()),
    }
  }

  pub const fn with_message_const(kind: ErrorKind, message: &'static str) -> Error {
    Error {
      kind,
      message: Some(ErrorMessage::StaticStr(message)),
      #[cfg(debug_assertions)]
      backtrace: None,
    }
  }

  pub fn kind(&self) -> ErrorKind {
    self.kind
  }

  pub fn errno(&self) -> usize {
    -(self.kind as isize) as usize
  }
}

impl fmt::Debug for Error {
  #[cfg(not(debug_assertions))]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if let Some(message) = self.message.as_ref() {
      match message {
        ErrorMessage::StaticStr(message) => {
          write!(f, "[{:?}] {}", self.kind, message)
        }
      }
    } else {
      write!(f, "{:?}", self.kind)
    }
  }

  #[cfg(debug_assertions)]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if let Some(message) = self.message.as_ref() {
      match message {
        ErrorMessage::StaticStr(message) => {
          if let Some(ref trace) = self.backtrace {
            write!(
              f,
              "[{:?}] {}\n    This error originates from:\n{:?}",
              self.kind, message, trace
            )
          } else {
            write!(f, "[{:?}] {}", self.kind, message)
          }
        }
      }
    } else if let Some(ref trace) = self.backtrace {
      write!(
        f,
        "{:?}: This error originates from:\n{:?}",
        self.kind, trace
      )
    } else {
      write!(f, "{:?}", self.kind)
    }
  }
}

impl From<PageAllocError> for Error {
  fn from(_error: PageAllocError) -> Error {
    Error::new(ErrorKind::AllocationError)
  }
}

impl From<ErrorKind> for Error {
  fn from(error: ErrorKind) -> Error {
    Error::new(error)
  }
}

impl From<AccessError> for Error {
  fn from(_error: AccessError) -> Error {
    Error::new(ErrorKind::PageFault)
  }
}

impl From<NullUserPointerError> for Error {
  fn from(_error: NullUserPointerError) -> Error {
    Error::new(ErrorKind::PageFault)
  }
}
