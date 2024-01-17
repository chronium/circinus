#ifndef _CILIBC_UNISTD_H
#define _CILIBC_UNISTD_H 1

#if defined(__cplusplus)
extern "C" {
#endif

#include <stddef.h>

  ssize_t write(int fd, const void* buf, size_t count);

#if defined(__cplusplus)
} /* extern "C" */
#endif

#endif /* _CILIBC_UNISTD_H */
