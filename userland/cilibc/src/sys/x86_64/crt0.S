.section .text

.global _start
_start:
  mov %rsp, %rbp

  call _init

  mov (%rsp), %rdi
  mov 8(%rsp), %rsi
  mov 16(%rsp), %rdx
  call init_stdlib

  movl %eax, %edi
  call exit

.size _start, . - _start