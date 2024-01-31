#include "__stdio.h"
#include <fcntl.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

int fputc(register int c, FILE *stream) {
  if (stream->bufpos + 1 >= stream->bufsiz)
    fflush(stream);

  stream->buf[stream->bufpos++] = (unsigned char)c;

  if (stream->flags & _IOLBF && c == '\n')
    fflush(stream);

  if (stream->bufpos >= stream->bufsiz)
    fflush(stream);

  return c;
}

int fflush(FILE *stream) {
  // TODO: Error

  write(stream->fd, stream->buf, stream->bufpos);
  stream->bufpos = 0;

  return 0;
}

int fputs(const char *restrict s, FILE *restrict stream) {
  const char *p = s;

  // TODO: EOF check
  while (*p != 0) {
    fputc(*p++, stream);
  }

  return p - s;
}

int putchar(int c) { return fputc(c, stdout); }

int parse_mode_flags(const char *flags);

FILE *fopen(const char *restrict filename, const char *restrict mode) {
  char init_mode = *mode;
  if (init_mode != 'r' && init_mode != 'w' && init_mode != 'a') {
    // TODO: Errno
    return NULL;
  }

  int flags = parse_mode_flags(mode);
  
  int new_mode = flags & O_CREAT ? 0666 : 0; 

  int fd = open(filename, flags, new_mode);

  return __cimakestream(fd, _IOFBF);
}

int parse_mode_flags(const char *mode_str) {
  int flags;

  if (strchr(mode_str, '+') != NULL) {
    flags = O_RDWR;
  } else if (*mode_str == 'r') {
    flags = O_RDONLY;
  } else {
    flags = O_WRONLY;
  }

  // TODO: The rest

  return flags;
}

int fclose(FILE *stream) {
  return close(stream->fd);
}