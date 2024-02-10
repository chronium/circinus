#ifndef _CILIBC_BITS_SYS_TIME_H
#define _CILIBC_BITS_SYS_TIME_H

#if defined(__cplusplus)
extern "C" {
#endif

#ifndef __CILIBC_TIME_TYPE__
#define __CILIBC_TIME_TYPE__ 1

#ifndef __TIME_TYPE__
#define __TIME_TYPE__ unsigned long long
#endif

typedef __TIME_TYPE__ time_t;

#endif

#ifndef __CILIBC_SUSECONDS_TYPE__
#define __CILIBC_SUSECONDS_TYPE__ 1

#ifndef __SUSECONDS_TYPE__
#define __SUSECONDS_TYPE__ unsigned long long
#endif

typedef __SUSECONDS_TYPE__ suseconds_t;

#endif

#ifndef __CILIBC_CLOCKID_TYPE__
#define __CILIBC_CLOCKID_TYPE__ 1

#ifndef __CLOCKID_TYPE__
#define __CLOCKID_TYPE__ int
#endif

typedef __CLOCKID_TYPE__ clockid_t;

#endif

#if defined(__cplusplus)
} /* extern "C" */
#endif

#endif /* _CILIBC_BITS_SYS_TIME_H */