#ifndef _SYS_H
#define _SYS_H

#include "stdint.h"

#define STDIN 0
#define STDOUT 1
#define STDERR 2

#define SYS_WRITE 1
#define SYS_EXIT -1

typedef uint32_t filedesc_t;

void sys1(uint64_t sysno, uint64_t arg1)
{
  asm volatile(
      "movq %1, %%rdi;"
      "movq %0, %%rax;"
      "syscall;"
      :
      : "r"(sysno), "r"(arg1)
      : "rdi", "rax");
}

void sys3(uint64_t sysno, uint64_t arg1, uint64_t arg2, uint64_t arg3)
{
  asm volatile(
      "movq %1, %%rdi;"
      "movq %2, %%rsi;"
      "movq %3, %%rdx;"
      "movq %0, %%rax;"
      "syscall;"
      :
      : "r"(sysno), "r"(arg1), "r"(arg2), "r"(arg3)
      : "rdi", "rsi", "rdx", "rax");
}

void sys$exit(uint64_t status)
{
  sys1(SYS_EXIT, status);
}

void sys$write(filedesc_t fd, void *ptr, uint64_t len)
{
  sys3(SYS_WRITE, fd, (uint64_t)ptr, len);
}

extern void main();

void _start()
{
  main();

  sys$exit(0);
}

#endif