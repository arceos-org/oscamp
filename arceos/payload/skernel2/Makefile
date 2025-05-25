TARGET := skernel2
ARCH ?= riscv64

ifeq ($(ARCH),riscv64)
    TARGET_TRIPLE := riscv64gc-unknown-none-elf
    BINARY_ARCH := riscv64
else ifeq ($(ARCH),x86_64)
    TARGET_TRIPLE := x86_64-unknown-none
    BINARY_ARCH := x86_64
else ifeq ($(ARCH),aarch64)
    TARGET_TRIPLE := aarch64-unknown-none
    BINARY_ARCH := aarch64
else
    $(error Unsupported architecture: $(ARCH))
endif

TARGET_ELF := ../../target/$(TARGET_TRIPLE)/release/$(TARGET)
TARGET_BIN := $(TARGET).$(ARCH)

all: $(TARGET) FORCE

$(TARGET): $(TARGET_ELF)
	@echo "Building for $(ARCH) architecture..."
	@rust-objcopy --binary-architecture=$(BINARY_ARCH) --strip-all -O binary $< $@

$(TARGET_ELF):
	@echo "Compiling for $(ARCH) architecture..."
	@cargo build -p $(TARGET) --target $(TARGET_TRIPLE) --release


clean:
	@rm -rf ./$(TARGET).*
	@for arch in riscv64 x86_64 aarch64 ; do \
        echo "Cleaning $$arch build..."; \
        cargo clean -p $(TARGET) --target $$(make --no-print-directory ARCH=$$arch get-target-triple) --release 2>/dev/null || true; \
    done

FORCE:

.PHONY: FORCE