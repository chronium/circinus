use alloc::{string::String, vec::Vec};

pub fn execute(mut args: Vec<String>) -> crate::Result<()> {
	let cmd = args.remove(0);
	match cmd.as_str() {
		"ls" => ls::run(args),
		"echo" => echo::run(args),
		"fb" => fb::run(args),
		_ => Err(crate::ErrorKind::UnknownCommand),
	}
}

mod echo;
pub mod fb;
mod ls;
