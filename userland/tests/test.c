#include <stdio.h>
#include <stdlib.h>
#include <stddef.h>
#include <string.h>
#include <unistd.h>

int main(int argc, char* argv[]) {
  puts("Hello World!\nFrom the libc!\n");

  printf("Hello %s\n", "printf");
  printf("Numbers! %i\n", 1234);

  printf("\n");

  printf("argc=%d\n", argc); 
  printf("argv[0]=%s\n", argv[0]);
  printf("argv[1]=%s\n", argv[1]);
  
  return 0;
}