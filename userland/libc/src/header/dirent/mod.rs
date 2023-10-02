use core::{mem, ptr};

use alloc::boxed::Box;

use crate::{
  c_str::CStr,
  file::File,
  platform::{sys::Sys, types::*},
};

use super::fcntl;

const DIR_BUF_SIZE: usize = mem::size_of::<dirent>() * 3;

// No repr(C) needed, C won't see the content
pub struct DIR {
  file: File,
  buf: [c_char; DIR_BUF_SIZE],
  // index and len are specified in bytes
  index: usize,
  len: usize,

  // The last value of d_off, used by telldir
  offset: usize,
}

#[repr(C)]
#[derive(Clone)]
pub struct dirent {
  pub d_ino: ino_t,
  pub d_off: off_t,
  pub d_reclen: c_ushort,
  pub d_type: c_uchar,
  pub d_name: [c_char; 256],
}

#[no_mangle]
pub unsafe extern "C" fn opendir(path: *const c_char) -> *mut DIR {
  let path = CStr::from_ptr(path);
  let file = match File::open(
    path,
    fcntl::O_RDONLY | fcntl::O_DIRECTORY | fcntl::O_CLOEXEC,
  ) {
    Ok(file) => file,
    Err(_) => return ptr::null_mut(),
  };

  Box::into_raw(Box::new(DIR {
    file,
    buf: [0; DIR_BUF_SIZE],
    index: 0,
    len: 0,
    offset: 0,
  }))
}

#[no_mangle]
pub unsafe extern "C" fn closedir(dir: *mut DIR) -> c_int {
  let mut dir = Box::from_raw(dir);

  let ret = Sys::close(*dir.file);

  dir.file.reference = true;

  ret
}

#[no_mangle]
pub unsafe extern "C" fn readdir(dir: *mut DIR) -> *mut dirent {
  if (*dir).index >= (*dir).len {
    let read = Sys::getdents(
      *(*dir).file,
      (*dir).buf.as_mut_ptr() as *mut dirent,
      (*dir).buf.len(),
    );
    if read <= 0 {
      // TODO: errno
      return ptr::null_mut();
    }
    (*dir).index = 0;
    (*dir).len = read as usize;
  }

  let ptr = (*dir).buf.as_mut_ptr().add((*dir).index) as *mut dirent;

  (*dir).index += (*ptr).d_reclen as usize;
  (*dir).offset = (*ptr).d_off as usize;
  ptr
}
