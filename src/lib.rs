mod hardware;
use wasm_bindgen::prelude::*;

const RAM_SIZE: usize = 0x8000;
const FRAME_WIDTH: usize = 144;
const FRAME_HEIGHT: usize = 160;
const FRAME_SIZE: usize = FRAME_WIDTH * FRAME_HEIGHT;

pub static mut frame_buffer: [u32; FRAME_SIZE] = [0; FRAME_SIZE];

#[wasm_bindgen]
pub fn get_frame_buffer_ptr() -> u32 {
    unsafe { frame_buffer.as_ptr() as u32 }
}
#[wasm_bindgen]
pub fn get_frame_buffer_size() -> u32 {
    (FRAME_SIZE * 4) as u32
}
