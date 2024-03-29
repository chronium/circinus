#ifndef _CILIBC_SYS_SYSCALL_H
#define _CILIBC_SYS_SYSCALL_H 1

#if defined(__cplusplus)
extern "C" {
#endif

void *syscall(void *num);
void *syscall1(void *num, void *arg1);
void *syscall2(void *num, void *arg1, void *arg2);
void *syscall3(void *num, void *arg1, void *arg2, void *arg3);
void *syscall4(void *num, void *arg1, void *arg2, void *arg3, void *arg4);
void *syscall5(void *num, void *arg1, void *arg2, void *arg3, void *arg4,
               void *arg5);

#define SYS_EXIT -1
#define SYS_WRITE 1
#define SYS_READ 2
#define SYS_OPEN 4
#define SYS_CLOSE 8
#define SYS_CLOCK_GETTIME 12
#define SYS_CLOCK_NANOSLEEP 13
#define SYS_BRK 128

#if defined(__cplusplus)
} /* extern "C" */
#endif

#endif /* _CILIBC_SYS_SYSCALL_H */