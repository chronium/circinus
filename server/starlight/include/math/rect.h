#ifndef _STARLIGHT_MATH_RECT_H
#define _STARLIGHT_MATH_RECT_H 1

#if defined(__cplusplus)
extern "C"
#endif

#include <stdint.h>
#include <sys/types.h>

    typedef struct framebuffer framebuffer_t;

typedef struct {
  ssize_t x;
  ssize_t y;

  ssize_t width;
  ssize_t height;
} rect_t;

rect_t *rect_resize(rect_t *, ssize_t x, ssize_t y);
rect_t *rect_offset(rect_t *, ssize_t x, ssize_t y);

void rect_center(rect_t *src, rect_t *dst, ssize_t *x, ssize_t *y);

void outline_rect(framebuffer_t *, rect_t *, uint32_t);
void outline_split_rect_wide(framebuffer_t *, rect_t *, ssize_t width, uint32_t,
                             uint32_t);

void fill_rect(framebuffer_t *, rect_t *, uint32_t);

void trishade_rect(framebuffer_t *, rect_t *, ssize_t width, uint32_t light,
                   uint32_t base, uint32_t dark);

#if defined(__cplusplus)
} /* extern "C" */
#endif

#endif /* _STARLIGHT_MATH_RECT_H */