#ifndef _CILIBC_BITS_SYS_TYPES_H
#define _CILIBC_BITS_SYS_TYPES_H 1

#if defined(__cplusplus)
extern "C" {
#endif

// TODO: uid_t, gid_t, pid_t

#include <bits/stddef.h>

#ifndef __CILIBC_SSIZE_TYPE__
#define __CILIBC_SSIZE_TYPE__ 1

#ifndef __SSIZE_TYPE__
#define __SSIZE_TYPE__ long int
#endif

typedef __SSIZE_TYPE__ ssize_t;

#endif

#ifndef __CILIBC_OFF_TYPE__
#define __CILIBC_OFF_TYPE__ 1

#ifndef __OFF_TYPE__
#define __OFF_TYPE__ __SSIZE_TYPE__
#endif

typedef __OFF_TYPE__ off_t;

#endif

#if defined(__cplusplus)
} /* extern "C" */
#endif

#endif /* _CILIBC_BITS_SYS_TYPES_H */
