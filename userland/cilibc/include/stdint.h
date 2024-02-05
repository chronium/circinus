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

#if defined(__cplusplus)
} /* extern "C" */
#endif

#endif /* _CILIBC_STDINT_H */