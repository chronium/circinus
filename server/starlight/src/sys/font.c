#include "framebuffer.h"
#include <fcntl.h>
#include <font.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

font_t *font_open(const char *device) {
  // TODO: Use stat

  font_t *font = (font_t *)malloc(sizeof(font_t));

  void *buf = malloc(256 * 16 + 4 * sizeof(uint64_t));

  int fd = open(device, O_RDONLY);

  read(fd, buf, 256 * 16 + 4 * sizeof(uint64_t));

  font->width = ((uint64_t *)buf)[0];
  font->height = ((uint64_t *)buf)[1];
  font->stride = ((uint64_t *)buf)[2];
  font->max_glyph = ((uint64_t *)buf)[3];
  font->data = buf + 4 * sizeof(uint64_t);

  return font;
}

void blit_char(font_t *font, unsigned char c, ssize_t x, ssize_t y,
               uint32_t color, framebuffer_t *fb) {
  if (c > font->max_glyph)
    return;

  size_t offset = c * font->stride;

  for (size_t yy = 0; yy < font->height; yy++) {
    for (size_t xx = 0; xx < font->width; xx++) {
      ssize_t xp = x + xx;
      ssize_t yp = y + yy;

      if ((font->data[yy + offset] >> xx) & 1)
        fb_putpix(fb, xp, yp, color);
    }
  }
}

void blit_string(font_t *font, char *s, ssize_t x, ssize_t y,
               uint32_t color, framebuffer_t *fb) {
  ssize_t xoff = 0;
  while (*s != 0) {
    blit_char(font, *s++, x + xoff, y, color, fb);

    xoff += font->width;
  }
}