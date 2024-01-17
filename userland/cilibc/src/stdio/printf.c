#include <stdio.h>
#include <string.h>
#include <unistd.h>

int putchar(int c) {
  int buf[2];
  buf[0] = c;
  write(1, buf, 2);
  return c;
}

int printf(const char* fmt, ...) {
  va_list arg; 

  va_start(arg, fmt);
  
  int written_len = vprintf(fmt, arg);

  va_end(arg);

  return written_len;
}

int vprint_s(const char* s) {
  return puts(s);
}

int vprintf(const char* fmt, va_list arg) {
  int fmt_length = strlen(fmt);
  int written_len;
  char ch;
  char s;

  char buf[2];

  for (int i = 0; i < fmt_length; i++) {
    ch = fmt[i];

    if (ch == '%') {
      s = fmt[++i];
      
      switch (s) {
        case 's':
          written_len += vprint_s(va_arg(arg, const char*));
          break;
        default:
          break;
      }

    } else {
      written_len++;
      buf[0] = fmt[i];
      buf[1] = 0;
      puts(buf);
    }
  }

  return written_len;
}
