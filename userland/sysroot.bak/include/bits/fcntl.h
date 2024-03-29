#ifndef _BITS_FCNTL_H
#define _BITS_FCNTL_H

#ifdef __cplusplus
extern "C"
{
#endif

    int open(const char *filename, int flags, ...);
    int fcntl(int fd, int cmd, ...);

#ifdef __cplusplus
} // extern "C"
#endif

#endif /* _BITS_FCNTL_H */