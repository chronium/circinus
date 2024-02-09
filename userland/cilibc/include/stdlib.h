#ifndef _CILIBC_STDLIB_H
#define _CILIBC_STDLIB_H 1

#if defined(__cplusplus)
extern "C" {
#endif

#define EXIT_SUCCESS 0
#define EXIT_FAILURE 1

// TODO: wchar_t, div_t, ldiv_t, lldiv_t

#include <bits/stddef.h>

void *malloc(size_t);
void free(void *);

void *realloc(void *, size_t);

[[noreturn]] void exit(int);

#if defined(__cplusplus)
} /* extern "C" */
#endif

#endif /* _CILIBC_STDLIB_H */
