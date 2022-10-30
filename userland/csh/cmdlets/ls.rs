use alloc::{string::String, vec::Vec};
use base::io::{O_RDONLY, OpenFlags, FileMode};
use circinus::{fs::{path::Path, stat::Stat}, sys};

pub fn run(args: Vec<String>) -> crate::Result<()> {
	if args.len() < 1 {
		return Err(crate::ErrorKind::InvalidArgument);
	}

	let path = Path::new(args[0].as_str());

	if !path.is_dir() {
		return Err(crate::ErrorKind::NotADirectory);
	}

	let fd = sys::open(path, OpenFlags::O_DIRECTORY, FileMode::new(O_RDONLY));
	println!("fd: {:?}", fd);

	for arg in args {
		let stat = Stat::new(&Path::new(arg.as_str()));
		println!("{}: {:?}", arg, stat);
	}

	Err(crate::ErrorKind::Unimplemented)
}
