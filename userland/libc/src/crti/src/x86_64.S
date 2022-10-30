.section .init
.global _init
_init:
    push rbp
    mov rbp, rsp
    // Created a new stack frame and updated the stack pointer
    // Body will bve filled in by gcc and ended by crtn.o

.section .fini
.global _fini
_fini:
    push rbp
    mov rbp, rsp
    //Created a new stack frame and updated the stack pointer
    // Body will be filled in by gcc and ended by crtn.o
