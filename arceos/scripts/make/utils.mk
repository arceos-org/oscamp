# Utility definitions and functions

GREEN_C := \033[92;1m
CYAN_C := \033[96;1m
YELLOW_C := \033[93;1m
GRAY_C := \033[90m
WHITE_C := \033[37m
END_C := \033[0m

define run_cmd
  @printf '$(WHITE_C)$(1)$(END_C) $(GRAY_C)$(2)$(END_C)\n'
  @$(1) $(2)
endef

define make_disk_image_fat32
  @printf "    $(GREEN_C)Creating$(END_C) FAT32 disk image \"$(1)\" ...\n"
  @dd if=/dev/zero of=$(1) bs=1M count=64
  @mkfs.fat -F 32 $(1)
endef

define make_disk_image
  $(if $(filter $(1),fat32), $(call make_disk_image_fat32,$(2)))
endef


ifeq ($(ARCH), riscv64)
define mk_pflash
  @RUSTFLAGS="" cargo build -p origin  --target riscv64gc-unknown-none-elf --release
  @rust-objcopy --binary-architecture=riscv64 --strip-all -O binary ./target/riscv64gc-unknown-none-elf/release/origin /tmp/origin.bin
  @printf "pfld\00\00\00\01" > /tmp/prefix.bin
  @printf "%08x" `stat -c "%s" /tmp/origin.bin` | xxd -r -ps > /tmp/size.bin
  @cat /tmp/prefix.bin /tmp/size.bin > /tmp/head.bin
  @dd if=/dev/zero of=./$(1) bs=1M count=32
  @dd if=/tmp/head.bin of=./$(1) conv=notrunc
  @dd if=/tmp/origin.bin of=./$(1) seek=16 obs=1 conv=notrunc
endef
endif

ifeq ($(ARCH), aarch64)
define mk_pflash
  @RUSTFLAGS="" cargo build -p origin  --target aarch64-unknown-none-softfloat --release
  @rust-objcopy --binary-architecture=aarch64 --strip-all -O binary ./target/aarch64-unknown-none-softfloat/release/origin /tmp/origin.bin
  @printf "pfld\00\00\00\01" > /tmp/prefix.bin
  @printf "%08x" `stat -c "%s" /tmp/origin.bin` | xxd -r -ps > /tmp/size.bin
  @cat /tmp/prefix.bin /tmp/size.bin > /tmp/head.bin
  @dd if=/dev/zero of=./$(1) bs=1M count=64
  @dd if=/tmp/head.bin of=./$(1) conv=notrunc
  @dd if=/tmp/origin.bin of=./$(1) seek=16 obs=1 conv=notrunc
endef
endif

ifeq ($(ARCH), x86_64)
define mk_pflash
  @RUSTFLAGS="" cargo build -p origin  --target x86_64-unknown-none --release
  @rust-objcopy --binary-architecture=x86_64 --strip-all -O binary ./target/x86_64-unknown-none/release/origin /tmp/origin.bin
  @printf "pfld\00\00\00\01" > /tmp/prefix.bin
  @printf "%08x" `stat -c "%s" /tmp/origin.bin` | xxd -r -ps > /tmp/size.bin
  @cat /tmp/prefix.bin /tmp/size.bin > /tmp/head.bin
  @dd if=/dev/zero of=./$(1) bs=1M count=64
  @dd if=/tmp/head.bin of=./$(1) conv=notrunc
  @dd if=/tmp/origin.bin of=./$(1) seek=16 obs=1 conv=notrunc
endef
endif

define setup_disk
  $(call build_origin)
  @mkdir -p ./mnt
  @sudo mount $(1) ./mnt
  @sudo mkdir -p ./mnt/sbin
  @sudo cp /tmp/origin.bin ./mnt/sbin
  @sudo umount ./mnt
  @rm -rf mnt
endef

define build_origin
  @RUSTFLAGS="" cargo build -p origin  --target riscv64gc-unknown-none-elf --release
  @rust-objcopy --binary-architecture=riscv64 --strip-all -O binary ./target/riscv64gc-unknown-none-elf/release/origin /tmp/origin.bin
endef
