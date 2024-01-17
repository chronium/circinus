.section .text

.global _start
_start:
  movq $0, %rbp
  pushq %rbp
  pushq %rbp
  movq %rsp, %rbp

  pushq %rsi
  pushq %rdi

  call init_stdlib

  call _init

  popq %rdi
  popq %rsi

  call main

  movl %eax, %edi
  call exit

.size _start, . - _start
