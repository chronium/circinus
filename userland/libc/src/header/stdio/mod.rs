pub mod constants;
pub mod default;
mod helpers;
pub mod printf;

use alloc::{boxed::Box, vec::Vec};
use core::{
  borrow::{Borrow, BorrowMut},
  cmp,
  ffi::VaList as va_list,
  fmt::{self, Write as WriteFmt},
  ops::{Deref, DerefMut},
  ptr, slice, str,
};
use spin::{Mutex, MutexGuard};

use crate::{
  c_str::CStr,
  c_vec::CVec,
  file::File,
  io::{self, BufRead, BufWriter, LineWriter, Read, Write},
  platform::{self, errno, sys::Sys, types::*},
};

pub use self::{constants::*, default::*};

use super::{
  errno::{self, STR_ERROR},
  fcntl::{self, FD_CLOEXEC, O_CLOEXEC, O_CREAT},
  string::strlen,
};

enum Buffer<'a> {
  Borrowed(&'a mut [u8]),
  Owned(Vec<u8>),
}

impl<'a> Deref for Buffer<'a> {
  type Target = [u8];

  fn deref(&self) -> &Self::Target {
    match self {
      Buffer::Borrowed(inner) => inner,
      Buffer::Owned(inner) => inner.borrow(),
    }
  }
}

impl<'a> DerefMut for Buffer<'a> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    match self {
      Buffer::Borrowed(inner) => inner,
      Buffer::Owned(inner) => inner.borrow_mut(),
    }
  }
}

pub trait Pending {
  fn pending(&self) -> size_t;
}

impl<W: core_io::Write> Pending for BufWriter<W> {
  fn pending(&self) -> size_t {
    self.buf.len() as size_t
  }
}

impl<W: core_io::Write> Pending for LineWriter<W> {
  fn pending(&self) -> size_t {
    self.inner.buf.len() as size_t
  }
}

pub trait Writer: Write + Pending {}

impl<W: core_io::Write> Writer for BufWriter<W> {}
impl<W: core_io::Write> Writer for LineWriter<W> {}

pub struct FILE {
  lock: Mutex<()>,

  file: File,
  pub(crate) flags: c_int,
  read_buf: Buffer<'static>,
  read_pos: usize,
  read_size: usize,
  unget: Vec<u8>,
  pub(crate) writer: Box<dyn Writer + Send>,

  pid: Option<c_int>,

  pub(crate) orientation: c_int,
}

impl Read for FILE {
  fn read(&mut self, out: &mut [u8]) -> io::Result<usize> {
    let unget_read_size = cmp::min(out.len(), self.unget.len());
    for i in 0..unget_read_size {
      out[i] = self.unget.pop().unwrap();
    }
    if unget_read_size != 0 {
      return Ok(unget_read_size);
    }

    let len = {
      let buf = self.fill_buf()?;
      let len = buf.len().min(out.len());

      out[..len].copy_from_slice(&buf[..len]);
      len
    };
    self.consume(len);
    Ok(len)
  }
}

impl BufRead for FILE {
  fn fill_buf(&mut self) -> io::Result<&[u8]> {
    if self.read_pos == self.read_size {
      self.read_size = match self.file.read(&mut self.read_buf) {
        Ok(0) => {
          self.flags |= F_EOF;
          0
        }
        Ok(n) => n,
        Err(err) => {
          self.flags |= F_ERR;
          return Err(err);
        }
      };
      self.read_pos = 0;
    }
    Ok(&self.read_buf[self.read_pos..self.read_size])
  }

  fn consume(&mut self, i: usize) {
    self.read_pos = (self.read_pos + i).min(self.read_size);
  }
}

impl Write for FILE {
  fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
    match self.writer.write(buf) {
      Ok(n) => Ok(n),
      Err(err) => {
        self.flags |= F_ERR;
        Err(err)
      }
    }
  }

  fn flush(&mut self) -> io::Result<()> {
    match self.writer.flush() {
      Ok(()) => Ok(()),
      Err(err) => {
        self.flags |= F_ERR;
        Err(err)
      }
    }
  }
}
impl WriteFmt for FILE {
  fn write_str(&mut self, s: &str) -> fmt::Result {
    self
      .write_all(s.as_bytes())
      .map(|_| ())
      .map_err(|_| fmt::Error)
  }
}

impl FILE {
  pub fn lock(&mut self) -> LockGuard {
    unsafe { flockfile(self) }
    LockGuard(self)
  }

  pub fn try_set_orientation(&mut self, mode: c_int) -> c_int {
    let stream = self.lock();
    stream.0.try_set_orientation_unlocked(mode)
  }

  pub fn try_set_orientation_unlocked(&mut self, mode: c_int) -> c_int {
    if self.orientation == 0 {
      self.orientation = match mode {
        1..=i32::MAX => 1,
        i32::MIN..=-1 => -1,
        0 => self.orientation,
      };
    }
    self.orientation
  }

  pub fn try_set_byte_orientation_unlocked(&mut self) -> core::result::Result<(), c_int> {
    match self.try_set_orientation_unlocked(-1) {
      i32::MIN..=-1 => Ok(()),
      x => Err(x),
    }
  }
}

#[no_mangle]
pub unsafe extern "C" fn flockfile(file: *mut FILE) {
  MutexGuard::leak((*file).lock.lock());
}

#[no_mangle]
pub unsafe extern "C" fn funlockfile(file: *mut FILE) {
  (*file).lock.force_unlock();
}

#[no_mangle]
pub unsafe extern "C" fn puts(s: *const c_char) -> c_int {
  let mut stream = (*stdout).lock();
  if (*stream).try_set_byte_orientation_unlocked().is_err() {
    return -1;
  }

  let buf = slice::from_raw_parts(s as *mut u8, strlen(s));

  if stream.write_all(buf).is_err() {
    return -1;
  }
  if stream.write(&[b'\n']).is_err() {
    return -1;
  }
  0
}

#[no_mangle]
pub unsafe extern "C" fn fflush(stream: *mut FILE) -> c_int {
  if stream.is_null() {
    if fflush(stdout) != 0 {
      return EOF;
    }

    if fflush(stderr) != 0 {
      return EOF;
    }
  } else {
    let mut stream = (*stream).lock();
    if stream.flush().is_err() {
      return EOF;
    }
  }

  0
}

#[no_mangle]
pub unsafe extern "C" fn fgetc(stream: *mut FILE) -> c_int {
  let mut stream = (*stream).lock();
  if (*stream).try_set_byte_orientation_unlocked().is_err() {
    return EOF;
  }

  getc_unlocked(&mut *stream)
}

#[no_mangle]
pub unsafe extern "C" fn getc(stream: *mut FILE) -> c_int {
  let mut stream = (*stream).lock();
  getc_unlocked(&mut *stream)
}

#[no_mangle]
pub unsafe extern "C" fn getchar() -> c_int {
  fgetc(&mut *stdin)
}

#[no_mangle]
pub unsafe extern "C" fn getc_unlocked(stream: *mut FILE) -> c_int {
  if (*stream).try_set_byte_orientation_unlocked().is_err() {
    return EOF;
  }

  let mut buf = [0];

  match (*stream).read(&mut buf) {
    Ok(0) | Err(_) => EOF,
    Ok(_) => buf[0] as c_int,
  }
}

pub struct LockGuard<'a>(&'a mut FILE);

impl<'a> Deref for LockGuard<'a> {
  type Target = FILE;

  fn deref(&self) -> &Self::Target {
    self.0
  }
}

impl<'a> DerefMut for LockGuard<'a> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    self.0
  }
}

impl<'a> Drop for LockGuard<'a> {
  fn drop(&mut self) {
    unsafe {
      funlockfile(self.0);
    }
  }
}

#[no_mangle]
pub unsafe extern "C" fn vfprintf(file: *mut FILE, format: *const c_char, ap: va_list) -> c_int {
  let mut file = (*file).lock();
  if file.try_set_byte_orientation_unlocked().is_err() {
    return -1;
  }

  printf::printf(&mut *file, format, ap)
}

#[no_mangle]
pub unsafe extern "C" fn vprintf(format: *const c_char, ap: va_list) -> c_int {
  vfprintf(&mut *stdout, format, ap)
}

#[no_mangle]
pub unsafe extern "C" fn vasprintf(
  strp: *mut *mut c_char,
  format: *const c_char,
  ap: va_list,
) -> c_int {
  let mut alloc_writer = CVec::new();
  let ret = printf::printf(&mut alloc_writer, format, ap);
  alloc_writer.push(0).unwrap();
  alloc_writer.shrink_to_fit().unwrap();
  *strp = alloc_writer.leak() as *mut c_char;
  ret
}

#[no_mangle]
pub unsafe extern "C" fn vsnprintf(
  s: *mut c_char,
  n: size_t,
  format: *const c_char,
  ap: va_list,
) -> c_int {
  printf::printf(
    &mut platform::StringWriter(s as *mut u8, n as usize),
    format,
    ap,
  )
}

#[no_mangle]
pub unsafe extern "C" fn vsprintf(s: *mut c_char, format: *const c_char, ap: va_list) -> c_int {
  printf::printf(&mut platform::UnsafeStringWriter(s as *mut u8), format, ap)
}

#[no_mangle]
pub unsafe extern "C" fn vfscanf(file: *mut FILE, format: *const c_char, ap: va_list) -> c_int {
  todo!()
}

#[no_mangle]
pub unsafe extern "C" fn vscanf(format: *const c_char, ap: va_list) -> c_int {
  todo!()
}

#[no_mangle]
pub unsafe extern "C" fn vsscanf(s: *const c_char, format: *const c_char, ap: va_list) -> c_int {
  todo!()
}

pub unsafe fn flush_io_streams() {
  let flush = |stream: *mut FILE| {
    let stream = &mut *stream;
    stream.flush()
  };
  let _ = flush(stdout);
  let _ = flush(stderr);
}

#[no_mangle]
pub unsafe extern "C" fn fclose(stream: *mut FILE) -> c_int {
  let stream = &mut *stream;
  flockfile(stream);

  let mut r = stream.flush().is_err();
  let close = Sys::close(*stream.file) < 0;
  r = r || close;

  if stream.flags & constants::F_PERM == 0 {
    let mut stream = Box::from_raw(stream);
    stream.file.reference = true;
  } else {
    funlockfile(stream);
  }

  r as c_int
}

#[no_mangle]
pub unsafe extern "C" fn fopen(filename: *const c_char, mode: *const c_char) -> *mut FILE {
  let initial_mode = *mode;
  if initial_mode != b'r' as i8 && initial_mode != b'w' as i8 && initial_mode != b'a' as i8 {
    platform::errno = errno::EINVAL;
    return core::ptr::null_mut();
  }

  let flags = helpers::parse_mode_flags(mode);

  let new_modew = if flags & fcntl::O_CREAT == O_CREAT {
    0o666
  } else {
    0
  };

  let fd = fcntl::sys_open(filename, flags, new_modew);
  if fd < 0 {
    return ptr::null_mut();
  }

  if flags & O_CLOEXEC > 0 {
    fcntl::sys_fcntl(fd, fcntl::F_SETFD, FD_CLOEXEC as c_ulonglong);
  }

  if let Some(f) = helpers::_fdopen(fd, mode) {
    f
  } else {
    Sys::close(fd);
    ptr::null_mut()
  }
}

#[no_mangle]
pub unsafe extern "C" fn perror(s: *const c_char) {
  let s_cstr = CStr::from_ptr(s);
  let s_str = str::from_utf8_unchecked(s_cstr.to_bytes());

  let mut w = platform::FileWriter(2);
  if errno >= 0 && errno < STR_ERROR.len() as c_int {
    w.write_fmt(format_args!("{}: {}\n", s_str, STR_ERROR[errno as usize]))
      .unwrap();
  } else {
    w.write_fmt(format_args!("{}: Unknown error {}\n", s_str, errno))
      .unwrap();
  }
}

#[no_mangle]
pub unsafe extern "C" fn fseek(stream: *mut FILE, offset: c_long, whence: c_int) -> c_int {
  fseeko(stream, offset as off_t, whence)
}

#[no_mangle]
pub unsafe extern "C" fn fseeko(stream: *mut FILE, off: off_t, whence: c_int) -> c_int {
  let mut stream = (*stream).lock();
  fseek_locked(&mut *stream, off, whence)
}

pub unsafe fn fseek_locked(stream: &mut FILE, mut off: off_t, whence: c_int) -> c_int {
  if whence == SEEK_CUR {
    off -= (stream.read_size - stream.read_pos) as off_t;
  }

  if stream.flush().is_err() {
    return -1;
  }

  let err = Sys::lseek(*stream.file, off, whence);
  if err < 0 {
    return err as c_int;
  }

  stream.flags &= !(F_EOF | F_ERR);
  stream.read_pos = 0;
  stream.read_size = 0;
  stream.unget = Vec::new();
  0
}

#[no_mangle]
pub unsafe extern "C" fn ftell(stream: *mut FILE) -> c_long {
  ftello(stream) as c_long
}

#[no_mangle]
pub unsafe extern "C" fn ftello(stream: *mut FILE) -> off_t {
  let mut stream = (*stream).lock();
  ftell_locked(&mut *stream)
}

pub unsafe extern "C" fn ftell_locked(stream: &mut FILE) -> off_t {
  let pos = Sys::lseek(*stream.file, 0, SEEK_CUR);
  if pos < 0 {
    return -1;
  }

  pos - (stream.read_size - stream.read_pos) as off_t - stream.unget.len() as off_t
}

#[no_mangle]
pub unsafe extern "C" fn rewind(stream: *mut FILE) {
  fseeko(stream, 0, SEEK_SET);
}

#[no_mangle]
pub unsafe extern "C" fn fread(
  ptr: *mut c_void,
  size: size_t,
  nitems: size_t,
  stream: *mut FILE,
) -> size_t {
  if size == 0 || nitems == 0 {
    return 0;
  }

  let mut stream = (*stream).lock();
  if stream.try_set_byte_orientation_unlocked().is_err() {
    return 0;
  }

  let buf = slice::from_raw_parts_mut(ptr as *mut u8, size * nitems);
  let mut read = 0;

  while read < buf.len() {
    match stream.read(&mut buf[read..]) {
      Ok(0) | Err(_) => break,
      Ok(n) => read += n,
    }
  }

  read / size
}

#[no_mangle]
pub unsafe extern "C" fn putchar(c: c_int) -> c_int {
  fputc(c, &mut *stdout)
}

#[no_mangle]
pub unsafe extern "C" fn fputc(c: c_int, stream: *mut FILE) -> c_int {
  let mut stream = (*stream).lock();
  if stream.try_set_byte_orientation_unlocked().is_err() {
    return EOF;
  }

  putc_unlocked(c, &mut *stream)
}

#[no_mangle]
pub unsafe extern "C" fn putc_unlocked(c: c_int, stream: *mut FILE) -> c_int {
  if (*stream).try_set_byte_orientation_unlocked().is_err() {
    return EOF;
  }

  match (*stream).write(&[c as u8]) {
    Ok(0) | Err(_) => EOF,
    Ok(_) => c,
  }
}
