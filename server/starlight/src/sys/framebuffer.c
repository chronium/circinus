#include <framebuffer.h>
#include <stdlib.h>
#include <fcntl.h>
#include <string.h>
#include <unistd.h>
#include <stdint.h>
#include <sys/types.h>

framebuffer_t *fb_open(const char *device) {
  framebuffer_t *fb = (framebuffer_t *)malloc(sizeof(framebuffer_t));
  memset(fb, 0, sizeof(framebuffer_t));

  int fd = open(device, O_RDWR); 

  fb->fd = fd;
  read(fd, &fb->info, sizeof(fb_info_t));
  fb->bufsiz = fb->info.width * fb->info.height * 4;
  fb->buf = (uint8_t *)malloc(fb->bufsiz);

  return fb;
}

void fb_swap(framebuffer_t *fb) {
  write(fb->fd, fb->buf, fb->bufsiz);
}

void fb_putpix(framebuffer_t *fb, ssize_t x, ssize_t y, uint32_t color) {
  if (x < 0 || y < 0 || x >= fb->info.width || y >= fb->info.height) return;

  ((uint32_t *)fb->buf)[INDEX(*fb, x, y)] = color;
}

void fb_line(framebuffer_t *fb, ssize_t x1, ssize_t y1, ssize_t x2, ssize_t y2, uint32_t color) {
    int dx = labs(x2 - x1), sx = x1 < x2 ? 1 : -1;
    int dy = -labs(y2 - y1), sy = y1 < y2 ? 1 : -1; 
    int err = dx + dy, e2; /* error value e_xy */
 
    while (1) {
        fb_putpix(fb, x1, y1, color);

        if (x1 == x2 && y1 == y2) break;
        e2 = 2 * err;
        if (e2 >= dy) { err += dy; x1 += sx; } /* e_xy+e_x > 0 */
        if (e2 <= dx) { err += dx; y1 += sy; } /* e_xy+e_y < 0 */
    }
}

void fb_fillrect(framebuffer_t *fb, ssize_t x, ssize_t y, ssize_t width, ssize_t height, uint32_t color) {
  for (int yy = y; yy < y + height; yy++) {
    for (int xx = x; xx < x + width; xx++) {
      fb_putpix(fb, xx, yy, color);
    }
  }
}
