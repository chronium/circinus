#include <stdio.h>

int main(int argc, char **argv)
{
  puts("Hello World!\n");

  printf("Hello %s!\n", "World");
  printf("argc = %d\n", argc);

  for (int i = 0; i < argc; i++)
    printf("argv[%d] = %s\n", i, argv[i]);

  return 0;
}
