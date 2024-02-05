#include <stdio.h>
#include <stdlib.h>
#include <stddef.h>

int main(int argc, char* argv[]) {
  puts("Hello World!\nFrom the libc!\n");

  printf("Hello %s\n", "printf");
  printf("Numbers! %i\n", 12345);

  return 0;
}