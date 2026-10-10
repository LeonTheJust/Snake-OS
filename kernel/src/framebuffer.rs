use bootloader_api::info::FrameBuffer;
pub(crate) fn drawPixel(framebuffer: &mut FrameBuffer, x: usize, y: usize, color: u8) {
    let info = framebuffer.info();
    let buffer = framebuffer.buffer_mut();

    let pixel = y * info.stride + x;

    let offset = pixel * info.bytes_per_pixel;

    if offset + 3 < buffer.len() {
        buffer[offset] = color;
        buffer[offset + 1] = color;
        buffer[offset + 2] = color;
    }



}

pub(crate) fn drawRect(framebuffer: &mut FrameBuffer, xstart: usize, ystart: usize, xend: usize, yend: usize, color: u8) {
    let info = framebuffer.info();
    let buffer = framebuffer.buffer_mut();

    for y in ystart..yend {
        for x in xstart..xend {
            let pixel = y * info.stride + x;
            let offset = pixel * info.bytes_per_pixel;
            if offset + 3 < buffer.len() {
                buffer[offset] = color;
                buffer[offset + 1] = color;
                buffer[offset + 2] = color;
            }
        }
    }
}

pub(crate) fn drawCircle(framebuffer: &mut FrameBuffer, x_center : usize, y_center: usize, radius: usize, color: u8) {
    let info = framebuffer.info();
    let buffer = framebuffer.buffer_mut();
    let PI = 3.1415926535;

    let x = x_center - radius;
    for i in 0..360{

    }

}

pub(crate) fn drawLineX(framebuffer: &mut FrameBuffer, xstart: usize, ystart: usize, xend: usize, color: u8) {
    let info = framebuffer.info();
    let buffer = framebuffer.buffer_mut();

    for x in xstart..xend {
        let pixel = ystart * info.stride + x;
        let offset = pixel * info.bytes_per_pixel;
        if offset + 3 < buffer.len() {
            buffer[offset] = color;
            buffer[offset + 1] = color;
            buffer[offset + 2] = color;
        }
    }
}

pub(crate) fn drawLineY(framebuffer: &mut FrameBuffer, ystart: usize, yend: usize, x: usize, color: u8) {
    let info = framebuffer.info();
    let buffer = framebuffer.buffer_mut();

    for i in ystart..yend {
        let pixel = x * info.stride + i;
        let offset = pixel * info.bytes_per_pixel;
        if offset + 3 < buffer.len() {
            buffer[offset] = color;
            buffer[offset + 1] = color;
            buffer[offset + 2] = color;
        }
    }
}

pub(crate) fn clear(framebuffer: &mut FrameBuffer) {
    let info = framebuffer.info();
    let buffer = framebuffer.buffer_mut();
    
    for y in 0..info.height {
        for x in 0..info.width {
            let pixel = y * info.stride + x;
            let offset = pixel * info.bytes_per_pixel;
            if offset + 3 < buffer.len() {
                buffer[offset] = 0;
                buffer[offset + 1] = 0;
                buffer[offset + 2] = 0;
            }
        }
    }
}

