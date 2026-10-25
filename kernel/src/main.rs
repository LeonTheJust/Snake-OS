#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

mod framebuffer;
mod serial;
mod logger;
mod interrupts;
mod gdt;
mod memory;

use bootloader_api::{entry_point, BootInfo};
use uart_16550::{Uart16550, backend::PioBackend};
use core::sync::atomic::{AtomicBool, Ordering};
use core::sync::atomic::Ordering::Relaxed;
use bootloader_api::info::MemoryRegionKind;
use x86_64::structures::paging::FrameAllocator;
use crate::interrupts::InterruptHandler;
use crate::logger::Logger;
use crate::memory::MemoryAllocator;
use crate::serial::{emergency_message, u64_to_str, Serial};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
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

    let mut mem = MemoryAllocator::new(region);

    loop {
        core::hint::spin_loop();
        //Logger::log(&mut log, serial::u64_to_str(interrupts::tick_counter.load(Relaxed), &mut buffer), level);
        //Logger::log(&mut log, serial::u64_to_str(interrupts::KEYBOARD.load(Relaxed) as u64, &mut buffer), level);
        x86_64::instructions::hlt();
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}