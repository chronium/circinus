#include <sys/syscall.h>
#include <unistd.h>
#include <stdint.h>

ssize_t write(int fd, const void* buf, size_t count) {
  return (ssize_t)syscall3((void*)SYS_WRITE, (void*)(uintptr_t)fd, (void*)buf, (void*)(uintptr_t)count);
}

