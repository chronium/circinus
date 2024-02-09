CC:=x86_64-elf-gcc
TARGET:=x86_64-unknown-circinus

export CFLAGS=\
	-std=c11 \
	-fno-builtin \
	-fno-stack-protector \
	-Wall \
	-g \
	-I . \
	-static \
	-nostdinc \
	-nostdlib \
	-isystem ../../sysroot/include

export LDFLAGS=\
	../../sysroot/lib/crt0.o \
	../../sysroot/lib/crti.o \
	../../sysroot/lib/crtn.o \
	../../sysroot/lib/libc.a

BINS=\
	echo \
	ls \
	cat

export BUILD?=bins
export DESTDIR=../build

.PHONY: all clean install $(BINS)

all: $(BINS)

clean:
	rm -rf $(BUILD)
	for bin in $(BINS); do \
		$(MAKE) -C $$bin clean; \
	done

install: all
	mkdir -p "$(DESTDIR)"
	for bin in $(BINS); do \
		cp -v "$(BUILD)/$$bin" "$(DESTDIR)"; \
	done

$(BINS):
	$(MAKE) -C $@
