use crate::platform::types::*;

#[no_mangle]
pub extern "C" fn isspace(c: c_int) -> c_int {
	c_int::from(
		c == c_int::from(b' ')
			|| c == c_int::from(b'\t')
			|| c == c_int::from(b'\n')
			|| c == c_int::from(b'\r')
			|| c == 0x0b || c == 0x0c,
	)
}
