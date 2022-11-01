#include <stdarg.h>
#include <stddef.h>

typedef struct FILE FILE;

int vasprintf(char **strp, const char *fmt, va_list ap);

int asprintf(char **strp, const char *fmt, ...) {
  int ret;
  va_list ap;
  va_start(ap, fmt);
  ret = vasprintf(strp, fmt, ap);
  va_end(ap);
  return ret;
}

int vfprintf(FILE *stream, const char *fmt, va_list ap);

int fprintf(FILE *stream, const char *fmt, ...) {
  int ret;
  va_list ap;
  va_start(ap, fmt);
  ret = vfprintf(stream, fmt, ap);
  va_end(ap);
  return ret;
}

int vprintf(const char *fmt, va_list ap);

int printf(const char *fmt, ...) {
  int ret;
  va_list ap;
  va_start(ap, fmt);
  ret = vprintf(fmt, ap);
  va_end(ap);
  return ret;
}

int vsnprintf(char *str, size_t size, const char *fmt, va_list ap);

int snprintf(char *str, size_t size, const char *fmt, ...) {
  int ret;
  va_list ap;
  va_start(ap, fmt);
  ret = vsnprintf(str, size, fmt, ap);
  va_end(ap);
  return ret;
}

int vsprintf(char *str, const char *fmt, va_list ap);

int sprintf(char *str, const char *fmt, ...) {
  int ret;
  va_list ap;
  va_start(ap, fmt);
  ret = vsprintf(str, fmt, ap);
  va_end(ap);
  return ret;
}

int vfscanf(FILE *stream, const char *fmt, va_list ap);

int fscanf(FILE *stream, const char *fmt, ...) {
  int ret;
  va_list ap;
  va_start(ap, fmt);
  ret = vfscanf(stream, fmt, ap);
  va_end(ap);
  return ret;
}

int vscanf(const char *fmt, va_list ap);

int scanf(const char *fmt, ...) {
  int ret;
  va_list ap;
  va_start(ap, fmt);
  ret = vscanf(fmt, ap);
  va_end(ap);
  return ret;
}

int vsscanf(const char *input, const char *fmt, va_list ap);

int sscanf(const char *input, const char *fmt, ...) {
  int ret;
  va_list ap;
  va_start(ap, fmt);
  ret = vsscanf(input, fmt, ap);
  va_end(ap);
  return ret;
}
