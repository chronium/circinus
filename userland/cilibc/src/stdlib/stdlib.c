#include <stdlib.h>

#define ABS(x) ({ \
    typeof(x) _x = (x); \
    _x < 0 ? -_x : _x; \
})

int abs(int i) {
  return ABS(i);
}

long labs(long l) {
  return ABS(l);
}

long long llabs(long long l) {
  return ABS(l);
}
