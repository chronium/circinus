FILES=\
	echo.main.o

SRC=$(patsubst %,$(BUILD)/%,$(FILES))

BIN=../$(BUILD)/echo

.PHONY: all clean install

all: $(BIN)

install: $(BIN)

clean:
	rm -rf $(BUILD)

$(BUILD)/%.o: %.c
	mkdir -p "$$(dirname "$@")"
	$(CC) "$<" -o "$@" $(CFLAGS) -c

$(BIN): $(SRC)
	mkdir -p "$$(dirname "$@")"
	$(CC) "$<" -o "$@" $(CFLAGS) $(LDFLAGS)