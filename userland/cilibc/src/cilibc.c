#include <stddef.h>
#include <unistd.h>
#include <stdio.h>
#include <stdlib.h>

#include "stdio/__stdio.h"

FILE *__sF[3];

void init_stdlib(int argc, char **argv) {
  FILE *stdin_file = __cimakebuf(STDIN_FILENO, _IOLBF, malloc(BUFSIZ));
  FILE *stdout_file = __cimakebuf(STDOUT_FILENO, _IOLBF, malloc(BUFSIZ));
  FILE *stderr_file = __cimakebuf(STDERR_FILENO, _IONBF, NULL);

  __sF[0] = stdin_file;
  __sF[1] = stdout_file;
  __sF[2] = stderr_file;
}

int liballoc_lock() {
  return 0;
}

int liballoc_unlock() {
  return 0;
}

static void *last_brk = NULL;

void* liballoc_alloc(size_t pages) {
  if (last_brk == NULL)
    last_brk = brk(0);
  last_brk = brk(last_brk + pages * 4096);

  return last_brk - pages * 4096;
}

int liballoc_free(void* ptr, size_t pages) {
  return 0;
}
