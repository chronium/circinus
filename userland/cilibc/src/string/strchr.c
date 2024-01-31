#include <string.h>

char *strchr(const char *s, int c) {
  unsigned char cc = (unsigned char)c;

  while (*s != 0)
    if (*s++ == cc)
      return (char *)s;
  
  return NULL;
}