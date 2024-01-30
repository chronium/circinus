#include <string.h>

void *memset(void *s, int c, size_t n) {
  unsigned char *p = s;
  unsigned char value = (unsigned char)c;

  for (size_t i = 0; i < n; i++)
    p[i] = value;

  return s;
}