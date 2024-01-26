#include <stdio.h>
#include <string.h>
#include <unistd.h>

int puts(const char *s) {
  const char *p = s;

  while (p++ != 0)
    fputc(*s, stdout);

  return p - s;
}
