use core::ptr;

use cbitset::BitSet256;

use crate::platform::types::*;

#[no_mangle]
pub unsafe extern "C" fn strlen(s: *const c_char) -> size_t {
	strnlen(s, usize::MAX)
}

#[no_mangle]
pub unsafe extern "C" fn strnlen(s: *const c_char, size: size_t) -> size_t {
	let mut i = 0;
	while i < size {
		if *s.add(i) == 0 {
			break;
		}
		i += 1;
	}
	i as size_t
}

#[no_mangle]
pub unsafe extern "C" fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int {
	let s1 = core::slice::from_raw_parts(s1 as *const c_uchar, n);
	let s2 = core::slice::from_raw_parts(s2 as *const c_uchar, n);

	for (&a, &b) in s1.iter().zip(s2.iter()) {
		let val = (a as c_int) - (b as c_int);
		if a != b || a == 0 {
			return val;
		}
	}

	0
}

#[no_mangle]
pub unsafe extern "C" fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int {
	strncmp(s1, s2, usize::MAX)
}

pub unsafe fn inner_strspn(s1: *const c_char, s2: *const c_char, cmp: bool) -> size_t {
	let mut s1 = s1 as *const u8;
	let mut s2 = s2 as *const u8;

	let mut set = BitSet256::new();

	while *s2 != 0 {
		set.insert(*s2 as usize);
		s2 = s2.offset(1);
	}

	let mut i = 0;
	while *s1 != 0 {
		if set.contains(*s1 as usize) != cmp {
			break;
		}
		i += 1;
		s1 = s1.offset(1);
	}
	i
}

#[no_mangle]
pub unsafe extern "C" fn strspn(s1: *const c_char, s2: *const c_char) -> size_t {
	inner_strspn(s1, s2, true)
}

#[no_mangle]
pub unsafe extern "C" fn strcspn(s1: *const c_char, s2: *const c_char) -> size_t {
	inner_strspn(s1, s2, false)
}

#[no_mangle]
pub unsafe extern "C" fn strpbrk(s1: *const c_char, s2: *const c_char) -> *mut c_char {
	let p = s1.add(strcspn(s1, s2));
	if *p != 0 {
		p as *mut c_char
	} else {
		ptr::null_mut()
	}
}

#[no_mangle]
pub unsafe extern "C" fn strtok(s1: *mut c_char, delimiter: *const c_char) -> *mut c_char {
	static mut HAYSTACK: *mut c_char = ptr::null_mut();
	strtok_r(s1, delimiter, &mut HAYSTACK)
}

#[no_mangle]
pub unsafe extern "C" fn strtok_r(
	s: *mut c_char,
	delimiter: *const c_char,
	lasts: *mut *mut c_char,
) -> *mut c_char {
	// Loosely based on GLIBC implementation
	let mut haystack = s;
	if haystack.is_null() {
		if (*lasts).is_null() {
			return ptr::null_mut();
		}
		haystack = *lasts;
	}

	// Skip past any extra delimiter left over from previous call
	haystack = haystack.add(strspn(haystack, delimiter));
	if *haystack == 0 {
		*lasts = ptr::null_mut();
		return ptr::null_mut();
	}

	// Build token by injecting null byte into delimiter
	let token = haystack;
	haystack = strpbrk(token, delimiter);
	if !haystack.is_null() {
		haystack.write(0);
		haystack = haystack.add(1);
		*lasts = haystack;
	} else {
		*lasts = ptr::null_mut();
	}

	token
}
