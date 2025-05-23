use std::io::{self, Read};
use std::io::SeekFrom;
use std::io::Seek;
use std::fs::File;
use alloc::vec::Vec;
use alloc::vec;
use axhal::paging::MappingFlags;
use axhal::mem::{PAGE_SIZE_4K, VirtAddr, MemoryAddr};
use axmm::AddrSpace;

use elf::abi::{PT_INTERP, PT_LOAD};
use elf::endian::AnyEndian;
use elf::parse::ParseAt;
use elf::segment::ProgramHeader;
use elf::segment::SegmentTable;
use elf::ElfBytes;

const ELF_HEAD_BUF_SIZE: usize = 256;

pub fn load_user_app(fname: &str, uspace: &mut AddrSpace) -> io::Result<usize> {
    let mut file = File::open(fname)?;
    let (phdrs, entry, _, _) = load_elf_phdrs(&mut file)?;
    ax_println!("phdrs: {:#X} phnum: {}", phdrs[0].p_vaddr, phdrs.len());

    for phdr in &phdrs {
        ax_println!(
            "phdr: offset: {:#X}=>{:#X} size: {:#X}=>{:#X}",
            phdr.p_offset, phdr.p_vaddr, phdr.p_filesz, phdr.p_memsz
        );

        let vaddr = VirtAddr::from(phdr.p_vaddr as usize).align_down_4k();
        let vaddr_end = VirtAddr::from((phdr.p_vaddr+phdr.p_memsz) as usize)
            .align_up_4k();

        ax_println!("{:#x} - {:#x}", vaddr, vaddr_end);
        uspace.map_alloc(vaddr, vaddr_end-vaddr, MappingFlags::READ|MappingFlags::WRITE|MappingFlags::EXECUTE|MappingFlags::USER, true)?;

        let mut data = vec![0u8; phdr.p_memsz as usize];
        ax_println!("data: {:#X} size: {:#X}", phdr.p_vaddr, phdr.p_memsz);
        file.seek(SeekFrom::Start(phdr.p_offset))?;

        let filesz = phdr.p_filesz as usize;
        let mut index = 0;
        while index < filesz {
            let n = file.read(&mut data[index..filesz])?;
            index += n;
        }
        assert_eq!(index, filesz);
        uspace.write(VirtAddr::from(phdr.p_vaddr as usize), &data)?;
    }

    Ok(entry)
}

fn load_elf_phdrs(file: &mut File) -> io::Result<(Vec<ProgramHeader>, usize, usize, usize)> {
    let mut buf: [u8; ELF_HEAD_BUF_SIZE] = [0; ELF_HEAD_BUF_SIZE];
    file.read(&mut buf)?;

    let ehdr = ElfBytes::<AnyEndian>::parse_elf_header(&buf[..]).unwrap();
    info!("e_entry: {:#X}", ehdr.e_entry);

    let phnum = ehdr.e_phnum as usize;
    // Validate phentsize before trying to read the table so that we can error early for corrupted files
    let entsize = ProgramHeader::validate_entsize(ehdr.class, ehdr.e_phentsize as usize).unwrap();
    let size = entsize.checked_mul(phnum).unwrap();
    assert!(size > 0 && size <= PAGE_SIZE_4K);
    let phoff = ehdr.e_phoff;
    let mut buf = alloc::vec![0u8; size];
    let _ = file.seek(SeekFrom::Start(phoff));
    file.read(&mut buf)?;
    let phdrs = SegmentTable::new(ehdr.endianness, ehdr.class, &buf[..]);

    let phdrs: Vec<ProgramHeader> = phdrs
        .iter()
        .filter(|phdr| phdr.p_type == PT_LOAD || phdr.p_type == PT_INTERP)
        .collect();
    Ok((phdrs, ehdr.e_entry as usize, ehdr.e_phoff as usize, ehdr.e_phnum as usize))
}
