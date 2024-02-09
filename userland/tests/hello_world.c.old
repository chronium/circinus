#include "sys.h"
#include "lib.h"

void main()
{
  const char *str = "hello world!\n";
  uint64_t len = strlen(str);
  sys$write(STDOUT, (void *)str, len);
}