target = x86_64-unknown-circinus
src_dir = src

sys = $(src_dir)/sys
sys64 = $(sys)/x86_64
string = $(src_dir)/string
stdio = $(src_dir)/stdio
stdlib = $(src_dir)/stdlib

crt_objects = $(sys64)/crt0.o $(sys64)/crti.o $(sys64)/crtn.o
other_objects = $(sys64)/syscall.o\
								$(src_dir)/alloc/liballoc.o\
								$(sys)/exit.o\
								$(sys)/write.o\
								$(sys)/brk.o\
								$(string)/strlen.o\
								$(string)/memset.o\
								$(stdio)/puts.o\
								$(stdio)/fops.o\
								$(stdio)/printf.o\
								$(stdlib)/mem.o\
								$(stdio)/__stdio.o\
								$(src_dir)/cilibc.o
objects = $(crt_objects) $(other_objects)
build_dir = ../sysroot

CFLAGS=-Iinclude/

.PHONY: all clean install

all: libc.a


%.o: %.S
	${target}-as -c $< -o $@

%.o: %.c
	${target}-gcc $(CFLAGS) -c $< -o $@

libc.a: $(objects)
	${target}-ar rcs libc.a $(objects)

install: libc.a
	mkdir -pv $(build_dir)/lib
	rm -rf $(build_dir)/lib/*
	cp -rv libc.a $(crt_objects) $(build_dir)/lib
	mkdir -pv $(build_dir)/include
	rm -rf $(build_dir)/include/*
	cp -rv include/* $(build_dir)/include

clean:
	rm -f $(objects) libc.a