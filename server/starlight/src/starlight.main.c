#include <fcntl.h>
#include <font.h>
#include <framebuffer.h>
#include <math/rect.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <widget/sl_element.h>

// #define BASE 0xFFdb9834
// #define LIGHT 0xFFe2ad5d
// #define DARK 0xFFc1862e

#define BASE 0xFF8d8c7f
#define LIGHT 0xFFa6a595
#define DARK 0xFF7c7b70

// #define TEXT 0xFF000000
#define TEXT 0xFFFFFFFF
#define DISABLED 0xFF6B6B6B

#define EDGE 3

void draw_button(framebuffer_t *fb, ssize_t x, ssize_t y, ssize_t width,
                 ssize_t height) {

  fb_fillrect(fb, x, y, width, height, BASE);
  fb_fillrect(fb, x, y, EDGE, height, LIGHT);
  fb_fillrect(fb, x, y, width, EDGE, LIGHT);
  fb_fillrect(fb, x, y + height - EDGE, width, EDGE, DARK);
  fb_fillrect(fb, x + width - EDGE, y + EDGE, EDGE, height - EDGE * 2, DARK);
}

SlElement *slMakeBase(SlElement *elem, SlElementKind kind,
                      SlDrawFunction draw) {
  elem->kind = kind;
  elem->draw = draw;
  elem->children_count = 0;

  return elem;
}

void drawWindow(SlElement *elem, rect_t *parent, framebuffer_t *fb,
                font_t *font) {
  if (elem->kind != SL_WINDOW)
    return;
  SlWindow *wnd = (SlWindow *)elem;

  rect_t dst = {parent->x + elem->rect.x, parent->y + elem->rect.y,
                elem->rect.width, elem->rect.height};

  trishade_rect(fb, &dst, EDGE, LIGHT, BASE, DARK);

  blit_string(font, wnd->title, dst.x + EDGE + EDGE, dst.y + EDGE + EDGE, TEXT,
              fb);
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

void _addChild(SlElement *parent, SlElement *child) {
  if (parent->children == NULL) {
    parent->children = malloc(sizeof(SlElement *));
  }

  size_t new_count = parent->children_count + 1;
  parent->children = realloc(parent->children, new_count * sizeof(SlElement *));

  parent->children[parent->children_count] = child;
  parent->children_count = new_count;
}

void drawLabel(SlElement *elem, rect_t *parent, framebuffer_t *fb,
               font_t *font) {
  if (elem->kind != SL_LABEL)
    return;
  SlLabel *lbl = (SlLabel *)elem;

  rect_t dst = {parent->x + elem->rect.x, parent->y + elem->rect.y,
                elem->rect.width, elem->rect.height};

  blit_string(font, lbl->text, dst.x, dst.y, TEXT, fb);
}

SlLabel *makeLabel(char *text, font_t *font) {
  SlLabel *lbl = (SlLabel *)malloc(sizeof(SlLabel));
  lbl->text = text;

  slMakeBase(&lbl->base, SL_LABEL, drawLabel);
  slResize(lbl, font->width * strlen(text), font->height);

  return lbl;
}

void _slDraw(SlElement *elem, rect_t *parent, framebuffer_t *fb, font_t *font) {
  elem->draw(elem, parent, fb, font);

  for (int i = 0; i < elem->children_count; i++)
    _slDraw(elem->children[i], &elem->rect, fb, font);
}

void drawButton(SlElement *elem, rect_t *parent, framebuffer_t *fb,
                font_t *font) {
  if (elem->kind != SL_BUTTON)
    return;
  SlButton *btn = (SlButton *)elem;

  rect_t dst = {parent->x + elem->rect.x, parent->y + elem->rect.y,
                elem->rect.width, elem->rect.height};

  if (!btn->disabled) {
    trishade_rect(fb, &dst, EDGE, LIGHT, BASE, DARK);
    blit_string(font, btn->text, dst.x + EDGE * 2, dst.y + EDGE * 2, TEXT, fb);
  } else {
    trishade_rect(fb, &dst, EDGE, DARK, BASE, LIGHT);
    blit_string(font, btn->text, dst.x + EDGE * 2, dst.y + EDGE * 2, DISABLED,
                fb);
  }
}

struct MousePacket {
  int16_t dX;
  int16_t dY;
  int8_t buttons;
};

SlButton *makeButton(char *text, font_t *font) {
  SlButton *btn = (SlButton *)malloc(sizeof(SlButton));
  btn->text = text;
  btn->disabled = 0;

  slMakeBase(&btn->base, SL_BUTTON, drawButton);
  slResize(btn, font->width * strlen(text) + EDGE * 2, font->height + EDGE * 4);

  return btn;
}

int main(int argc, char *argv[]) {
  framebuffer_t *fb = fb_open(FB_DEVICE);
  printf("width = %d, height = %d\n", fb->info.width, fb->info.height);

  font_t *font = font_open(BIZCAT);
  printf("font_width = %d, font_height = %d, stride = %d, max_glyph = %d\n",
         font->width, font->height, font->stride, font->max_glyph);

  int mouse = open("/Devices/Mouse/", O_RDONLY);
  if (mouse < 1) {
    printf("Could not open /Devices/Mouse");
  }

  SlWindow *desk = makeWindow("Desktop");
  slReposition(desk, 0, 0);
  slResize(desk, fb->info.width, fb->info.height);

  SlWindow *wnd = makeWindow("Test Window");
  slReposition(wnd, 300, 300);
  slResize(wnd, 320, 200);

  addChild(desk, wnd); 

  SlLabel *lbl = makeLabel("I'm a label!", font);
  slReposition(lbl, 30, 50);

  addChild(wnd, lbl);

  SlButton *btn_enabled = makeButton("I'm a button!", font);
  SlButton *btn_disabled = makeButton("I'm disabled!", font);
  btn_disabled->disabled = 1;

  slReposition(btn_enabled, 30, 80);
  slReposition(btn_disabled, 30, 120);

  addChild(wnd, btn_enabled);
  addChild(wnd, btn_disabled);

  rect_t zero = {0, 0, 0, 0};

  struct MousePacket packet;

  SlLabel *ml = makeLabel("MOUSE!!!", font);
  ssize_t mx, my = 0;

  while (1) {
    ssize_t size = read(mouse, &packet, sizeof(struct MousePacket));

    if (size != 0) {
      mx += packet.dX;
      my += -packet.dY;

      if (mx < 0) mx = 0;
      if (my < 0) my = 0;
      if (mx >= fb->info.width) mx = fb->info.width;
      if (my >= fb->info.height) my = fb->info.height;

      slReposition(ml, mx, my);
    }
    
    slDraw(desk, &zero, fb, font);
    slDraw(ml, &zero, fb, font);

    fb_swap(fb);
  }

  return 0;
}