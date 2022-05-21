/// Prints a string.
#[macro_export]
macro_rules! print {
	($($arg:tt)*) => {{
		#![allow(unused_imports)]
		use core::fmt::Write;
		write!($crate::io::stdout(), "{}", format_args!($($arg)*)).ok();
	}};
}

/// Prints a string and a newline.
#[macro_export]
macro_rules! println {
    () => {{
        $crate::print!(
            "\n"
        );
    }};
    ($fmt:expr) => {{
        $crate::print!(
          concat!( $fmt, "\n"),
        );
    }};
    ($fmt:expr, $($arg:tt)*) => {{
        $crate::print!(
            concat!( $fmt, "\n"),
            $($arg)*
        );
    }};
}
