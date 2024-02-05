#include <stdint.h>
#include <unistd.h>
#ifndef _STARLIGHT_FONT_H
#define _STARLIGHT_FONT_H 1

#if defined(__cplusplus)
extern "C" {
#endif

typedef struct framebuffer framebuffer_t;

typedef struct font {
  uint64_t width;
  uint64_t height;
  uint64_t stride;
  uint64_t max_glyph;
  uint8_t *data;
} font_t;

#define BIZCAT "/Devices/Bizcat"

font_t *font_open(const char *);

void blit_char(font_t *, unsigned char, ssize_t, ssize_t, uint32_t, framebuffer_t *);
void blit_string(font_t *, char *, ssize_t, ssize_t, uint32_t, framebuffer_t *);

#if defined(__cplusplus)
} /* extern "C" */
#endif

#endif /* _STARLIGHT_FONT_H */
