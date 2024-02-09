#ifndef _CILIBC_BITS_STDDEF_H
#define _CILIBC_BITS_STDDEF_H 1

#if defined(__cplusplus)
extern "C" {
#endif
  
#ifndef __CILIBC_SIZE_TYPE__
#define __CILIBC_SIZE_TYPE__ 1

#ifndef __SIZE_TYPE__
#define __SIZE_TYPE__ unsigned long int
#endif

typedef __SIZE_TYPE__ size_t;

#endif

#ifndef __CILIBC_NULL__
#define __CILIBC_NULL__ 1
#define NULL (void*)0
#endif

#if defined(__cplusplus)
} /* extern "C" */
#endif

#endif /* _CILIBC_BITS_STDDEF_H */
