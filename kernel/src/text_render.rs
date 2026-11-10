use bootloader_api::info::FrameBuffer;
use font8x8::{UnicodeFonts, BASIC_FONTS};
use crate::serial::emergency_message;

/*pub fn draw_character(frame_buffer: &mut FrameBuffer, y :usize, x: usize, c: char){
    let character_font = BASIC_FONTS.get(c);
    let info = frame_buffer.info();
    let bytes_per_pixel = info.bytes_per_pixel;
    let buffer = frame_buffer.buffer_mut();
    if character_font.is_some() {
        let character = character_font.unwrap();
        for i in 0..8{
            if(character[i] & 128 > 0){
                buffer[((y + i) * info.stride + x + 7)*bytes_per_pixel] = 255;
            }
            if(character[i] & 64 > 0){
                buffer[((y + i) * info.stride + 6 + x)*bytes_per_pixel] = 255;
            }
            if(character[i] & 32 > 0){
                buffer[((y + i) * info.stride + 5 + x)*bytes_per_pixel] = 255;
            }
            if(character[i] & 16 > 0){
                buffer[((y + i)* info.stride + 4 + x)*bytes_per_pixel] = 255;
            }
            if(character[i] & 8 > 0){
                buffer[((y + i) * info.stride + 3 + x)*bytes_per_pixel] = 255;
            }
            if(character[i] & 4 > 0){
                buffer[((y + i) * info.stride + 2 + x)*bytes_per_pixel] = 255;
            }
            if(character[i] & 2 > 0){
                buffer[((y + i) * info.stride + 1 + x)*bytes_per_pixel] = 255;
            }
            if(character[i] & 1 > 0){
                buffer[((y + i) * info.stride + x)*bytes_per_pixel]  = 255;
            }
        }
    }

}*/

pub const scale: i32 = 3;
pub fn draw_character(frame_buffer: &mut FrameBuffer, y: usize, x: usize, c: char) {
    let character_font = BASIC_FONTS.get(c);
    let info = frame_buffer.info();
    let bytes_per_pixel = info.bytes_per_pixel;
    let buffer = frame_buffer.buffer_mut();
    if let Some(character) = character_font {
        for row in 0..8 {
            for col in 0..8 {
                if (character[row] & (1u8 << col)) != 0 {
                    let pixel_x = x + col * (scale as usize) ;
                    let pixel_y = y + row * (scale as usize) ;
                    for dy in 0..scale {
                        for dx in 0..scale {
                            let screen_x = pixel_x + (dx as usize);
                            let screen_y = pixel_y + (dy as usize);
                            if screen_x < info.width && screen_y < info.height {
                                let offset =
                                    (screen_y * info.stride + screen_x)
                                        * bytes_per_pixel;
                                buffer[offset] = 255;
                            }
                        }
                    }
                }
            }
        }
    }
}


pub fn delete_character(
    frame_buffer: &mut FrameBuffer,y: usize, x: usize ) {
    let info = frame_buffer.info();
    let bytes_per_pixel = info.bytes_per_pixel;
    let buffer = frame_buffer.buffer_mut();
    let size = 8 * scale;
    for dy in 0..size {
        for dx in 0..size {
            let screen_x = x + (dx as usize);
            let screen_y = y + (dy as usize);
            if screen_x < info.width && screen_y < info.height {
                let offset = (screen_y * info.stride + screen_x) * bytes_per_pixel;
                for byte in 0..bytes_per_pixel {
                    buffer[offset + byte] = 0;
                }
            }
        }
    }
}
