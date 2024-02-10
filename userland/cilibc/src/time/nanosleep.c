#include <time.h>

int nanosleep(const struct timespec *rqtp, struct timespec *rmtp) {
  return clock_nanosleep(CLOCK_REALTIME, 0, rqtp, rmtp);
}