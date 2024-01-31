#include "__stdio.h"
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

int isatty(int fd) {
  return fd == STDIN_FILENO || fd == STDOUT_FILENO || fd == STDERR_FILENO;
}

FILE *__cimakestream(int fd, int flags) {
  FILE *f = (FILE *)malloc(sizeof(FILE));
  f->flags = flags;
  f->fd = fd;

  return __cimakebuf(f);
}

FILE *__cimakebuf(FILE *stream) {
  if (stream->flags & _IONBF) {
    stream->buf = malloc(1);
    stream->bufsiz = 1;

    return stream;
  }

  stream->buf = malloc(BUFSIZ);
  stream->bufsiz = BUFSIZ;

  if (isatty(stream->fd))
    stream->flags = (stream->flags & ~(_IONBF | _IOFBF | _IOLBF)) | _IOLBF;
  else
    stream->flags = (stream->flags & ~(_IONBF | _IOFBF | _IOLBF)) | _IOFBF; 

  return stream; 
}