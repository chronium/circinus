#ifndef _CILIBC_SYS_TIME_H
#define _CILIBC_SYS_TIME_H 1

#if defined(__cplusplus)
#extern "C"
#endif

#include <bits/sys/time.h>

struct timeval {
  time_t tv_sec;
  suseconds_t tv_usec;
};

#if defined(__cplusplus)
} /* extern "C" */
#endif


#endif /* _CILIBC_SYS_TIME_H */