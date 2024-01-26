#include <stdio.h>
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
