use core::ptr;

use crate::{
  header::{ctype, stdio::flush_io_streams},
  platform::{self, sys::Sys, types::*},
};

use super::string::strlen;

pub const EXIT_SUCCESS: c_int = 0;
pub const EXIT_FAILURE: c_int = 1;

static mut ATEXIT_FUNCS: [Option<extern "C" fn()>; 32] = [None; 32];

#[no_mangle]
pub unsafe extern "C" fn strtod(s: *const c_char, endptr: *mut *mut c_char) -> c_double {
  strto_float_impl!(c_double, s, endptr)
}

#[no_mangle]
pub unsafe extern "C" fn malloc(size: size_t) -> *mut c_void {
  let ptr = platform::alloc(size);
  if ptr.is_null() {
    // TODO: errno = ENOMEM
    panic!("malloc failed");
  }
  ptr
}

#[no_mangle]
pub unsafe extern "C" fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void {
  let ptr = platform::realloc(ptr, size);
  if ptr.is_null() {
    // TODO: errno = ENOMEM
    panic!("realloc failed");
  }
  ptr
}

#[no_mangle]
pub unsafe extern "C" fn free(ptr: *mut c_void) {
  platform::free(ptr);
}

#[no_mangle]
#[linkage = "weak"]
pub unsafe extern "C" fn _fini() {}

#[no_mangle]
pub unsafe extern "C" fn exit(status: c_int) {
  extern "C" {
    static __fini_array_start: extern "C" fn();
    static __fini_array_end: extern "C" fn();

    fn _fini();
  }

  for i in (0..ATEXIT_FUNCS.len()).rev() {
    if let Some(func) = ATEXIT_FUNCS[i].take() {
      (func)();
    }
  }

  let mut f = &__fini_array_end as *const _;
  #[allow(clippy::op_ref)]
  while f > &__fini_array_start {
    f = f.offset(-1);
    (*f)();
  }

  _fini();

  flush_io_streams();

  Sys::exit(status);
}

unsafe fn find_env(search: *const c_char) -> Option<(usize, *mut c_char)> {
  for (i, mut item) in platform::environ_iter().enumerate() {
    let mut search = search;
    loop {
      let end_of_query = *search == 0 || *search == b'=' as c_char;

      assert_ne!(*item, 0, "environ has an item without value");

      if *item == b'=' as c_char || end_of_query {
        if *item == b'=' as c_char && end_of_query {
          return Some((i, item.add(1)));
        } else {
          break;
        }
      }

      if *item != *search {
        break;
      }

      item = item.add(1);
      search = search.add(1);
    }
  }

  None
}

#[no_mangle]
pub unsafe extern "C" fn getenv(name: *const c_char) -> *mut c_char {
  find_env(name).map(|val| val.1).unwrap_or(ptr::null_mut())
}

unsafe fn copy_kv(
  existing: *mut c_char,
  key: *const c_char,
  value: *const c_char,
  key_len: usize,
  value_len: usize,
) {
  core::ptr::copy_nonoverlapping(key, existing, key_len);
  core::ptr::write(existing.add(key_len), b'=' as c_char);
  core::ptr::copy_nonoverlapping(value, existing.add(key_len + 1), value_len);
  core::ptr::write(existing.add(key_len + 1 + value_len), 0);
}

#[no_mangle]
pub unsafe extern "C" fn setenv(
  key: *const c_char,
  value: *const c_char,
  overwrite: c_int,
) -> c_int {
  let key_len = strlen(key);
  let value_len = strlen(value);

  if let Some((i, existing)) = find_env(key) {
    if overwrite == 0 {
      return 0;
    }

    let existing_len = strlen(existing);

    if existing_len >= value_len {
      ptr::copy_nonoverlapping(value, existing, value_len);
      // TODO: fill end with zeroes
      ptr::write(existing.add(value_len), 0);
    } else {
      let ptr = platform::alloc(key_len as usize + 1 + value_len as usize + 1) as *mut c_char;
      copy_kv(ptr, key, value, key_len, value_len);
      platform::environ.add(1).write(ptr);
    }
  } else {
    let ptr = platform::alloc(key_len as usize + 1 + value_len as usize + 1) as *mut c_char;
    copy_kv(ptr, key, value, key_len, value_len);
    put_new_env(ptr);
  }

  0
}

unsafe fn put_new_env(insert: *mut c_char) {
  // XXX: Another problem is that `environ` can be set to any pointer, which means
  // there is a chance of a memory leak. But we can check if it was the same as
  // before, like musl does.
  if platform::environ == platform::OUR_ENVIRON.as_mut_ptr() {
    *platform::OUR_ENVIRON.last_mut().unwrap() = insert;
    platform::OUR_ENVIRON.push(core::ptr::null_mut());
    // Likely a no-op but is needed due to Stacked Borrows.
    platform::environ = platform::OUR_ENVIRON.as_mut_ptr();
  } else {
    platform::OUR_ENVIRON.clear();
    platform::OUR_ENVIRON.extend(platform::environ_iter());
    platform::OUR_ENVIRON.push(insert);
    platform::OUR_ENVIRON.push(core::ptr::null_mut());
    platform::environ = platform::OUR_ENVIRON.as_mut_ptr();
  }
}
