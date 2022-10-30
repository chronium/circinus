.section .init
    // This happens after crti.o and gcc has inserted code
    // Pop the stack frame
    pop rbp
    ret

.section .fini
    // This happens after crti.o and gcc has inserted code
    // Pop the stack frame
    pop rbp
    ret
