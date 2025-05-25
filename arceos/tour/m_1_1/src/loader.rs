use core::{mem, slice};
use axstd::io;
use axhal::paging::MappingFlags;
use axhal::mem::{PAGE_SIZE_4K, phys_to_virt};
use axmm::AddrSpace;
use crate::APP_ENTRY;
use alloc::vec;
use alloc::vec::Vec;

#[cfg(target_arch = "riscv64")]
/// Physical address for pflash#1
const PFLASH_START: usize = 0x2200_0000;
#[cfg(target_arch = "aarch64")]
/// Physical address for pflash#1
const PFLASH_START: usize = 0x0400_0000;
const MAGIC: u32 = 0x64_6C_66_70;
const VERSION: u32 = 0x01;

struct PayloadHead {
    _magic: u32,
    _version: u32,
    _size: u32,
    _pad: u32,
}

pub fn load_user_app(uspace: &mut AddrSpace) -> io::Result<()> {
    let buf = load_pflash();

    uspace.map_alloc(APP_ENTRY.into(), PAGE_SIZE_4K, MappingFlags::READ|MappingFlags::WRITE|MappingFlags::EXECUTE|MappingFlags::USER, true).unwrap();

    let (paddr, _, _) = uspace
        .page_table()
        .query(APP_ENTRY.into())
        .unwrap_or_else(|_| panic!("Mapping failed for segment: {:#x}", APP_ENTRY));

    ax_println!("paddr: {:#x}", paddr);

    unsafe {
        core::ptr::copy_nonoverlapping(
            buf.as_ptr(),
            phys_to_virt(paddr).as_mut_ptr(),
            buf.len(),
        );
    }

    Ok(())
}

pub fn load_pflash() -> Vec<u8> {
    let va = phys_to_virt(PFLASH_START.into());
    let data = va.as_usize() as *const u32;
    let data = unsafe {
        slice::from_raw_parts(data, mem::size_of::<PayloadHead>())
    };
    assert_eq!(data[0], MAGIC);
    assert_eq!(data[1].to_be(), VERSION);

    let size = data[2].to_be() as usize;
    let start = va + mem::size_of::<PayloadHead>();
    ax_println!("Pflash: start {:#X} size {}", start, size);
    let mut buf = vec![0u8; size];
    unsafe {
        core::ptr::copy_nonoverlapping(
            start.as_usize() as *const u8,
            buf.as_mut_ptr(),
            size,
        );
    }
    buf
}
