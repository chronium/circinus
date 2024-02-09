#include <fcntl.h>
#include <stdio.h>
#include <unistd.h>

#define WIDTH 1024
#define HEIGHT 768
#define BPP 4

void fill_rect(unsigned char *fb, int x, int y, int width, int height,
               unsigned int color) {
  for (int py = y; py < y + height; py++) {
    for (int px = x; px < x + width; px++) {
      int offset = (py * WIDTH + px) * BPP;

      fb[offset] = (color >> 24) & 0xFF;
      fb[offset + 1] = (color >> 16) & 0xFF;
      fb[offset + 2] = (color >> 8) & 0xFF;
      fb[offset + 3] = color & 0xFF;
    }
  }
}

struct fbinfo {
  long int width;
  long int height;
};

int main(int argc, char *argv[]) {
  puts("Hello World!\nFrom the libc!\n");

  printf("Hello %s\n", "printf");
  printf("Numbers! %i\n", 1234);

  printf("\n");

  printf("argc=%d\n", argc);
  printf("argv[0]=%s\n", argv[0]);

  printf("argv[0][0]=%c\n", argv[0][0]);

  FILE *f = fopen("/ext2/test.txt", "r");

  fclose(f);

  int fb = open("/Devices/Framebuffer", O_RDWR);

  struct fbinfo fbinfo;
  read(fb, &fbinfo, sizeof(struct fbinfo));

  unsigned char buf[WIDTH * HEIGHT * BPP];

  printf("width = %d, height = %d\n", fbinfo.width, fbinfo.height);

  while (1) {
    fill_rect(buf, 100, 100, 200, 150, 0xFF0000FF);

    write(fb, buf, WIDTH * HEIGHT * BPP);
  }

  close(fb);

  return 0;
}