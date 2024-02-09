#ifndef _CILIBC_INTTYPES_H
#define _CILIBC_INTTYPES_H

#include <stdint.h>

typedef struct {
  intmax_t quot;
  intmax_t rem;
} imaxdiv_t;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

intmax_t imaxabs(intmax_t i);

imaxdiv_t imaxdiv(intmax_t i, intmax_t j);

intmax_t strtoimax(const char *s, char **endptr, int base);

uintmax_t strtoumax(const char *s, char **endptr, int base);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* _CILIBC_INTTYPES_H */

#include <bits/inttypes.h>
