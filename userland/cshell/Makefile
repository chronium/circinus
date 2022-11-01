export CC?=x86_64-elf-gcc

export TARGET=x86_64-unknown-circinus

CFLAGS?=\
	-std=c11 \
	-fno-builtin \
	-fno-stack-protector \
	-Wall \
	-g \
	-I . \
	-static \
	-nostdinc \
	-nostdlib \
	-isystem ../libc/sysroot/include

LDFLAGS?=\
	../libc/sysroot/lib/crt0.o \
	../libc/sysroot/lib/crti.o \
	../libc/sysroot/lib/crtn.o \
	../libc/sysroot/lib/libc.a

FILES=\
	main.o

BUILD=build
DESTDIR=../build

SRC=$(patsubst %,$(BUILD)/%,$(FILES))

BIN=$(BUILD)/sh

.PHONY: all clean install

all: $(BIN)

clean:
	rm -rf $(BUILD)/*

install: $(BIN)
	mkdir -p "$(DESTDIR)"
	cp $(BUILD)/sh $(DESTDIR)/

libc/sysroot:
	$(MAKE) -C ../libc sysroot

DEPS=libc/sysroot

$(BUILD)/%.o: %.c $(DEPS)
	mkdir -p "$$(dirname "$@")"
	$(CC) "$<" -o "$@" $(CFLAGS) -c

$(BIN): $(SRC)
	mkdir -p "$$(dirname "$@")"
	$(CC) "$<" -o "$@" $(CFLAGS) $(LDFLAGS)


