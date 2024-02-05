#ifndef _STARLIGHT_WIDGET_SL_ELEMENT_H
#define _STARLIGHT_WIDGET_SL_ELEMENT_H 1

#if defined(__cplusplus)
extern "C" {
#endif

#include <stddef.h>

#include <math/rect.h>

typedef enum { SL_WINDOW, SL_LABEL, SL_BUTTON } SlElementKind;

typedef struct slElement SlElement;

typedef struct framebuffer framebuffer_t;
typedef struct font font_t;

typedef void (*SlDrawFunction)(framebuffer_t *, font_t *, SlElement *);

struct slElement {
  SlElementKind kind;

  SlDrawFunction draw;

  SlElement **children;
  size_t children_count;

  rect_t rect;
};

typedef struct {
  SlElement base;
  char *title;
} SlWindow;

typedef struct {
  SlElement base;
  char *text;
} SlLabel;

SlWindow *makeWindow(char *);
SlLabel *makeLabel(char *);

SlElement *_slResize(SlElement *, ssize_t width, ssize_t height);
SlElement *_slReposition(SlElement *, ssize_t x, ssize_t y);

SlElement *slMakeBase(SlElement *, SlElementKind, SlDrawFunction);

#define slResize(e, width, height) _slResize(&(e)->base, (width), (height))
#define slReposition(e, x, y) _slReposition(&(e)->base, (x), (y))
#define slDraw(e, fb, ft) (e)->base.draw((fb), (ft), &(e)->base);

#if defined(__cplusplus)
} /* extern "C" */
#endif

#endif /* _STARLIGHT_WIDGET_SL_ELEMENT_H */