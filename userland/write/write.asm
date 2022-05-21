global _start
section .text

_start:
  mov rdi, 1 ; stdout
  mov rsi, hw
  mov rdx, 7 ; 6 chars + newline
  ; sys_write
  mov rax, 1
  syscall

  ; sys_exit
  mov rax, -1
  syscall

section .data
hw: db "Hello!", 10