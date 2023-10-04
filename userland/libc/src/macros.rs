#[macro_export]
macro_rules! strto_float_impl {
  ($type:ident, $s:expr, $endptr:expr) => {{
    let mut s = $s;
    let endptr = $endptr;

    // TODO: Handle named floats: NaN, Inf...

    while ctype::isspace(*s as c_int) != 0 {
      s = s.offset(1);
    }

    let mut result: $type = 0.0;
    let mut radix = 10;

    let result_sign = match *s as u8 {
      b'-' => {
        s = s.offset(1);
        -1.0
      }
      b'+' => {
        s = s.offset(1);
        1.0
      }
      _ => 1.0,
    };

    if *s as u8 == b'0' && *s.offset(1) as u8 == b'x' {
      s = s.offset(2);
      radix = 16;
    }

    while let Some(digit) = (*s as u8 as char).to_digit(radix) {
      result *= radix as $type;
      result += digit as $type;
      s = s.offset(1);
    }

    if *s as u8 == b'.' {
      s = s.offset(1);

      let mut i = 1.0;
      while let Some(digit) = (*s as u8 as char).to_digit(radix) {
        i *= radix as $type;
        result += digit as $type / i;
        s = s.offset(1);
      }
    }

    let s_before_exponent = s;

    let exponent = match (*s as u8, radix) {
      (b'e' | b'E', 10) | (b'p' | b'P', 16) => {
        s = s.offset(1);

        let is_exponent_positive = match *s as u8 {
          b'-' => {
            s = s.offset(1);
            false
          }
          b'+' => {
            s = s.offset(1);
            true
          }
          _ => true,
        };

        // Exponent digits are always in base 10.
        if (*s as u8 as char).is_digit(10) {
          let mut exponent_value = 0;

          while let Some(digit) = (*s as u8 as char).to_digit(10) {
            exponent_value *= 10;
            exponent_value += digit;
            s = s.offset(1);
          }

          let exponent_base = match radix {
            10 => 10u128,
            16 => 2u128,
            _ => unreachable!(),
          };

          if is_exponent_positive {
            Some(exponent_base.pow(exponent_value) as $type)
          } else {
            Some(1.0 / (exponent_base.pow(exponent_value) as $type))
          }
        } else {
          // Exponent had no valid digits after 'e'/'p' and '+'/'-', rollback
          s = s_before_exponent;
          None
        }
      }
      _ => None,
    };

    if !endptr.is_null() {
      // This is stupid, but apparently strto* functions want
      // const input but mut output, yet the man page says
      // "stores the address of the first invalid character in *endptr"
      // so obviously it doesn't want us to clone it.
      *endptr = s as *mut _;
    }

    if let Some(exponent) = exponent {
      result_sign * result * exponent
    } else {
      result_sign * result
    }
  }};
}

/// Print to stdout
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let _ = write!($crate::platform::FileWriter(1), $($arg)*);
    });
}

/// Print with new line to stdout
#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

/// Print to stderr
#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let _ = write!($crate::platform::FileWriter(2), $($arg)*);
    });
}

/// Print with new line to stderr
#[macro_export]
macro_rules! eprintln {
    () => (eprint!("\n"));
    ($fmt:expr) => (eprint!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (eprint!(concat!($fmt, "\n"), $($arg)*));
}

/// Lifted from libstd
#[macro_export]
macro_rules! dbg {
  () => {
    eprintln!("[{}:{}]", file!(), line!());
  };
  ($val:expr) => {
    // Use of `match` here is intentional because it affects the lifetimes
    // of temporaries - https://stackoverflow.com/a/48732525/1063961
    match $val {
      tmp => {
        eprintln!(
          "[{}:{}] {} = {:#?}",
          file!(),
          line!(),
          stringify!($val),
          &tmp
        );
        tmp
      }
    }
  };
}
