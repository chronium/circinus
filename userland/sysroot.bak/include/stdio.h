#ifndef _CILIBC_STDIO_H
#define _CILIBC_STDIO_H

#include <stdarg.h>
#include <stddef.h>
#include <stdint.h>
#include <sys/types.h>

#define EOF -1

#define BUFSIZ 1024

#define UNGET 8

#define FILENAME_MAX 4096

#define F_PERM 1

#define F_NORD 4

#define F_NOWR 8

#define F_EOF 16

#define F_ERR 32

#define F_SVB 64

#define F_APP 128

#define F_BADJ 256

#define SEEK_SET 0

#define SEEK_CUR 1

#define SEEK_END 2

#define _IOFBF 0

#define _IOLBF 1

#define _IONBF 2

#define L_tmpnam 7

#define TMP_MAX 2147483647

typedef struct FILE FILE;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

extern FILE *stdin;

extern FILE *stdout;

extern FILE *stderr;

void flockfile(FILE *file);

void funlockfile(FILE *file);

int puts(const char *s);

int fflush(FILE *stream);

int fgetc(FILE *stream);

int getc(FILE *stream);

int getchar(void);

int getc_unlocked(FILE *stream);

int vfprintf(FILE *file, const char *format, va_list ap);

int vprintf(const char *format, va_list ap);

int vasprintf(char **strp, const char *format, va_list ap);

int vsnprintf(char *s, size_t n, const char *format, va_list ap);

int vsprintf(char *s, const char *format, va_list ap);

int vfscanf(FILE *file, const char *format, va_list ap);

int vscanf(const char *format, va_list ap);

int vsscanf(const char *s, const char *format, va_list ap);

int fclose(FILE *stream);

FILE *fopen(const char *filename, const char *mode);

void perror(const char *s);

int fseek(FILE *stream, long offset, int whence);

int fseeko(FILE *stream, off_t off, int whence);

long ftell(FILE *stream);

off_t ftello(FILE *stream);

void rewind(FILE *stream);

size_t fread(void *ptr, size_t size, size_t nitems, FILE *stream);

int putchar(int c);

int fputc(int c, FILE *stream);

int putc_unlocked(int c, FILE *stream);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* _CILIBC_STDIO_H */

#include <bits/stdio.h>
