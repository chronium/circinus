#include <stdio.h>
#include <string.h>
#include <unistd.h>

int puts(const char *s) {
  return write(1, s, strlen(s));
}
