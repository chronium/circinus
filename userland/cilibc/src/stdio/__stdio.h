#ifndef _CILIBC_STDIO___STDIO_H
#define _CILIBC_STDIO___STDIO_H 1

#if defined(__cplusplus)
extern "C" {
#endif

#include <stdio.h>
#include <stdlib.h>

FILE *__cimakestream(int fd, int flags);

FILE *__cimakebuf(FILE *stream);

#if defined(__cplusplus)
} // extern "C"
#endif

#endif /*_CILIBC_STDIO___STDIO_H */
