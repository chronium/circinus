#include "../alloc/liballoc.h"
#include <stddef.h>
#include <stdlib.h>

void *malloc(size_t size) { return kmalloc(size); }

void free(void *ptr) { kfree(ptr); }

void *realloc(void *ptr, size_t size) { return krealloc(ptr, size); }

void *calloc(size_t ptr, size_t count) { return kcalloc(ptr, count); }
