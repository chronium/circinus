#ifndef _CILIBC_STDLIB_H
#define _CILIBC_STDLIB_H

#include <stddef.h>
#include <alloca.h>

#define EXIT_SUCCESS 0

#define EXIT_FAILURE 1

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

double strtod(const char *s, char **endptr);

void *malloc(size_t size);

void *realloc(void *ptr, size_t size);

void free(void *ptr);

void _fini(void);

void exit(int status);

char *getenv(const char *name);

int setenv(const char *key, const char *value, int overwrite);

void abort(void);

void *calloc(size_t nelem, size_t elsize);

int abs(int i);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* _CILIBC_STDLIB_H */

#include <bits/stdlib.h>
