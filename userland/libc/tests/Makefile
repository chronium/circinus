export TARGET:=x86_64-unknown-circinus

EXECS:=\
	hello

BINS=$(patsubst %,bins_static/%,$(EXECS))

DESTDIR:=$(shell pwd)/../../build

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

DEPS:
	$(MAKE) -C .. sysroot

bins_static/%: %.c DEPS
	mkdir -p "$$(dirname "$@")"
	$(CC) "$<" -o "$@" $(CFLAGS)

install: $(BINS)
	mkdir -p "$(DESTDIR)"
	cp $(BINS) "$(DESTDIR)"
