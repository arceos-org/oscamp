#![no_std]
#![no_main]

#[macro_use]
extern crate log;
#[macro_use]
extern crate alloc;
extern crate axstd as std;
use alloc::string::ToString;
use riscv_vcpu::AxVCpuExitReason;
use axerrno::{ax_err_type, AxResult};
use memory_addr::VirtAddr;
use alloc::string::String;
use std::fs::File;
use riscv_vcpu::RISCVVCpu;
use riscv_vcpu::AxVCpuExitReason::NestedPageFault;

const VM_ASPACE_BASE: usize = 0x0;
const VM_ASPACE_SIZE: usize = 0x7fff_ffff_f000;
const PHY_MEM_START: usize = 0x8000_0000;
const PHY_MEM_SIZE: usize = 0x100_0000;
const KERNEL_BASE: usize = 0x8020_0000;

use axmm::AddrSpace;
use axhal::paging::MappingFlags;

#[no_mangle]
fn main() {
    info!("Starting virtualization...");
    unsafe {
        riscv_vcpu::setup_csrs();
    }

    // Setup AddressSpace and regions.
    let mut aspace = AddrSpace::new_empty(VirtAddr::from(VM_ASPACE_BASE), VM_ASPACE_SIZE).unwrap();

    // Physical memory region. Full access flags.
    let mapping_flags = MappingFlags::from_bits(0xf).unwrap();
    aspace.map_alloc(PHY_MEM_START.into(), PHY_MEM_SIZE, mapping_flags, true).unwrap();

    // Load corresponding images for VM.
    info!("VM created success, loading images...");
    let image_fname = "/sbin/u_3_0_riscv64-qemu-virt.bin";
    load_vm_image(image_fname.to_string(), KERNEL_BASE.into(), &aspace).expect("Failed to load VM images");

    // Create VCpus.
    let mut arch_vcpu = RISCVVCpu::init();

    // Setup VCpus.
    info!("bsp_entry: {:#x}; ept: {:#x}", KERNEL_BASE, aspace.page_table_root());
    arch_vcpu.set_entry(KERNEL_BASE.into()).unwrap();
    arch_vcpu.set_ept_root(aspace.page_table_root()).unwrap();

    loop {
        match vcpu_run(&mut arch_vcpu) {
            Ok(exit_reason) => match exit_reason {
                AxVCpuExitReason::Nothing => {},
                NestedPageFault{addr, access_flags} => {
                    debug!("addr {:#x} access {:#x}", addr, access_flags);
                    assert_eq!(addr, 0x2200_0000.into(), "Now we ONLY handle pflash#2.");
                    let mapping_flags = MappingFlags::from_bits(0xf).unwrap();
                    // Passthrough-Mode
                    // let _ = aspace.map_linear(addr, addr.as_usize().into(), 4096, mapping_flags);

                    // Emulator-Mode
                    // Pretend to load file to fill buffer.
                    let buf = "pfld";

                    // 从 disk.img 加载 pflash 文件内容
                    // const DISK_IMG_PATH: &str = "/home/linux/vscode/oscamp/arceos/disk.img";
                    // let mut disk_file = File::open(DISK_IMG_PATH).expect("Failed to open disk.img");
                    // let mut buf = [0u8; 4096]; // 每次加载一个页帧大小
                    // use std::io::Seek;
                    // use crate::std::io::Read;
                    // disk_file
                    //     .seek(std::io::SeekFrom::Start(KERNEL_BASE.try_into().unwrap()))
                    //     .expect("Failed to seek to pflash location");
                    // disk_file
                    //     .read_exact(&mut buf)
                    //     .expect("Failed to read pflash data");

                    aspace.map_alloc(addr, 4096, mapping_flags, true);
                    aspace.write(addr, buf.as_bytes());
                },
                _ => {
                    panic!("Unhandled VM-Exit: {:?}", exit_reason);
                }
            },
            Err(err) => {
                panic!("run VCpu get error {:?}", err);
            }
        }
    }
}

fn load_vm_image(image_path: String, image_load_gpa: VirtAddr, aspace: &AddrSpace) -> AxResult {
    use std::io::{BufReader, Read};
    let (image_file, image_size) = open_image_file(image_path.as_str())?;

    let image_load_regions = aspace
        .translated_byte_buffer(image_load_gpa, image_size)
        .expect("Failed to translate kernel image load address");
    let mut file = BufReader::new(image_file);

    for buffer in image_load_regions {
        file.read_exact(buffer).map_err(|err| {
            ax_err_type!(
                Io,
                format!("Failed in reading from file {}, err {:?}", image_path, err)
            )
        })?
    }

    Ok(())
}

fn vcpu_run(arch_vcpu: &mut RISCVVCpu) -> AxResult<AxVCpuExitReason> {
    use axhal::arch::{local_irq_save_and_disable, local_irq_restore};
    let flags = local_irq_save_and_disable();
    let ret = arch_vcpu.run();
    local_irq_restore(flags);
    ret
}

fn open_image_file(file_name: &str) -> AxResult<(File, usize)> {
    let file = File::open(file_name).map_err(|err| {
        ax_err_type!(
            NotFound,
            format!(
                "Failed to open {}, err {:?}, please check your disk.img",
                file_name, err
            )
        )
    })?;
    let file_size = file
        .metadata()
        .map_err(|err| {
            ax_err_type!(
                Io,
                format!(
                    "Failed to get metadate of file {}, err {:?}",
                    file_name, err
                )
            )
        })?
        .size() as usize;
    Ok((file, file_size))
}
