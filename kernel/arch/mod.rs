#[cfg(target_arch = "x86_64")]
mod x64;

#[cfg(target_arch = "x86_64")]
pub use self::x64::*;
