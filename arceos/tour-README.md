# Quick Start

## 1. Install Build Dependencies

Install [cargo-binutils](https://github.com/rust-embedded/cargo-binutils) to use `rust-objcopy` and `rust-objdump` tools:

```bash
cargo install cargo-binutils
```

#### Dependencies for C apps, qemu tools

Install `libclang-dev`, `qemu-system-riscv64`, `dosfstools`, `wget`:

```bash
sudo apt install libclang-dev qemu-system-misc dosfstools wget
```

Download & install [musl](https://musl.cc) toolchains:

```bash
# download
wget https://musl.cc/riscv64-linux-musl-cross.tgz
# install
tar zxf riscv64-linux-musl-cross.tgz
# exec below command in bash OR add below info in ~/.bashrc
export PATH=`pwd`/riscv64-linux-musl-cross/bin:$PATH
```

## 2. Build and Run
### goto arceos directory
```
cd arceos
```
### clean up images of pflash and disk
```
rm pflash.img
rm disk.img
```
### build pflash and disk images
```
make pflash_img
make disk_img
```

### run tour/u_X_0
```
make run A=tour/u_1_0
make run A=tour/u_2_0
make run A=tour/u_3_0
make run A=tour/u_4_0
make run A=tour/u_5_0
make run A=tour/u_6_0
make run A=tour/u_6_1
make run A=tour/u_7_0 BLK=y
make run A=tour/u_8_0 BLK=y
```

### run tour/m_X_0
#### m_1_0 m_1_1 m_2_0
```
make payload
./update_disk.sh ./payload/origin/origin
make run A=tour/m_1_0 BLK=y
make run A=tour/m_1_1 BLK=y
make run A=tour/m_2_0 BLK=y
```
#### m_3_0
```
make payload
./update_disk.sh payload/hello_c/hello
make run A=tour/m_3_0 BLK=y
```
#### m_3_1
```
make payload
./update_disk.sh payload/fileops_c/fileops
make run A=tour/m_3_1 BLK=y
```

### run tour/h_X_0
#### h_1_0
```
make payload
./update_disk.sh payload/skernel/skernel
make run A=tour/h_1_0/ BLK=y
```

#### h_2_0
```
make pflash_img
make disk_img （在 disk.img 不存在时执行）
make A=tour/u_3_0/
./update_disk.sh tour/u_3_0/u_3_0_riscv64-qemu-virt.bin
make run A=tour/h_2_0/ BLK=y
```

#### h_3_0
```
make A=tour/u_6_0/
./update_disk.sh tour/u_6_0/u_6_0_riscv64-qemu-virt.bin
make run A=tour/h_3_0 BLK=y LOG=info
```

#### h_4_0
```
make A=tour/m_1_1
./update_disk.sh ./tour/m_1_1/m_1_1_riscv64-qemu-virt.bin
make run A=tour/h_4_0 BLK=y
```

