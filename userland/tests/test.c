#include <stdio.h>
#include <stdlib.h>

int main(int argc, char* argv[]) {
  puts("Hello World!\nFrom the libc!\n");

  printf("Hello %s\n", "printf");
  printf("Numbers! %i\n", 1234);

  malloc(0);
  malloc(1);

  return 0;
}
