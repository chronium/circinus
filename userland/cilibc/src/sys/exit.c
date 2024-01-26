#include "syscall.h"
#include <stdint.h>
#include <stdlib.h>

[[ noreturn ]] void exit(int exit_code) {
  extern void _fini();
  _fini();
  (void) syscall1((void*)SYS_EXIT, (void*)(uintptr_t)exit_code);
  for (;;) {}
}
