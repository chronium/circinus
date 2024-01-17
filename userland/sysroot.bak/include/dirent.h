#ifndef _CILIBC_DIRENT_H
#define _CILIBC_DIRENT_H

#include <sys/types.h>

typedef struct DIR DIR;

typedef struct dirent {
  ino_t d_ino;
  off_t d_off;
  unsigned short d_reclen;
  unsigned char d_type;
  char d_name[256];
} dirent;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

struct DIR *opendir(const char *path);

int closedir(struct DIR *dir);

struct dirent *readdir(struct DIR *dir);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* _CILIBC_DIRENT_H */

#include <bits/dirent.h>
