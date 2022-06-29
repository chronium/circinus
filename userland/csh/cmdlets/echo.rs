use alloc::{string::String, vec::Vec};

pub fn run(args: Vec<String>) -> crate::Result<()> {
	println!("{}", args.join(" "));
	Ok(())
}
