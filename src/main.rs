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
    os::hlt_loop();
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

use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use os::memory;
    use x86_64::{structures::paging::Page, VirtAddr};
    use os::memory::BootInfoFrameAllocator;

    println!("Hello World{}", "!");

    os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    let page_ptr: *mut u64 = page.start_address().as_mut_ptr(); //create a pointer to the our page address
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    //use x86_64::registers::control::Cr3;

    // let addresses = [
    //     // the identity-mapped vga buffer page
    //     0xb8000,
    //     // some code page
    //     0x201008,
    //     // some stack page
    //     0x0100_0020_1a10,
    //     // virtual address mapped to physical address 0
    //     boot_info.physical_memory_offset,
    // ];

    // for &address in &addresses {
    //     let virt = VirtAddr::new(address);
    //     let phys = mapper.translate_addr(virt);
    //     println!("{:?} -> {:?}", virt, phys);
    // }

    // as before
    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    os::hlt_loop();
}