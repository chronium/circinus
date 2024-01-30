#ifndef _CILIBC_STRING_H
#define _CILIBC_STRING_H 1

#if defined(__cplusplus)
extern "C" {
#endif

#include <stddef.h>

  size_t strlen(const char *s);
  
  void *memset(void*, int, size_t);

#if defined(__cplusplus)
} /* extern "C" */
#endif

#endif /* _CILIBC_STRING_H */