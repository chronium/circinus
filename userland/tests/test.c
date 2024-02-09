#include <stdio.h>
#include <unistd.h>

int main(int argc, char* argv[]) {
  puts("Hello World!\nFrom the libc!\n");

  printf("Hello %s\n", "printf");
  printf("Numbers! %i\n", 1234);

  printf("\n");

  printf("argc=%d\n", argc); 
  printf("argv[0]=%s\n", argv[0]);

  printf("argv[0][0]=%c\n", argv[0][0]);

  FILE *f = fopen("/ext2/test.txt", "r");

  fclose(f);
  
  return 0;
}