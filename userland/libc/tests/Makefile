EXECS:=\
	hello

BINS=$(patsubst %,bins_static/%,$(EXECS))

CFLAGS?=\
	-std=c11 \
	-fno-builtin \
	-fno-stack-protector \
	-Wall \
	-pedantic \
	-g \
	-I . \
	-static \
	-nostdinc \
	-nostdlib \
	-isystem ../sysroot/include \
	../sysroot/lib/crt0.o \
	../sysroot/lib/crti.o \
	../sysroot/lib/crtn.o \
	../sysroot/lib/libc.a

.PHONY: all clean

all: $(BINS)

clean:
	rm -rf bins_* gen *.out

../sysroot:
	$(MAKE) -C .. sysroot

DEPS=../sysroot

bins_static/%: %.c $(DEPS)
	mkdir -p "$$(dirname "$@")"
	$(CC) "$<" -o "$@" $(CFLAGS)
