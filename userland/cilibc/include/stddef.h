#ifndef _CILIBC_STDDEF_H
#define _CILIBC_STDDEF_H 1

#if defined(__cplusplus)
extern "C" {
#endif

  // TODO: wchar_t

#include <bits/stddef.h>

#define NULL 0
#define offsetof(type, member) ((size_t)(&((type *)0)->member))

#ifndef __CILIBC_PTRDIFF_TYPE__
#define __CILIBC_PTRDIFF_TYPE__ 1

#ifndef __PTRDIFF_TYPE__
#define __PTRDIFF_TYPE__ long int
#endif

typedef __PTRDIFF_TYPE__ ptrdiff_t;

#endif

#if defined(__cplusplus)
} /* extern "C" */
#endif

#endif /* _CILIBC_STDDEF_H */
