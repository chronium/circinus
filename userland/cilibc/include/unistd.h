#ifndef _CILIBC_UNISTD_H
#define _CILIBC_UNISTD_H 1

#if defined(__cplusplus)
extern "C" {
#endif

#include <bits/sys/types.h>

  ssize_t write(int fd, const void* buf, size_t count);
  void* brk(void* addr);

#define STDERR_FILENO 2
#define STDIN_FILENO 0
#define STDOUT_FILENO 1

#if defined(__cplusplus)
} /* extern "C" */
#endif

#endif /* _CILIBC_UNISTD_H */
