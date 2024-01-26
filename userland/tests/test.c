#include <stdio.h>
#include <stdlib.h>
#include <stddef.h>
#include <unistd.h>

int main(int argc, char* argv[]) {
  puts("Hello World!\nFrom the libc!\n");

  printf("Hello %s\n", "printf");
  printf("Numbers! %i\n", 1234);
  
  return 0;
}
