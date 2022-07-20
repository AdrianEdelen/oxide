#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(oxide::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;
mod serial;
use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};

//entry point into the os
entry_point!(kernel_main);
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    #[cfg(test)]
    test_main();

    oxide::init();
    //startup output

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

#[test_case]
fn test_println_simple(){
    println!("test_println_simple output");
}

#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println!("")
    }
}

// endregion