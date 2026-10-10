#![no_std]
#![no_main]

mod framebuffer;
mod serial;
mod logger;
mod interrups;

use bootloader_api::{entry_point, BootInfo};
use uart_16550::{Uart16550, backend::PioBackend};
use crate::logger::Logger;
use crate::serial::Serial;

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
    let level = logger::LogLevel::new(1);
    Logger::log(&mut log,"Hello", level);

    let _ = (width, height);
    framebuffer::clear(framebuffer);
    loop {
        core::hint::spin_loop();
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}