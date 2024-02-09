#include <unistd.h>
#include "syscall.h"

ssize_t read(int fd, void *dst, size_t size) {
  return (ssize_t)syscall3((void *)SYS_READ, (void *)fd, dst, (void *)size);
}