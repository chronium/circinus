#include <unistd.h>
#include "syscall.h"

void *brk(void *addr) {
  return syscall1((void*)SYS_BRK, addr);
}
