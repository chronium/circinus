TARGET?=x86_64-unknown-circinus
DESTDIR?=$(shell pwd)/../sysroot

CARGO?=cargo
CARGO_TEST?=$(CARGO)
CARGO_COMMON_FLAGS=-Z build-std=core,alloc,compiler_builtins
CARGOFLAGS?=$(CARGO_COMMON_FLAGS)
RUSTCFLAGS?=
export OBJCOPY?=objcopy

BUILD="target/$(TARGET)"
CARGOFLAGS+="--target=$(TARGET)"

ifeq ($(TARGET),x86_64-unknown-linux-gnu)
	export CC=x86_64-linux-gnu-gcc
	export LD=x86_64-linux-gnu-ld
	export AR=x86_64-linux-gnu-ar
	export OBJCOPY=x86_64-linux-gnu-objcopy
endif

ifeq ($(TARGET),x86_64-unknown-circinus)
	export CC=x86_64-elf-gcc
	export LD=x86_64-elf-ld
	export AR=x86_64-elf-ar
	export OBJCOPY=x86_64-elf-objcopy
endif

SRC=\
	Cargo.* \
	$(shell find src -type f)

.PHONY: all clean install install-headers libs test

all: | libs

test: sysroot
	$(MAKE) -C tests
	cp tests/bins_*/* ../build/

clean:
	$(CARGO) clean
	$(MAKE) -C tests clean
	rm -rf sysroot

check:
	$(CARGO) check

install-headers: libs
	mkdir -pv "$(DESTDIR)/include"
	cp -rv "include/"* "$(DESTDIR)/include"
	cp -rv "target/include"/* "$(DESTDIR)/include"

libs: \
	$(BUILD)/release/libc.a \
	$(BUILD)/release/libc.so \
	$(BUILD)/release/crt0.o \
	$(BUILD)/release/crti.o \
	$(BUILD)/release/crtn.o

install-libs: libs
	mkdir -pv "$(DESTDIR)/lib"
	cp -v "$(BUILD)/release/libc.a" "$(DESTDIR)/lib"
	cp -v "$(BUILD)/release/libc.so" "$(DESTDIR)/lib"
	ln -sfrv "$(DESTDIR)/lib/libc.so" "$(DESTDIR)/lib/libc.so.6"
	cp -v "$(BUILD)/release/crt0.o" "$(DESTDIR)/lib"
	ln -sfrv "$(DESTDIR)/lib/crt0.o" "$(DESTDIR)/lib/crt1.o"
	cp -v "$(BUILD)/release/crti.o" "$(DESTDIR)/lib"
	cp -v "$(BUILD)/release/crtn.o" "$(DESTDIR)/lib"

install: install-headers install-libs

sysroot: all
	rm -rf $@
	rm -rf $@.partial
	mkdir -p $@.partial
	$(MAKE) install DESTDIR=$@.partial
	mv $@.partial $@
	touch $@

$(BUILD)/release/libc.a: $(BUILD)/release/libcilibc.a
	echo "create $@" > "$@.mri"

	for lib in $^; do\
		echo "addlib $$lib" >> "$@.mri";\
	done

	echo "save" >> "$@.mri"
	echo "end" >> "$@.mri"

	$(AR) -M < "$@.mri"

$(BUILD)/release/libc.so: $(BUILD)/release/libcilibc.a
	$(CC) -nostdlib -shared -Wl,--allow-multiple-definition -Wl,--whole-archive $^ -Wl,--no-whole-archive -Wl,-soname,libc.so.6 -o $@

$(BUILD)/release/libcilibc.a: $(SRC)
	CARGO_INCREMENTAL=0 $(CARGO) rustc --release $(CARGOFLAGS) -- --emit link=$@ $(RUSTFLAGS)

	./renamesyms.sh $@ $(BUILD)/release/deps/
	touch $@

$(BUILD)/release/crt0.o: $(SRC)
	CARGO_INCREMENTAL=0 $(CARGO) rustc --release --manifest-path src/crt0/Cargo.toml $(CARGOFLAGS) -- --emit obj=$@ -C panic=abort $(RUSTFLAGS)

$(BUILD)/release/crti.o: $(SRC)
	CARGO_INCREMENTAL=0 $(CARGO) rustc --release --manifest-path src/crti/Cargo.toml $(CARGOFLAGS) -- --emit obj=$@ -C panic=abort $(RUSTFLAGS)

$(BUILD)/release/crtn.o: $(SRC)
	CARGO_INCREMENTAL=0 $(CARGO) rustc --release --manifest-path src/crtn/Cargo.toml $(CARGOFLAGS) -- --emit obj=$@ -C panic=abort $(RUSTFLAGS)
