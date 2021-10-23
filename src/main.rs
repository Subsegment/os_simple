#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(os_simple::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use os_simple::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    os_simple::init();

    use x86_64::registers::control::Cr3;

    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table at: {:?}", level_4_page_table.start_address());

    let ptr = 0x2031b2 as *mut u32;

// read from a code page
    unsafe { let x = *ptr; }
    println!("read worked");

// write to a code page
    unsafe { *ptr = 42; }
    println!("write worked");

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    os_simple::hlt_loop();
}



/// This function is called on panic.

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    os_simple::hlt_loop();
}

///panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os_simple::test_panic_handler(info);
}
