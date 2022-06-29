#![no_std]
#![feature(start)]
#![feature(panic_info_message)]
#![feature(allocator_api)]
#![feature(default_alloc_error_handler)]

use alloc::{string::String, vec};
use base::io::Read;
use cstr_core::CStr;

use crate::parse::Parser;

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
		let mut ins = String::new();
		print!("> ");

		let mut buf = [b'\0'; 256];
		let len = circinus::io::stdin().read(&mut buf).unwrap();
		if len > 0 {
			let input = unsafe { CStr::from_bytes_with_nul_unchecked(&buf) }.to_string_lossy();
			print!("input: {}", input);

			input.chars().for_each(|c| {
				if c != '\0' {
					ins.push(c);
				}
			});

			let mut parser = Parser::new(ins);
			let args = parser.parse().unwrap();
			println!("{:?}", args);
			println!("{:?}", cmdlets::execute(args));
		}
	}
}

#[macro_use]
extern crate circinus;

pub mod cmdlets;
pub mod parse;
pub mod result;

pub use result::{ErrorKind, Result};
