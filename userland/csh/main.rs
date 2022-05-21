#![no_std]
#![feature(start)]
#![feature(panic_info_message)]
#![feature(allocator_api)]
#![feature(default_alloc_error_handler)]

use alloc::{string::String, vec};
use base::io::Read;
use cstr_core::CStr;

extern crate alloc;

const SEASHELL: &'static str = r#"CSH (pronounced sea shell)
     _.---._
 .'"".'/|\`.""'.
:  .' / | \ `.  :
'.'  /  |  \  `.'
 `. /   |   \ .'
   `-.__|__.-'"#;

start!();

fn main() {
	println!("{}", SEASHELL);

	let mut v = vec![1024, 512, 256];
	println!("{:?}", v);
	v.push(10);
	v.push(11);
	println!("{:?}", v);

	loop {
		print!("> ");

		let mut buf = [b'\0'; 256];
		let len = circinus::io::stdin().read(&mut buf).unwrap();
		if len > 0 {
			print!(
				"echo: {}",
				unsafe { CStr::from_bytes_with_nul_unchecked(&buf) }.to_string_lossy()
			);
		}
	}
}

#[macro_use]
extern crate circinus;
