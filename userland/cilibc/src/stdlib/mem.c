#include <stddef.h>
#include <stdlib.h>
#include "../alloc/liballoc.h"

void *malloc(size_t size) {
  return kmalloc(size);
}

void free(void *ptr) {
  kfree(ptr);
}

void *realloc(void* ptr, size_t size) {
  return krealloc(ptr, size);
}
