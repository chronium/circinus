#include <stddef.h>
#include <stdlib.h>
#include "../alloc/liballoc.h"

void *malloc(size_t size) {
  return kmalloc(size);
}