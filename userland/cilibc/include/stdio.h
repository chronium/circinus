#ifndef _CILIBC_STDIO_H
#define _CILIBC_STDIO_H 1

#if defined(__cplusplus)
extern "C" {
#endif

  #include <stdarg.h>

  typedef struct { int fd; char* buf; } FILE;

  int puts(const char *s);

  int putchar(int);

  int printf(const char *restrict, ...);
  int vprintf(const char*restrict, va_list);

  int fputc(int, FILE *);
#define putc(c, stream) fputc(c, stream);

#define EOF 0

  extern FILE *__sF[3];

#define stderr (__sF[2])
#define stdin (__sF[0])
#define stdout (__sF[1])

#define BUFSIZ 512

#if defined(__cplusplus)
} /* extern "C" */
#endif

#endif /* _CILIBC_STDIO_H */
