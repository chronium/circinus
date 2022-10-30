#![no_std]
#![feature(linkage)]

use core::arch::global_asm;

#[cfg(target_arch = "x86_64")]
global_asm!(include_str!("x86_64.S"));

#[linkage = "weak"]
#[no_mangle]
extern "C" fn cilibc_panic(_pi: &::core::panic::PanicInfo) -> ! {
	loop {}
}

#[panic_handler]
#[linkage = "weak"]
#[no_mangle]
pub unsafe extern "C" fn rust_begin_unwind(pi: &::core::panic::PanicInfo) -> ! {
	cilibc_panic(pi)
}
