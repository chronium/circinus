use crate::{header::ctype, platform::types::*};

#[no_mangle]
pub unsafe extern "C" fn strtod(s: *const c_char, endptr: *mut *mut c_char) -> c_double {
	strto_float_impl!(c_double, s, endptr)
}
