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
