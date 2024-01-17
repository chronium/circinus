#ifndef _CILIBC_STRING_H
#define _CILIBC_STRING_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void *memchr(const void *haystack, int needle, size_t len);

size_t strlen(const char *s);

size_t strnlen(const char *s, size_t size);

int strncmp(const char *s1, const char *s2, size_t n);

int strcmp(const char *s1, const char *s2);

size_t strspn(const char *s1, const char *s2);

size_t strcspn(const char *s1, const char *s2);

char *strpbrk(const char *s1, const char *s2);

char *strtok(char *s1, const char *delimiter);

char *strtok_r(char *s, const char *delimiter, char **lasts);

char *strchr(const char *s, int c);

void *memcpy(void *s1, const void *s2, size_t n);

void *memset(void *s, int c, size_t n);

char *strcat(char *s1, const char *s2);

char *strncat(char *s1, const char *s2, size_t n);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* _CILIBC_STRING_H */
