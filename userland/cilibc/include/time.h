#ifndef _CILIBC_TIME_H
#define _CILIBC_TIME_H 1

#include <bits/null.h>
#include <sys/time.h>

struct timespec {
  time_t tv_sec;
  long tv_nsec;
};

struct tm {
  int tm_sec;   /* Seconds [0,60]. */
  int tm_min;   /* Minutes [0,59]. */
  int tm_hour;  /* Hour [0,23]. */
  int tm_mday;  /* Day of month [1,31]. */
  int tm_mon;   /* Month of year [0,11]. */
  int tm_year;  /* Years since 1900. */
  int tm_wday;  /* Day of week [0,6] (Sunday =0). */
  int tm_yday;  /* Day of year [0,365]. */
  int tm_isdst; /* Daylight Savings flag. */
};

int clock_gettime(clockid_t, struct timespec *);
int clock_nanosleep(clockid_t, int, const struct timespec *, struct timespec *);

int nanosleep(const struct timespec *, struct timespec *);

struct tm *localtime(const time_t *);

time_t time(time_t *);

#define CLOCK_REALTIME 0
#define CLOCK_MONOTONIC 1

#endif /* _CILIBC_TIME_H */