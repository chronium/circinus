#ifndef _CILIBC_FCNTL_H
#define _CILIBC_FCNTL_H

#include <starg.h>
#include <sys/types.h>

#define F_DUPFD 0

#define F_GETFD 1

#define F_SETFD 2

#define F_GETFL 3

#define F_SETFL 4

#define F_GETLK 5

#define F_SETLK 6

#define F_SETLKW 7

#define F_RDLCK 0

#define F_WRLCK 1

#define F_UNLCK 2

#define F_ULOCK 0

#define F_LOCK 1

#define F_TLOCK 2

#define F_TEST 3

#define O_RDONLY 0

#define O_WRONLY 1

#define O_RDWR 2

#define O_CREAT 64

#define O_EXCL 128

#define O_NOCTTY 256

#define O_TRUNC 512

#define O_APPEND 1024

#define O_NONBLOCK 2048

#define O_DIRECTORY 65536

#define O_CLOEXEC 524288

#define FD_CLOEXEC 524288

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

int sys_open(const char *path, int oflag, mode_t mode);

int sys_fcntl(int fd, int cmd, unsigned long long arg);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* _CILIBC_FCNTL_H */

#include <bits/fcntl.h>
