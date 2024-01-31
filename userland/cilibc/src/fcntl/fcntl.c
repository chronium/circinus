#include <fcntl.h>
#include "../sys/syscall.h"

int open(const char *path, int oflag, ...) {
  return (int)syscall3((void *)SYS_OPEN, (void *)path, (void *)oflag, (void *)0);
}

int close(int fd) {
  return (int)syscall1((void *)SYS_CLOSE, (void *)fd);
}