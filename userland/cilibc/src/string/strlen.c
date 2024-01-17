#include <string.h>

size_t strlen(const char *s) {
  const char *p = s;
  while (*p++ != 0);
  return p - s;
}
