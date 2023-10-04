#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![feature(linkage)]
#![feature(allocator_api)]
#![feature(alloc_error_handler)]
#![feature(c_variadic)]
#![feature(alloc_layout_extra)]
#![feature(lang_items)]
#![feature(thread_local)]

use crate::platform::sys::Sys;

#[macro_use]
extern crate alloc;

#[macro_use]
mod macros;
mod allocator;
pub mod c_str;
pub mod c_vec;
pub mod file;
pub mod header;
pub mod io;
pub mod platform;
pub mod start;

#[no_mangle]
pub extern "C" fn cilibc_panic(pi: &::core::panic::PanicInfo) -> ! {
  use core::fmt::Write;

  let mut w = platform::FileWriter(2);
  let _ = w.write_fmt(format_args!("CILIBC PANIC: {}\n", pi));

  Sys::exit(1)
}

#[cfg(not(test))]
#[alloc_error_handler]
#[linkage = "weak"]
#[no_mangle]
pub extern "C" fn rust_oom(layout: ::core::alloc::Layout) -> ! {
  use core::fmt::Write;

  let mut w = platform::FileWriter(2);
  let _ = w.write_fmt(format_args!(
    "CILIBC OOM: {} bytes aligned to {} bytes\n",
    layout.size(),
    layout.align()
  ));

  Sys::exit(1);
}

#[cfg(not(test))]
#[panic_handler]
#[linkage = "weak"]
#[no_mangle]
pub extern "C" fn rust_begin_unwind(pi: &::core::panic::PanicInfo) -> ! {
  cilibc_panic(pi)
}

#[cfg(not(test))]
#[lang = "eh_personality"]
#[no_mangle]
#[linkage = "weak"]
pub extern "C" fn rust_eh_personality() {}
