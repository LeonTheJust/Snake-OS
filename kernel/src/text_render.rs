use bootloader_api::info::FrameBuffer;
use font8x8::{UnicodeFonts, BASIC_FONTS};
use crate::serial::emergency_message;

pub fn draw_character(frame_buffer: &mut FrameBuffer, y :usize, x: usize, c: char){
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

}