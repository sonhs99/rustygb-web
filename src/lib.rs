mod hardware;
use wasm_bindgen::prelude::*;

const RAM_SIZE: usize = 0x8000;
const FRAME_WIDTH: usize = 144;
const FRAME_HEIGHT: usize = 160;
const FRAME_SIZE: usize = FRAME_WIDTH * FRAME_HEIGHT;

pub static mut frame_buffer: [u8; FRAME_SIZE * 4] = [0; FRAME_SIZE * 4];
pub static mut cross_button: u8 = 0;
pub static mut ab_button: u8 = 0;

#[wasm_bindgen]
extern "C" {
    fn alert(str: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(str: &str);
}

#[wasm_bindgen]
pub fn get_frame_buffer_ptr() -> u32 {
    unsafe { frame_buffer.as_ptr() as u32 }
}

#[wasm_bindgen]
pub fn press_button(key: u8) {
    unsafe {
        if key < 4 {
            cross_button |= 1 << key;
        } else {
            ab_button |= 1 << (key - 4);
        }
        log(&format!("{:X?}", (cross_button, ab_button)));
    }
}

#[wasm_bindgen]
pub fn release_button(key: u8) {
    unsafe {
        if key < 4 {
            cross_button &= !(1 << key);
        } else {
            ab_button &= !(1 << (key - 4));
        }
        log(&format!("{:X?}", (cross_button, ab_button)));
    }
}
