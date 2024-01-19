#include <stdio.h>
#include <unistd.h>

int fputc(int c, FILE *stream) {
  char b[1];
  b[0] = c;
  write(stream->fd, b, 1);
}
