#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(oxide::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;
mod serial;
use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use x86_64::structures::paging::PageTable;

use crate::vga_buffer::WRITER;

// entry point into the os
entry_point!(kernel_main);
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use oxide::memory::active_level_4_table;
    use x86_64::VirtAddr;

    // run startup routines
    oxide::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let l4_table = unsafe { active_level_4_table(phys_mem_offset) };

    for (i, entry) in l4_table.iter().enumerate() {
        if !entry.is_unused() {
            println!("L4 Entry {}: {:?}", i, entry);

            // get the physical address from the entry and convert it
            let phys = entry.frame().unwrap().start_address();
            let virt = phys.as_u64() + boot_info.physical_memory_offset;
            let ptr = VirtAddr::new(virt).as_mut_ptr();
            let l3_table: &PageTable = unsafe { &*ptr };

            for (i, entry) in l3_table.iter().enumerate() {
                if !entry.is_unused() {
                    println!("    L3 Entry {}: {:?}", i, entry);
                }
            }
        }
    }

    

    //run tests only in test cfg.
    #[cfg(test)]
    test_main();

    
    //startup output // welcome screen
    println!("Hello Oxide!");
    println!("Enjoy Your Stay");
    println!();

    //wait for interrupts
    oxide::hlt_loop();
}


// kernel panic handler
// call panic!("some text") to initiate a kernel panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    // serial_println()
    // maybe print the error and say press any key to restart.
    // or maybe just attempt a restart
    // print logs
    // exit_qemu(QemuExitCode::Failed); may want to just exit
    oxide::hlt_loop();
}


// region: tests

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    oxide::test_panic_handler(info)
}


#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}



// endregion