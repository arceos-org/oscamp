TARGET := fileops
ARCH ?= riscv64

ifeq ($(ARCH),riscv64)
    CC := riscv64-linux-musl-gcc
    STRIP := riscv64-linux-musl-strip
else ifeq ($(ARCH),x86_64)
    CC := x86_64-linux-musl-gcc
    STRIP := x86_64-linux-musl-strip
else ifeq ($(ARCH),aarch64)
    CC := aarch64-linux-musl-gcc
    STRIP := aarch64-linux-musl-strip
else
    $(error Unsupported architecture: $(ARCH))
endif

all: $(TARGET)

%: %.c
	$(CC) -static $< -o $@
	$(STRIP) $@

clean:
	@rm -rf ./$(TARGET)
