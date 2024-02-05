#ifndef _STARLIGHT_FRAMEBUFFER_H
#define _STARLIGHT_FRAMEBUFFER_H 1

#if defined(__cplusplus)
extern "C" {
#endif

#include <stdint.h>
#include <stddef.h>
#include <sys/types.h>

#define FB_DEVICE "/Devices/Framebuffer"

  typedef struct {
    size_t width;
    size_t height;
  } fb_info_t;

  typedef struct framebuffer {
    fb_info_t info;
    int fd;
    uint8_t *buf;
    size_t bufsiz;
  } framebuffer_t;

framebuffer_t *fb_open(const char *);
void fb_swap(framebuffer_t *);

void fb_putpix(framebuffer_t *, ssize_t, ssize_t, uint32_t color);
void fb_line(framebuffer_t *, ssize_t, ssize_t, ssize_t, ssize_t, uint32_t color);
void fb_fillrect(framebuffer_t *, ssize_t, ssize_t, ssize_t, ssize_t, uint32_t color);

#define INDEX(fb, x, y) ((y) * (fb).info.width + (x))

#if defined(__cplusplus)
} /* extern "C" */
#endif

#endif /* _STARLIGHT_FRAMEBUFFER_H */