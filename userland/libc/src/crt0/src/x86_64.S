  .global _start
  .type _start, @function
_start:
  mov rdi, rsp
  and rsp, 0xFFFFFFFFFFFFFFF0

  call cilibc_start
  .size _start, . - _start
