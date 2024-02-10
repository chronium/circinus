#include <time.h>
#include "syscall.h"

int clock_nanosleep(clockid_t clock, int flags, const struct timespec *rqtp, struct timespec *rmtp) {
  return (int)syscall4((void*)SYS_CLOCK_NANOSLEEP, (void *)clock, (void *)flags, (void *)rqtp, (void *)rmtp);
}
