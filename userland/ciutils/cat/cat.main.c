#include <stdio.h>
#include <stdlib.h>

#define ANSI_RESET "\x1B[0m"
#define ANSI_BLACK "\x1B[30m"
#define ANSI_RED "\x1B[31m"
#define ANSI_GREEN "\x1B[32m"
#define ANSI_YELLOW "\x1B[33m"
#define ANSI_BLUE "\x1B[34m"
#define ANSI_MAGENTA "\x1B[35m"
#define ANSI_CYAN "\x1B[36m"
#define ANSI_WHITE "\x1B[37m"
#define ANSI_REVERSE "\x1B[7m"

int main(int argc, char **argv) {
  if (argc < 2) {
    printf("Usage: cat <file>\n");
    // TODO: Figure out why non-zero error code freezes
    return 0;
  }

  FILE *fp = fopen(argv[1], "r");

  if (fp == NULL) {
    perror("Error opening file!");
    // TODO: Error 1
    return 0;
  }

  char c;

  while ((c = fgetc(fp)) != EOF)
    putchar(c);

  if (c != '\n')
    printf(ANSI_REVERSE "%%" ANSI_RESET "\n");

  fclose(fp);

  return 0;
}