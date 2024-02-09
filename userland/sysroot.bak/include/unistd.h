#ifndef _CILIBC_STDIO_H_
#define _CILIBC_STDIO_H_

#include <stddef.h>
#include <stdint.h>
#include <sys/types.h>

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

int execv(const char *path, char *const *argv);

int execve(const char *path, char *const *argv, char *const *envp);

char *getcwd(char *buf, size_t size);

int chdir(const char *path);

pid_t fork(void);

int execvp(const char *file, char *const *argv);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* _CILIBC_STDIO_H_ */

#include <bits/fcntl.h>
#include <bits/unistd.h>
