.section .text

.global _start
_start:
  movq $0, %rbp
  pushq %rbp
  pushq %rbp
  movq %rsp, %rbp

  pushq %rsi
  pushq %rdi

  call _init

  call init_stdlib

  popq %rdi
  popq %rsi

  call main

  movl %eax, %edi
  call exit

.size _start, . - _start
