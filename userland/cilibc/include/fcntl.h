#ifndef _CILIBC_FCNTL_H
#define _CILIBC_FCNTL_H 1

#if defined(__cplusplus)
extern "C" {
#endif

// TODO: mode_t, off_t, pid_t

#define O_RDONLY    000000000
#define O_WRONLY    000000001
#define O_RDWR      000000002
#define O_CREAT     000000100
#define O_EXCL      000000200
#define O_NOCTTY    000000400
#define O_TRUNC     000001000
#define O_APPEND    000002000
#define O_NONBLOCK  000004000
#define O_DIRECTORY 000200000
#define O_CLOEXEC   002000000

  int open(const char *, int, ...);
  int close(int);

#if defined(__cplusplus)
} /* extern "C" */
#endif

#endif /* _CILIBC_FCNTL_H */