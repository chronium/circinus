#include <time.h>
#include "syscall.h"

int clock_gettime(clockid_t clock, struct timespec *ts) {
  return (int)syscall2((void *)SYS_CLOCK_GETTIME, (void *)clock, (void *)ts);
}