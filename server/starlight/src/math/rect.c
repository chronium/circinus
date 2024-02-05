#include <framebuffer.h>
#include <math/rect.h>

void rect_center(rect_t *src, rect_t *dst, ssize_t *x, ssize_t *y) {
  ssize_t center_x = (dst->x + dst->width) / 2;
  ssize_t center_y = (dst->y + dst->height) / 2;

  ssize_t half_width = src->width / 2;
  ssize_t half_height = src->height / 2;

  *x = center_x - half_width;
  *y = center_y - half_height;
}

void outline_rect(framebuffer_t *fb, rect_t *rect, uint32_t color) {
  fb_line(fb, rect->x, rect->y, rect->x + rect->width, rect->y, color);
  fb_line(fb, rect->x, rect->y, rect->x, rect->y + rect->height, color);
  fb_line(fb, rect->x, rect->y + rect->height, rect->x + rect->width,
          rect->y + rect->height, color);
  fb_line(fb, rect->x + rect->width, rect->y, rect->x + rect->width,
          rect->y + rect->height, color);
}

void fill_rect(framebuffer_t *fb, rect_t *rect, uint32_t color) {
  fb_fillrect(fb, rect->x, rect->y, rect->width, rect->height, color);
}

void outline_split_rect_wide(framebuffer_t *fb, rect_t *rect, ssize_t width,
                             uint32_t light, uint32_t dark) {
  fb_fillrect(fb, rect->x, rect->y, rect->width, width, light);
  fb_fillrect(fb, rect->x, rect->y, width, rect->height, light);

  fb_fillrect(fb, rect->x, rect->y - width + rect->height, rect->width, width,
              dark);
  fb_fillrect(fb, rect->x + rect->width - width, rect->y + width, width,
              rect->height - width, dark);
}

void trishade_rect(framebuffer_t *fb, rect_t *rect, ssize_t width, uint32_t light,
                   uint32_t base, uint32_t dark) {
  fill_rect(fb, rect, base);

  outline_split_rect_wide(fb, rect, width, light, dark);
}