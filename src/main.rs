#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use os::println;

/// This function is called on panic outside of testing.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

//this panic is called for testing environment.
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1,1);
}

#[no_mangle]
pub extern "C" fn _start() {
    println!("Hello World{}", "!");

    os::init();

    //x86_64::instructions::interrupts::int3();

    fn stack_overflow() {
        stack_overflow();
    }

    stack_overflow();

    #[cfg(test)]
    test_main();

    // our main
    println!("Did not crash!");
    loop {}
}