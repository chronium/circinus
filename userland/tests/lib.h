#ifndef _LIB_H
#define _LIB_H

#include "stdint.h"

uint64_t strlen(const char *s)
{
  uint64_t len;
  while (*s++)
    len++;
  return len;
}

#endif