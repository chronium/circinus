#ifndef _CILIBC_STDIO_H
#define _CILIBC_STDIO_H 1

#if defined(__cplusplus)
extern "C" {
#endif

  #include <stdarg.h>

  int puts(const char *s);
  int putchar(int);

  int printf(const char *restrict, ...);
  int vprintf(const char*restrict, va_list);

#if defined(__cplusplus)
} /* extern "C" */
#endif

#endif /* _CILIBC_STDIO_H */
