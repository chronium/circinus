use alloc::sync::Arc;
use api::schema::{posix, unix};
use hashbrown::HashMap;
use utils::{byte_size::ByteSize, bytes_parser::BytesParser};

use crate::schema::system::cpio::{CpioDir, CpioFile, CpioSymlink};

use super::{CpioArchive, CpioNode};

impl<'a> From<&'a [u8]> for CpioArchive<'a> {
	fn from(data: &'a [u8]) -> Self {
		let mut archive = BytesParser::new(data);
		let mut root_files = HashMap::new();
		let mut num_files = 0;
		let mut loaded_size = 0;

		loop {
			let magic = consume_hex!(archive, 6);
			if magic != 0x070701 {
				panic!(
					"cpio: invalid magic (expected {:x}, got {:x})",
					0x070701, magic
				);
			}

			let ino = consume_hex!(archive, 8);
			let mode = posix::FileMode::new(consume_hex!(archive, 8) as u32);
			let _uid = consume_hex!(archive, 8);
			let _gid = consume_hex!(archive, 8);
			let _nlink = consume_hex!(archive, 8);
			let _mtime = consume_hex!(archive, 8);
			let filesize = consume_hex!(archive, 8);
			let _dev_major = consume_hex!(archive, 8);
			let _dev_minor = consume_hex!(archive, 8);

			let _c_rmaj = consume_hex!(archive, 8);
			let _c_rmin = consume_hex!(archive, 8);

			let path_len = consume_hex!(archive, 8);
			assert!(path_len > 0);

			let _checksum = consume_hex!(archive, 8);

			let mut path = consume_str!(archive, path_len - 1);
			if path.starts_with("./") {
				path = &path[1..];
			}
			if path == "TRAILER!!!" {
				break;
			}

			assert!(!path.is_empty());
			trace!("cpio: \"{}\" ({})", path, ByteSize::new(filesize));
			archive.skip(1).unwrap();
			archive.skip_until_alignment(4).unwrap();

			let mut files = &mut root_files;
			let mut filename = None;
			let mut components = unix::Path::new(path).components().peekable();
			while let Some(comp) = components.next() {
				if components.peek().is_none() {
					filename = Some(comp);
					break;
				}

				match files.get_mut(comp) {
					Some(CpioNode::Directory(dir)) => {
						files = &mut Arc::get_mut(dir).unwrap().files;
					}
					Some(_) => {
						panic!(
							"cpio: invalid path '{}' ('{}' is not a directory)",
							path, comp
						);
					}
					None => {
						panic!(
							"cpio: invalid path '{}' ('{}' does not exist)",
							path, comp
						);
					}
				}
			}

			let data = archive.consume_bytes(filesize).unwrap();
			if mode.is_symbolic_link() {
				let filename = filename.unwrap();
				files.insert(
					filename,
					CpioNode::Symlink(Arc::new(CpioSymlink {
						filename,
						stat: posix::Stat {
							inode_no: posix::INodeNo::new(ino),
							mode,
							..posix::Stat::zeroed()
						},
						dst: unix::PathBuf::from(
							core::str::from_utf8(data).unwrap(),
						),
					})),
				);
			} else if mode.is_directory() {
				let filename = filename.unwrap();
				files.insert(
					filename,
					CpioNode::Directory(Arc::new(CpioDir {
						filename,
						files: HashMap::new(),
						stat: posix::Stat {
							inode_no: posix::INodeNo::new(ino),
							mode,
							..posix::Stat::zeroed()
						},
					})),
				);
			} else if mode.is_regular_file() {
				let filename = filename.unwrap();
				files.insert(
					filename,
					CpioNode::File(Arc::new(CpioFile {
						filename,
						data,
						stat: posix::Stat {
							inode_no: posix::INodeNo::new(ino),
							mode,
							size: posix::FileSize(filesize as isize),
							..posix::Stat::zeroed()
						},
					})),
				);
			}

			archive.skip_until_alignment(4).unwrap();
			num_files += 1;
			loaded_size += data.len();
		}

		info!(
			"cpio: loaded {} files and directories ({})",
			num_files,
			ByteSize::new(loaded_size)
		);

		CpioArchive {
			root_dir: Arc::new(CpioDir {
				filename: "",
				stat: posix::Stat {
					inode_no: posix::INodeNo::new(2),
					mode: posix::FileMode::new(posix::S_IFDIR | 0o755),
					..posix::Stat::zeroed()
				},
				files: root_files,
			}),
		}
	}
}
