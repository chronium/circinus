#ifndef _CILIBC_STDIO_H
#define _CILIBC_STDIO_H 1

#if defined(__cplusplus)
extern "C" {
#endif

#include <stdarg.h>
#include <bits/sys/types.h>

  typedef struct {
    int fd;
    char* buf;
    size_t bufsiz;
    off_t bufpos;
    int flags;
  } FILE;

  int puts(const char *s);

  int putchar(int);

  int printf(const char *restrict, ...);
  int vprintf(const char*restrict, va_list);

  int putchar(int);
  int fputc(int, FILE *);
  int fputs(const char *restrict, FILE *restrict);

  int fflush(FILE *);

  FILE *fopen(const char *restrict, const char *restrict);
  int fclose(FILE *);

  int getc(FILE *);
  char *gets(FILE *);

#define putc(c, stream) fputc(c, stream);

#define EOF 0

  extern FILE *__sF[3];

#define stderr (__sF[2])
#define stdin (__sF[0])
#define stdout (__sF[1])

#define BUFSIZ 512

#define _IOFBF 0b00000001
#define _IOLBF 0b00000010
#define _IONBF 0b00000100

#if defined(__cplusplus)
} /* extern "C" */
#endif

#endif /* _CILIBC_STDIO_H */