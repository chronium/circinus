#ifndef _CILIBC_STDINT_H
#define _CILIBC_STDINT_H 1

#if defined(__cplusplus)
extern "C" {
#endif

typedef unsigned long int uintptr_t;
typedef long int intptr_t;

#ifndef __CILIBC__UINT8_TYPE__
#define __CILIBC__UINT8_TYPE__ 1

#ifndef __UINT8_TYPE__
#define __UINT8_TYPE__ unsigned char
#endif

typedef __UINT8_TYPE__ uint8_t;

#endif

#ifndef __CILIBC__UINT16_TYPE__
#define __CILIBC__UINT16_TYPE__ 1

#ifndef __UINT16_TYPE__
#define __UINT16_TYPE__ unsigned short
#endif

typedef __UINT16_TYPE__ uint16_t;

#endif

#ifndef __CILIBC__UINT32_TYPE__
#define __CILIBC__UINT32_TYPE__ 1

#ifndef __UINT32_TYPE__
#define __UINT32_TYPE__ unsigned int
#endif

typedef __UINT32_TYPE__ uint32_t;

#endif

#ifndef __CILIBC__UINT64_TYPE__
#define __CILIBC__UINT64_TYPE__ 1

#ifndef __UINT64_TYPE__
#define __UINT64_TYPE__ unsigned long long
#endif

typedef __UINT64_TYPE__ uint64_t;

#endif

#ifndef __CILIBC__INT8_TYPE__
#define __CILIBC__INT8_TYPE__ 1

#ifndef __INT8_TYPE__
#define __INT8_TYPE__ char
#endif

typedef __INT8_TYPE__ int8_t;

#endif

#ifndef __CILIBC__INT16_TYPE__
#define __CILIBC__INT16_TYPE__ 1

#ifndef __INT16_TYPE__
#define __INT16_TYPE__ short
#endif

typedef __INT16_TYPE__ int16_t;

#endif

#ifndef __CILIBC__INT32_TYPE__
#define __CILIBC__INT32_TYPE__ 1

#ifndef __INT32_TYPE__
#define __INT32_TYPE__ int
#endif

typedef __INT32_TYPE__ int32_t;

#endif

#ifndef __CILIBC__INT64_TYPE__
#define __CILIBC__INT64_TYPE__ 1

#ifndef __INT64_TYPE__
#define __INT64_TYPE__ long long
#endif

typedef __INT64_TYPE__ int64_t;

#endif

#if defined(__cplusplus)
} /* extern "C" */
#endif

#endif /* _CILIBC_STDINT_H */
