CC=x86_64-unknown-circinus-gcc

TARGET=x86_64-unknown-circinus

CFLAGS?=\
	-std=c11 \
	-fno-builtin \
	-fno-stack-protector \
	-Wall \
	-g \
	-I . \
	-static

LDFLAGS?=

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

$(BUILD)/%.o: %.c
	mkdir -p "$$(dirname "$@")"
	$(CC) "$<" -o "$@" $(CFLAGS) -c

$(BIN): $(SRC)
	mkdir -p "$$(dirname "$@")"
	$(CC) "$<" -o "$@" $(CFLAGS) $(LDFLAGS)


