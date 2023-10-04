use core::{ffi::VaList, ops::Range, slice};

use alloc::{
  collections::BTreeMap,
  string::{String, ToString},
  vec::Vec,
};

use crate::{
  io::{self, Write},
  platform::{self, types::*},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum IntKind {
  Byte,
  Short,
  Int,
  Long,
  LongLong,
  IntMax,
  PtrDiff,
  Size,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FmtKind {
  Percent,

  Signed,
  Unsigned,

  Scientific,
  Decimal,
  AnyNotation,

  String,
  Char,
  Pointer,
  GetWritten,
}

#[derive(Debug, Clone, Copy)]
enum Number {
  Static(usize),
  Index(usize),
  Next,
}

impl Number {
  unsafe fn resolve(self, varargs: &mut VaListCache, ap: &mut VaList) -> usize {
    let arg = match self {
      Number::Static(n) => return n,
      Number::Index(n) => varargs.get(n - 1, ap, None),
      Number::Next => {
        let i = varargs.i;
        varargs.i += 1;
        varargs.get(i, ap, None)
      }
    };

    match arg {
      VaArg::c_char(i) => i as usize,
      VaArg::c_double(i) => i as usize,
      VaArg::c_int(i) => i as usize,
      VaArg::c_long(i) => i as usize,
      VaArg::c_longlong(i) => i as usize,
      VaArg::c_short(i) => i as usize,
      VaArg::intmax_t(i) => i as usize,
      VaArg::pointer(i) => i as usize,
      VaArg::ptrdiff_t(i) => i as usize,
      VaArg::ssize_t(i) => i as usize,
      VaArg::wint_t(i) => i as usize,
    }
  }
}

#[derive(Debug, Clone, Copy)]
enum VaArg {
  c_char(c_char),
  c_double(c_double),
  c_int(c_int),
  c_long(c_long),
  c_longlong(c_longlong),
  c_short(c_short),
  intmax_t(intmax_t),
  pointer(*const c_void),
  ptrdiff_t(ptrdiff_t),
  ssize_t(ssize_t),
  wint_t(wint_t),
}

impl VaArg {
  unsafe fn arg_from(fmtkind: FmtKind, intkind: IntKind, ap: &mut VaList) -> VaArg {
    match (fmtkind, intkind) {
      (FmtKind::Percent, _) => panic!("Can't call arg_from on %"),

      (FmtKind::Char, IntKind::Long) | (FmtKind::Char, IntKind::LongLong) => {
        VaArg::wint_t(ap.arg::<wint_t>())
      }

      (FmtKind::Char, _)
      | (FmtKind::Unsigned, IntKind::Byte)
      | (FmtKind::Signed, IntKind::Byte) => VaArg::c_char(ap.arg::<c_char>()),
      (FmtKind::Unsigned, IntKind::Short) | (FmtKind::Signed, IntKind::Short) => {
        VaArg::c_short(ap.arg::<c_short>())
      }
      (FmtKind::Unsigned, IntKind::Int) | (FmtKind::Signed, IntKind::Int) => {
        VaArg::c_int(ap.arg::<c_int>())
      }
      (FmtKind::Unsigned, IntKind::Long) | (FmtKind::Signed, IntKind::Long) => {
        VaArg::c_long(ap.arg::<c_long>())
      }
      (FmtKind::Unsigned, IntKind::LongLong) | (FmtKind::Signed, IntKind::LongLong) => {
        VaArg::c_longlong(ap.arg::<c_longlong>())
      }
      (FmtKind::Unsigned, IntKind::IntMax) | (FmtKind::Signed, IntKind::IntMax) => {
        VaArg::intmax_t(ap.arg::<intmax_t>())
      }
      (FmtKind::Unsigned, IntKind::PtrDiff) | (FmtKind::Signed, IntKind::PtrDiff) => {
        VaArg::ptrdiff_t(ap.arg::<ptrdiff_t>())
      }
      (FmtKind::Unsigned, IntKind::Size) | (FmtKind::Signed, IntKind::Size) => {
        VaArg::ssize_t(ap.arg::<ssize_t>())
      }

      (FmtKind::AnyNotation, _) | (FmtKind::Decimal, _) | (FmtKind::Scientific, _) => {
        VaArg::c_double(ap.arg::<c_double>())
      }

      (FmtKind::GetWritten, _) | (FmtKind::Pointer, _) | (FmtKind::String, _) => {
        VaArg::pointer(ap.arg::<*const c_void>())
      }
    }
  }

  unsafe fn transmute(&self, fmtkind: FmtKind, intkind: IntKind) -> VaArg {
    union Untyped {
      c_char: c_char,
      c_double: c_double,
      c_int: c_int,
      c_long: c_long,
      c_longlong: c_longlong,
      c_short: c_short,
      intmax_t: intmax_t,
      pointer: *const c_void,
      ptrdiff_t: ptrdiff_t,
      ssize_t: ssize_t,
      wint_t: wint_t,
    }

    let untyped = match *self {
      VaArg::c_char(x) => Untyped { c_char: x },
      VaArg::c_double(x) => Untyped { c_double: x },
      VaArg::c_int(x) => Untyped { c_int: x },
      VaArg::c_long(x) => Untyped { c_long: x },
      VaArg::c_longlong(x) => Untyped { c_longlong: x },
      VaArg::c_short(x) => Untyped { c_short: x },
      VaArg::intmax_t(x) => Untyped { intmax_t: x },
      VaArg::pointer(x) => Untyped { pointer: x },
      VaArg::ptrdiff_t(x) => Untyped { ptrdiff_t: x },
      VaArg::ssize_t(x) => Untyped { ssize_t: x },
      VaArg::wint_t(x) => Untyped { wint_t: x },
    };

    match (fmtkind, intkind) {
      (FmtKind::Percent, _) => panic!("Can't call transmute on %"),

      (FmtKind::Char, IntKind::Long) | (FmtKind::Char, IntKind::LongLong) => {
        VaArg::wint_t(untyped.wint_t)
      }

      (FmtKind::Char, _)
      | (FmtKind::Unsigned, IntKind::Byte)
      | (FmtKind::Signed, IntKind::Byte) => VaArg::c_char(untyped.c_char),
      (FmtKind::Unsigned, IntKind::Short) | (FmtKind::Signed, IntKind::Short) => {
        VaArg::c_short(untyped.c_short)
      }
      (FmtKind::Unsigned, IntKind::Int) | (FmtKind::Signed, IntKind::Int) => {
        VaArg::c_int(untyped.c_int)
      }
      (FmtKind::Unsigned, IntKind::Long) | (FmtKind::Signed, IntKind::Long) => {
        VaArg::c_long(untyped.c_long)
      }
      (FmtKind::Unsigned, IntKind::LongLong) | (FmtKind::Signed, IntKind::LongLong) => {
        VaArg::c_longlong(untyped.c_longlong)
      }
      (FmtKind::Unsigned, IntKind::IntMax) | (FmtKind::Signed, IntKind::IntMax) => {
        VaArg::intmax_t(untyped.intmax_t)
      }
      (FmtKind::Unsigned, IntKind::PtrDiff) | (FmtKind::Signed, IntKind::PtrDiff) => {
        VaArg::ptrdiff_t(untyped.ptrdiff_t)
      }
      (FmtKind::Unsigned, IntKind::Size) | (FmtKind::Signed, IntKind::Size) => {
        VaArg::ssize_t(untyped.ssize_t)
      }

      (FmtKind::AnyNotation, _) | (FmtKind::Decimal, _) | (FmtKind::Scientific, _) => {
        VaArg::c_double(untyped.c_double)
      }

      (FmtKind::GetWritten, _) | (FmtKind::Pointer, _) | (FmtKind::String, _) => {
        VaArg::pointer(untyped.pointer)
      }
    }
  }
}

#[derive(Default)]
struct VaListCache {
  args: Vec<VaArg>,
  i: usize,
}

enum FmtCase {
  Lower,
  Upper,
}

impl VaListCache {
  unsafe fn get(
    &mut self,
    i: usize,
    ap: &mut VaList,
    default: Option<(FmtKind, IntKind)>,
  ) -> VaArg {
    if let Some(&arg) = self.args.get(i) {
      let mut arg = arg;
      if let Some((fmtkind, intkind)) = default {
        arg = arg.transmute(fmtkind, intkind);
      }
      return arg;
    }

    while self.args.len() < i {
      self.args.push(VaArg::c_int(ap.arg::<c_int>()))
    }

    self.args.push(match default {
      Some((fmtkind, intkind)) => VaArg::arg_from(fmtkind, intkind, ap),
      None => VaArg::c_int(ap.arg::<c_int>()),
    });

    self.args[i]
  }
}

unsafe fn pop_int_raw(format: &mut *const u8) -> Option<usize> {
  let mut int = None;

  while let Some(digit) = (**format as char).to_digit(10) {
    *format = format.add(1);
    if int.is_none() {
      int = Some(0);
    }
    *int.as_mut().unwrap() *= 10;
    *int.as_mut().unwrap() += digit as usize;
  }

  int
}

unsafe fn pop_index(format: &mut *const u8) -> Option<usize> {
  let mut format2 = *format;

  if let Some(i) = pop_int_raw(&mut format2) {
    if *format2 == b'$' {
      *format = format2.add(1);
      return Some(i);
    }
  }

  None
}

unsafe fn pop_int(format: &mut *const u8) -> Option<Number> {
  if **format == b'*' {
    *format = format.add(1);
    Some(pop_index(format).map(Number::Index).unwrap_or(Number::Next))
  } else {
    pop_int_raw(format).map(Number::Static)
  }
}

fn pad<W: Write>(
  w: &mut W,
  current_side: bool,
  pad_char: u8,
  range: Range<usize>,
) -> io::Result<()> {
  if current_side {
    for _ in range {
      w.write_all(&[pad_char])?;
    }
  }
  Ok(())
}

#[derive(Clone, Copy)]
struct PrintfIter {
  format: *const u8,
}

#[derive(Debug, Clone, Copy)]
struct PrintfArg {
  index: Option<usize>,
  alternate: bool,
  zero: bool,
  left: bool,
  sign_reserve: bool,
  sign_always: bool,
  min_width: Number,
  precision: Option<Number>,
  intkind: IntKind,
  fmt: u8,
  fmtkind: FmtKind,
}

#[derive(Debug)]
enum PrintfFmt {
  Plain(&'static [u8]),
  Arg(PrintfArg),
}

impl Iterator for PrintfIter {
  type Item = Result<PrintfFmt, ()>;

  fn next(&mut self) -> Option<Self::Item> {
    unsafe {
      let mut len = 0;
      while *self.format.add(len) != 0 && *self.format.add(len) != b'%' {
        len += 1;
      }

      if len > 0 {
        let slice = slice::from_raw_parts(self.format, len);
        self.format = self.format.add(len);
        return Some(Ok(PrintfFmt::Plain(slice)));
      }

      self.format = self.format.add(len);
      if *self.format == 0 {
        return None;
      }

      self.format = self.format.add(1);

      let mut peekahead = self.format;
      let index = pop_index(&mut peekahead).map(|i| {
        self.format = peekahead;
        i
      });

      let mut alternate = false;
      let mut zero = false;
      let mut left = false;
      let mut sign_reserve = false;
      let mut sign_always = false;

      loop {
        match *self.format {
          b'#' => alternate = true,
          b'0' => zero = true,
          b'-' => left = true,
          b' ' => sign_reserve = true,
          b'+' => sign_always = true,
          _ => break,
        }
        self.format = self.format.add(1);
      }

      let min_width = pop_int(&mut self.format).unwrap_or(Number::Static(0));
      let precision = if *self.format == b'.' {
        self.format = self.format.add(1);
        match pop_int(&mut self.format) {
          int @ Some(_) => int,
          None => return Some(Err(())),
        }
      } else {
        None
      };

      let mut intkind = IntKind::Int;
      loop {
        intkind = match *self.format {
          b'h' => {
            if intkind == IntKind::Short || intkind == IntKind::Byte {
              IntKind::Byte
            } else {
              IntKind::Short
            }
          }
          b'j' => IntKind::IntMax,
          b'l' => {
            if intkind == IntKind::Long || intkind == IntKind::LongLong {
              IntKind::LongLong
            } else {
              IntKind::Long
            }
          }
          b'q' | b'L' => IntKind::LongLong,
          b't' => IntKind::PtrDiff,
          b'z' => IntKind::Size,
          _ => break,
        };

        self.format = self.format.add(1);
      }

      let fmt = *self.format;
      let fmtkind = match fmt {
        b'%' => FmtKind::Percent,
        b'd' | b'i' => FmtKind::Signed,
        b'o' | b'u' | b'x' | b'X' => FmtKind::Unsigned,
        b'e' | b'E' => FmtKind::Scientific,
        b'f' | b'F' => FmtKind::Decimal,
        b'g' | b'G' => FmtKind::AnyNotation,
        b's' => FmtKind::String,
        b'c' => FmtKind::Char,
        b'p' => FmtKind::Pointer,
        b'n' => FmtKind::GetWritten,
        _ => return Some(Err(())),
      };

      self.format = self.format.add(1);

      Some(Ok(PrintfFmt::Arg(PrintfArg {
        index,
        alternate,
        zero,
        left,
        sign_reserve,
        sign_always,
        min_width,
        precision,
        intkind,
        fmt,
        fmtkind,
      })))
    }
  }
}

unsafe fn inner_printf<W: Write>(w: W, format: *const c_char, mut ap: VaList) -> io::Result<c_int> {
  let w = &mut platform::CountingWriter::new(w);

  let iterator = PrintfIter {
    format: format as *const u8,
  };

  let mut varargs = VaListCache::default();
  let mut positional = BTreeMap::new();

  for section in iterator {
    let arg = match section {
      Ok(PrintfFmt::Plain(_)) => continue,
      Ok(PrintfFmt::Arg(arg)) => arg,
      Err(()) => return Ok(-1),
    };

    if arg.fmtkind == FmtKind::Percent {
      continue;
    }

    for num in &[arg.min_width, arg.precision.unwrap_or(Number::Static(0))] {
      match num {
        Number::Next => varargs.args.push(VaArg::c_int(ap.arg::<c_int>())),
        Number::Index(i) => {
          positional.insert(i - 1, (FmtKind::Signed, IntKind::Int));
        }
        Number::Static(_) => (),
      }
    }

    match arg.index {
      Some(i) => {
        positional.insert(i - 1, (arg.fmtkind, arg.intkind));
      }
      None => varargs
        .args
        .push(VaArg::arg_from(arg.fmtkind, arg.intkind, &mut ap)),
    }
  }

  for (i, arg) in positional {
    varargs.get(i, &mut ap, Some(arg));
  }

  for section in iterator {
    let arg = match section {
      Ok(PrintfFmt::Plain(text)) => {
        w.write_all(text)?;
        continue;
      }
      Ok(PrintfFmt::Arg(arg)) => arg,
      Err(()) => return Ok(-1),
    };
    let alternate = arg.alternate;
    let zero = arg.zero;
    let mut left = arg.left;
    let sign_reserve = arg.sign_reserve;
    let sign_always = arg.sign_always;
    let min_width = arg.min_width.resolve(&mut varargs, &mut ap);
    let precision = arg.precision.map(|n| n.resolve(&mut varargs, &mut ap));
    let pad_zero = if zero { min_width } else { 0 };
    let signed_space = match pad_zero {
      0 => min_width as isize,
      _ => 0,
    };
    let pad_space = if signed_space < 0 {
      left = true;
      -signed_space as usize
    } else {
      signed_space as usize
    };
    let intkind = arg.intkind;
    let fmt = arg.fmt;
    let fmtkind = arg.fmtkind;
    let fmtcase = match fmt {
      b'x' | b'f' | b'e' | b'g' => Some(FmtCase::Lower),
      b'X' | b'F' | b'E' | b'G' => Some(FmtCase::Upper),
      _ => None,
    };

    let index = arg.index.map(|i| i - 1).unwrap_or_else(|| {
      if fmtkind == FmtKind::Percent {
        0
      } else {
        let i = varargs.i;
        varargs.i += 1;
        i
      }
    });

    match fmtkind {
      FmtKind::Signed => {
        let string = match varargs.get(index, &mut ap, Some((arg.fmtkind, arg.intkind))) {
          VaArg::c_char(i) => i.to_string(),
          VaArg::c_double(i) => panic!("this should not be possible"),
          VaArg::c_int(i) => i.to_string(),
          VaArg::c_long(i) => i.to_string(),
          VaArg::c_longlong(i) => i.to_string(),
          VaArg::c_short(i) => i.to_string(),
          VaArg::intmax_t(i) => i.to_string(),
          VaArg::pointer(i) => (i as usize).to_string(),
          VaArg::ptrdiff_t(i) => i.to_string(),
          VaArg::ssize_t(i) => i.to_string(),
          VaArg::wint_t(_) => unreachable!("this should not be possible"),
        };
        let positive = !string.starts_with('-');
        let zero = precision == Some(0) && string == "0";

        let mut len = string.len();
        let mut final_len = string.len().max(precision.unwrap_or(0));
        if positive && (sign_reserve || sign_always) {
          final_len += 1;
        }
        if zero {
          len = 0;
          final_len = 0;
        }

        pad(w, !left, b' ', final_len..pad_space)?;

        let bytes = if positive {
          if sign_reserve {
            w.write_all(&[b' '])?;
          } else if sign_always {
            w.write_all(&[b'+'])?;
          }
          string.as_bytes()
        } else {
          w.write_all(&[b'-'])?;
          &string.as_bytes()[1..]
        };
        pad(w, true, b'0', len..precision.unwrap_or(pad_zero))?;

        if !zero {
          w.write_all(bytes)?;
        }

        pad(w, left, b' ', final_len..pad_space)?;
      }
      FmtKind::String => {
        let ptr = match varargs.get(index, &mut ap, Some((FmtKind::String, IntKind::Int))) {
          VaArg::pointer(p) => p,
          _ => unreachable!(),
        } as *const c_char;

        if ptr.is_null() {
          w.write_all(b"(null)")?;
        } else {
          let max = precision.unwrap_or(::core::usize::MAX);

          if intkind == IntKind::Long || intkind == IntKind::LongLong {
            let mut ptr = ptr as *const wchar_t;
            let mut string = String::new();

            while *ptr != 0 {
              let c = match char::from_u32(*ptr as _) {
                Some(c) => c,
                None => {
                  // TODO: platform::errno
                  todo!()
                }
              };

              if string.len() + c.len_utf8() >= max {
                break;
              }

              string.push(c);
              ptr = ptr.add(1);
            }

            pad(w, !left, b' ', string.len()..pad_space)?;
            w.write_all(string.as_bytes())?;
            pad(w, left, b' ', string.len()..pad_space)?;
          } else {
            let mut len = 0;
            while *ptr.add(len) != 0 && len < max {
              len += 1;
            }

            pad(w, !left, b' ', len..pad_space)?;
            w.write_all(slice::from_raw_parts(ptr as *const u8, len))?;
            pad(w, left, b' ', len..pad_space)?;
          }
        }
      }
      FmtKind::Char => match varargs.get(index, &mut ap, Some((arg.fmtkind, arg.intkind))) {
        VaArg::c_char(c) => {
          pad(w, !left, b' ', 1..pad_space)?;
          w.write_all(&[c as u8])?;
          pad(w, left, b' ', 1..pad_space)?;
        }
        VaArg::wint_t(c) => {
          let c = match char::from_u32(c as _) {
            Some(c) => c,
            None => {
              // TODO: platform::errno
              todo!()
            }
          };
          let mut buf = [0; 4];

          pad(w, !left, b' ', 1..pad_space)?;
          w.write_all(c.encode_utf8(&mut buf).as_bytes())?;
          pad(w, left, b' ', 1..pad_space)?;
        }
        _ => unreachable!(),
      },
      FmtKind::Pointer => {
        let ptr = match varargs.get(index, &mut ap, Some((arg.fmtkind, arg.intkind))) {
          VaArg::pointer(p) => p,
          _ => panic!("this should not be possible"),
        };

        let mut len = 1;
        if ptr.is_null() {
          len = "(nil)".len();
        } else {
          let mut ptr = ptr as usize;
          while ptr >= 10 {
            ptr /= 10;
            len += 1;
          }
        }

        pad(w, !left, b' ', len..pad_space)?;
        if ptr.is_null() {
          write!(w, "(nil)")?;
        } else {
          write!(w, "0x{:x}", ptr as usize)?;
        }
        pad(w, left, b' ', len..pad_space)?;
      }
      _ => panic!("Unimplemented {:?}", fmtkind),
    }
  }

  Ok(w.written as c_int)
}

pub unsafe fn printf<W: Write>(w: W, format: *const c_char, ap: VaList) -> c_int {
  inner_printf(w, format, ap).unwrap_or(-1)
}
