#include <stddef.h>
#include <unistd.h>
#include <stdio.h>
#include <stdlib.h>

FILE *__sF[3];

void init_stdlib(int argc, char **argv) {
  FILE* stdin_file = (FILE*)malloc(sizeof(FILE));
  FILE* stdout_file = (FILE*)malloc(sizeof(FILE));
  FILE* stderr_file = (FILE*)malloc(sizeof(FILE));

  stdin_file->fd = STDIN_FILENO;
  stdin_file->buf = (char*)malloc(BUFSIZ);

  stdout_file->fd = STDOUT_FILENO;
  stdout_file->buf = (char*)malloc(BUFSIZ);

  stderr_file->fd = STDERR_FILENO;
  stderr_file->buf = (char*)malloc(BUFSIZ);

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
