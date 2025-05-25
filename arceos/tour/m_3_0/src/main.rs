#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[cfg(feature = "axstd")]
extern crate axstd as std;
extern crate alloc;

#[macro_use]
extern crate axlog;

mod task;
mod syscall;
mod loader;

use axstd::io;
use axhal::paging::MappingFlags;
use axhal::arch::UspaceContext;
use axhal::mem::VirtAddr;
use axsync::Mutex;
use alloc::sync::Arc;
use alloc::string::String;
use alloc::collections::BTreeMap;
use axmm::AddrSpace;
use loader::load_user_app;

const USER_STACK_SIZE: usize = 0x10000;
const KERNEL_STACK_SIZE: usize = 0x40000; // 256 KiB

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    // A new address space for user app.
    let mut uspace = axmm::new_user_aspace().unwrap();

    // Load user app binary file into address space.
    let entry = match load_user_app("/sbin/hello", &mut uspace) {
        Ok(e) => e,
        Err(err) => panic!("Cannot load app! {:?}", err),
    };
    ax_println!("entry: {:#x}", entry);

    // Init user stack.
    let ustack_top = init_user_stack(&mut uspace, true).unwrap();
    ax_println!("New user address space: {:#x?}", uspace);

    // Let's kick off the user process.
    let user_task = task::spawn_user_task(
        Arc::new(Mutex::new(uspace)),
        #[cfg(target_arch = "riscv64")]
        UspaceContext::new(entry, ustack_top),
        #[cfg(target_arch = "aarch64")]
        UspaceContext::new(entry, ustack_top, 0),
    );

    // Wait for user process to exit ...
    let exit_code = user_task.join();
    ax_println!("monolithic kernel exit [{:?}] normally!", exit_code);
}

fn init_user_stack(uspace: &mut AddrSpace, populating: bool) -> io::Result<VirtAddr> {
    let ustack_top = uspace.end();
    let ustack_vaddr = ustack_top - crate::USER_STACK_SIZE;
    ax_println!(
        "Mapping user stack: {:#x?} -> {:#x?}",
        ustack_vaddr, ustack_top
    );
    uspace.map_alloc(
        ustack_vaddr,
        crate::USER_STACK_SIZE,
        MappingFlags::READ | MappingFlags::WRITE | MappingFlags::USER,
        populating,
    ).unwrap();

    let app_name = "hello";
    let av = BTreeMap::new();
    let (stack_data, ustack_pointer) = kernel_elf_parser::get_app_stack_region(
        &[String::from(app_name)],
        &[],
        &av,
        ustack_vaddr,
        crate::USER_STACK_SIZE,
    );
    uspace.write(VirtAddr::from_usize(ustack_pointer), stack_data.as_slice())?;

    Ok(ustack_pointer.into())
}
