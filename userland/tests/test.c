#include <fcntl.h>
#include <stdio.h>
#include <unistd.h>
#include <time.h>

int main(int argc, char *argv[]) {
  puts("Hello World!\nFrom the libc!\n");

  printf("Hello %s\n", "printf");
  printf("Numbers! %i\n", 1234);

  printf("\n");

  printf("argc=%d\n", argc);
  printf("argv[0]=%s\n", argv[0]);

  printf("argv[0][0]=%c\n", argv[0][0]);

  FILE *f = fopen("/ext2/test.txt", "r");

  fclose(f);

  struct timespec ts;
  clock_gettime(CLOCK_MONOTONIC, &ts);

  printf("clock_monotonic seconds: %d\n", ts.tv_sec);

  struct timespec rqtp;
  rqtp.tv_sec = 10;
  rqtp.tv_nsec = 0;
  nanosleep(&rqtp, NULL);

  clock_gettime(CLOCK_REALTIME, &ts);

  printf("clock_monotonic seconds: %d\n", ts.tv_sec);

  return 0;
}