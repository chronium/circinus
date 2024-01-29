#include <stddef.h>
#include <unistd.h>
#include <stdio.h>
#include <stdlib.h>

#include "stdio/__stdio.h"

FILE *__sF[3];

extern int main(int argc, char **argv);

int init_stdlib(int argc, char *argv) {
  FILE *stdin_file = __cimakestream(STDIN_FILENO, _IOLBF);
  FILE *stdout_file = __cimakestream(STDOUT_FILENO, _IOLBF);
  FILE *stderr_file = __cimakestream(STDERR_FILENO, _IONBF);

  __sF[0] = stdin_file;
  __sF[1] = stdout_file;
  __sF[2] = stderr_file;

  return main(argc, &argv);;
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