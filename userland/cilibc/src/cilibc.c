#include <stddef.h>
#include <unistd.h>

void init_stdlib(int argc, char **argv) {
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
