use crate::{vfs, ctypes::c_int};

#[derive(Debug)]
#[repr(transparent)]
pub struct Timestamp(pub u32);

pub enum CwdOrFd {
    /// `AT_FDCWD`
    AtCwd,
    Fd(vfs::Fd),
}

impl CwdOrFd {
    pub fn parse(value: c_int) -> CwdOrFd {
        match value {
            -100 => CwdOrFd::AtCwd,
            _ => CwdOrFd::Fd(vfs::Fd::new(value)),
        }
    }
}

