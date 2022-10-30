ENTRY(_start)

SECTIONS {
	. = 0x200000;
	
	.text : {
		*(.text .text.*)
	}
	.plt : {
		*(.plt)
	}

	.rodata ALIGN(0x1000): {
		*(.rodata .rodata.*)
	}
	
	/* Dynamic linking info */
	.dynamic : { *(.dynamic) }
	.dynsym : { *(.dynsym) }
	.dynstr : { *(.dynstr) }
	.rela.got : { *(.rela.got) }
	.rela.dyn : { *(.rela.dyn) }
	.rel.dyn : { *(.rel.dyn) }

  .preinit_array    :
  {
    PROVIDE_HIDDEN (__preinit_array_start = .);
    KEEP (*(.preinit_array))
    PROVIDE_HIDDEN (__preinit_array_end = .);
  }
  .init_array    :
  {
    PROVIDE_HIDDEN (__init_array_start = .);
    KEEP (*(SORT_BY_INIT_PRIORITY(.init_array.*) SORT_BY_INIT_PRIORITY(.ctors.*)))
    KEEP (*(.init_array EXCLUDE_FILE (*crtbegin.o *crtbegin?.o *crtend.o *crtend?.o ) .ctors))
    PROVIDE_HIDDEN (__init_array_end = .);
  }
  .fini_array    :
  {
    PROVIDE_HIDDEN (__fini_array_start = .);
    KEEP (*(SORT_BY_INIT_PRIORITY(.fini_array.*) SORT_BY_INIT_PRIORITY(.dtors.*)))
    KEEP (*(.fini_array EXCLUDE_FILE (*crtbegin.o *crtbegin?.o *crtend.o *crtend?.o ) .dtors))
    PROVIDE_HIDDEN (__fini_array_end = .);
  }


	. = ALIGN(0x1000);
	.data ALIGN(0x1000): {
		*(.data .data.*)
	} 
	.bss ALIGN(0x1000) : {
		*(.bss .bss.*)
	}
}
