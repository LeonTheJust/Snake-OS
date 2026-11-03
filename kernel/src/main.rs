#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

mod framebuffer;
mod serial;
mod logger;
mod interrupts;
mod gdt;
mod memory;
mod heap;
mod keyboard;
mod terminal;

extern crate alloc;

use core::alloc::{GlobalAlloc, Layout};
use x86_64::structures::paging::Page;
use bootloader_api::{entry_point, BootInfo, BootloaderConfig};
use uart_16550::{Uart16550, backend::PioBackend};
use core::sync::atomic::{AtomicBool, Ordering};
use core::sync::atomic::Ordering::Relaxed;
use bootloader_api::config::Mapping;
use bootloader_api::info::{MemoryRegion, MemoryRegionKind};
use x86_64::registers::control::Cr3;
use x86_64::structures::paging::{FrameAllocator, PageTable, Size4KiB, Translate};
use x86_64::VirtAddr;
use crate::heap::{BumpAllocator, ALLOCATOR};
use crate::interrupts::InterruptHandler;
use crate::logger::Logger;
use crate::memory::{init_level_4_table, MemoryAllocator};
use crate::serial::{emergency_message, u64_to_str, Serial};
use alloc::boxed::Box;
use alloc::vec::Vec;
use crate::keyboard::{ KEYBOARD_QUEUE};

pub const BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};

entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    let config = BootloaderConfig::new_default();


    let framebuffer = boot_info.framebuffer.as_mut().unwrap();

    let info = framebuffer.info();
    let buffer = framebuffer.buffer_mut();

    // Fill the framebuffer with a simple pattern.
    for byte in buffer.iter_mut() {
        *byte = 0;
    }

    let width = info.width;
    let height = info.height;

    // Draw a white rectangle in the top-left corner.
    for y in 0..info.height {
        for x in 0..info.width {
            let pixel = y * info.stride + x;

            let offset = pixel * info.bytes_per_pixel;

            if offset + 3 < buffer.len() {
                buffer[offset] = 255;
                buffer[offset + 1] = 255;
                buffer[offset + 2] = 255;
            }

        }
    }

    let mut s = serial::Serial::new();
    let mut log = logger::Logger::new(s);
    let mut level = logger::LogLevel::new(3);
    let mut level2 = logger::LogLevel::new(2);
    let mut level1 = logger::LogLevel::new(1);
    gdt::init();
    interrupts::init();

    Logger::log(&mut log,"Before breakpoint\n", level);
    if(interrupts::flag2.load(Ordering::SeqCst)){
        Logger::log(&mut log, "test2\n", level);
    }
    if(interrupts::flag.load(Ordering::SeqCst)){
        Logger::log(&mut log, "test\n", level);
    }
    //x86_64::instructions::interrupts::int3();
    Logger::log(&mut log, "After breakpoint\n", level);

    let test_address = 0x1000usize;

    let mut buffer = [0u8;20];

    Logger::log(&mut log, serial::u64_to_str(interrupts::tick_counter.load(Relaxed), &mut buffer), level);

    /*unsafe {
        core::arch::asm!(
        "mov {value}, qword ptr [{address}]",
        address = in(reg) test_address,
        value = lateout(reg) _,
        options(readonly, nostack, preserves_flags),
        );
    }*/
    let mut sum = 0;
    let region = &boot_info.memory_regions;
    for region in boot_info.memory_regions.iter() {
        // region is a &MemoryRegion
        let start = region.start;
        let end = region.end;
        let kind = region.kind;
        if(region.kind == MemoryRegionKind::Usable){
            sum = sum + end - start;
        }
    }
    Logger::log(&mut log, u64_to_str(sum, &mut buffer), level);

    let _ = (width, height);
    framebuffer::clear(framebuffer);

    let mut memory = MemoryAllocator::new(region);

    let Some(offset) = boot_info.physical_memory_offset.into_option() else { panic!("Physical memory offset is not valid") };

    unsafe {
        let mut offset_page_table = memory::init_level_4_table(VirtAddr::new(offset));
        let virtual_addr = VirtAddr::new(0x4444_4444_0000);
        let page = Page::<Size4KiB>::containing_address(virtual_addr);
        let bump_first_page = memory::map_multiple_pages(16, &mut offset_page_table, &mut memory, page);
        let layout = Layout::new::<u64>();
        heap::ALLOCATOR.set_bump_allocator(bump_first_page.start_address().as_u64() as usize, bump_first_page.start_address().as_u64() as usize + (16 * 4096));
        let res = ALLOCATOR.alloc(layout);
        Logger::log(&mut log, "alllllllll goooooodddd" , level);
    }

    //keyboard_queue.push(0);
    /*for i in 1..100 {
        keyboard_queue.push(i as u8);
        emergency_message(u64_to_str(keyboard_queue.pop().unwrap() as u64, &mut buffer));
        emergency_message(" ");
        emergency_message(u64_to_str(i, &mut buffer));
        emergency_message("\n");
    }
    emergency_message(u64_to_str(keyboard_queue.pop().unwrap() as u64, &mut buffer));
    emergency_message(u64_to_str(keyboard_queue.pop().unwrap() as u64, &mut buffer));
    emergency_message(u64_to_str(keyboard_queue.pop().unwrap() as u64, &mut buffer));*/



    KEYBOARD_QUEUE.lock().push(0x12);
    loop {
        core::hint::spin_loop();
        //Logger::log(&mut log, serial::u64_to_str(interrupts::tick_counter.load(Relaxed), &mut buffer), level);
        //Logger::log(&mut log, serial::u64_to_str(interrupts::KEYBOARD.load(Relaxed) as u64, &mut buffer), level);
        /*let mut q = KEYBOARD_QUEUE.lock();
        let temp = q.pop();
        if ! temp.is_none() {
            if temp.unwrap() < 100 {
                let mut tmp = [0u8; 4];
                Logger::log(&mut log, (q.decode(temp.unwrap()).unwrap()).encode_utf8(&mut tmp), level2);
            }
        }
        drop(q);*/
        x86_64::instructions::hlt();
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}