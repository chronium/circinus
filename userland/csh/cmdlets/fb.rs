use alloc::{slice, string::String, vec, vec::Vec};
use base::io::{FileMode, OpenFlags, O_RDONLY};
use circinus::{
	fs::{path::Path, stat::Stat, Fd},
	sys::{open, read, read_obj, write, write_vec},
};

#[derive(Debug, Clone, Copy)]
struct FramebufferInfo {
	pub width: usize,
	pub height: usize,
}

impl FramebufferInfo {
	pub fn new() -> FramebufferInfo {
		FramebufferInfo {
			width: 0,
			height: 0,
		}
	}

	pub fn read(fd: Fd) -> FramebufferInfo {
		let mut info = FramebufferInfo::new();
		read_obj(fd, &mut info);
		info
	}
}

pub fn run(args: Vec<String>) -> Result<(), crate::ErrorKind> {
	let fb = open(
		Path::new("/Devices/Framebuffer"),
		OpenFlags::O_APPEND,
		FileMode::new(O_RDONLY),
	);

	let fb_info = FramebufferInfo::read(fb);

	println!("fb: {:?}", fb);
	println!("info: {:?}", fb_info);

	let mut buffer = vec![0xFFFFFFFFu32; fb_info.width * fb_info.height];

	for y in 0..fb_info.height as u32 {
		for x in 0..fb_info.width as u32 {
			let i = (y * fb_info.width as u32 + x) as usize;
			buffer[i] = 0xFF000000
				| (x * 0xFF / fb_info.width as u32) << 16
				| (y * 0xFF / fb_info.height as u32) << 8;
		}
	}

	write_vec(fb.as_int(), &buffer);

	Ok(())
}
