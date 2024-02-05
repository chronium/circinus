#include <stdio.h>
#include <stdlib.h>
#include <fcntl.h>
#include <unistd.h>
#include <framebuffer.h>
#include <math/rect.h>
#include <font.h>
#include <widget/sl_element.h>

//#define BASE 0xFFdb9834
//#define LIGHT 0xFFe2ad5d
//#define DARK 0xFFc1862e

#define BASE 0xFF8d8c7f
#define LIGHT 0xFFa6a595
#define DARK 0xFF7c7b70

#define TEXT 0xFF000000
#define DISABLED 0xFF6B6B6B

#define EDGE 3

void draw_button(framebuffer_t *fb, ssize_t x, ssize_t y, ssize_t width, ssize_t height) {

  fb_fillrect(fb, x, y, width, height, BASE);
  fb_fillrect(fb, x, y, EDGE, height, LIGHT);
  fb_fillrect(fb, x, y, width, EDGE, LIGHT);
  fb_fillrect(fb, x, y + height - EDGE, width, EDGE, DARK);
  fb_fillrect(fb, x + width - EDGE, y + EDGE, EDGE, height - EDGE * 2, DARK);
}

SlElement *slMakeBase(SlElement *elem, SlElementKind kind, SlDrawFunction draw) {
  elem->kind = kind;
  elem->draw = draw;

  return elem;
}

void drawWindow(framebuffer_t *fb, font_t *font, SlElement *elem) {
  if (elem->kind != SL_WINDOW) return;

  SlWindow *wnd = (SlWindow *)wnd;

  trishade_rect(fb, &elem->rect, EDGE, LIGHT, BASE, DARK);

  blit_string(font, wnd->title, elem->rect.x + EDGE + EDGE, elem->rect.y + EDGE + EDGE, TEXT, fb);
}

SlWindow *makeWindow(char *title) {
  SlWindow *wnd = (SlWindow *)malloc(sizeof(SlWindow));
  wnd->title = title;

  slMakeBase(&wnd->base, SL_WINDOW, drawWindow);

  return wnd;
}

SlElement *_slResize(SlElement *elem, ssize_t width, ssize_t height) {
  elem->rect.width = width;
  elem->rect.height = height;

  return elem;
}

SlElement *_slReposition(SlElement *elem, ssize_t x, ssize_t y) {
  elem->rect.x = x;
  elem->rect.y = y;
  
  return elem;
}

int main(int argc, char *argv[]) {
  int fb_fd = open(FB_DEVICE, O_RDWR);

  if (fb_fd < 0) {
    printf("Could not open " FB_DEVICE "\n");
    return EXIT_FAILURE;
  }

  framebuffer_t *fb = fb_open(FB_DEVICE);

  printf("width = %d, height = %d\n", fb->info.width, fb->info.height);

  font_t *font = font_open(BIZCAT);

  printf("font_width = %d, font_height = %d, stride = %d, max_glyph = %d\n", font->width, font->height, font->stride, font->max_glyph);

  draw_button(fb, 100, 100, 60, 40);
  draw_button(fb, 250, 120, 100, 30);
  draw_button(fb, 120, 250, 120, 60);
 
  SlWindow *wnd = makeWindow("Test Window");
  slReposition(wnd, 300, 300);
  slResize(wnd, 320, 200);

  rect_t rect = wnd->base.rect;
  ssize_t btn_w = 80;
  ssize_t btn_h = 30;
  rect_t ok = { rect.x + rect.width - 16 - btn_w, rect.y + rect.height - 16 - btn_h, btn_w, btn_h };
  rect_t cancel = { rect.x + rect.width - 16 - btn_w - 16 - btn_w, rect.y + rect.height - 16 - btn_h, btn_w, btn_h };

  slDraw(wnd, fb, font);

  trishade_rect(fb, &ok, 3, LIGHT, BASE, DARK);
  trishade_rect(fb, &cancel, 3, DARK, BASE, LIGHT);

  blit_char(font, 'A', 100, 100, 0xFF0000FF, fb);
  
  blit_string(font, "Hello World!", 100, 125, 0xFF0000FF, fb);

  blit_string(font, "Cancel", cancel.x + 18, cancel.y + 7, DISABLED, fb);
  blit_string(font, "Ok", ok.x + 32, ok.y + 7, TEXT, fb);

  fb_swap(fb);

  return 0;
}