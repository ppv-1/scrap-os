#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(scrap_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use scrap_os::allocator::HEAP_SIZE;
use alloc::{boxed::Box, vec::Vec, rc::Rc};

entry_point!(main);

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    scrap_os::test_panic_handler(info)
}

fn main(boot_info: &'static BootInfo) -> ! {
    use scrap_os::allocator;
    use scrap_os::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;

    scrap_os::init();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    test_main();
    loop {}
}

#[test_case]
fn simple_allocation() {
    let heap_value_1 = Box::new(41);
    let heap_value_2 = Rc::new(42);
    assert_eq!(*heap_value_1, 41);
    assert_eq!(*heap_value_2, 42);
}

#[test_case]
fn large_vec() {
    let n = 1000;
    let mut vec = Vec::new();
    for i in 0..n {
        vec.push(i);
    }
    assert_eq!(vec.iter().sum::<u64>(), (n - 1) * n / 2);
}   

#[test_case]
fn many_boxes() {
    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
}