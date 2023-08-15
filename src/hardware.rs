use rustygb::{self, Cartridge, Pixel, System};
use wasm_bindgen::prelude::*;

pub struct Hardware;

static mut update_flag: bool = false;

#[wasm_bindgen]
pub struct Emulator {
    system: System,
}

#[wasm_bindgen]
extern "C" {
    fn alert(str: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(str: &str);
}

impl Hardware {
    pub fn new() -> Hardware {
        Hardware {}
    }
}

fn copy_buffer(frame_buffer: &mut [u8]) {
    unsafe {
        for idx in 0..crate::FRAME_SIZE * 4 {
            frame_buffer[idx] = crate::frame_buffer[idx];
        }
    }
}

impl rustygb::Hardware for Hardware {
    fn draw_framebuffer(&mut self, frame_buffer: &rustygb::FrameBuffer) {
        unsafe {
            update_flag = true;
            for idx in 0..crate::FRAME_SIZE {
                (
                    crate::frame_buffer[idx * 4],
                    crate::frame_buffer[idx * 4 + 1],
                    crate::frame_buffer[idx * 4 + 2],
                    crate::frame_buffer[idx * 4 + 3],
                ) = match frame_buffer.pixels[idx as usize] {
                    Pixel::Black => (0x00, 0x00, 0x00, 0xFF),
                    Pixel::Dark => (0x55, 0x55, 0x55, 0xFF),
                    Pixel::Bright => (0xAA, 0xAA, 0xAA, 0xFF),
                    Pixel::White => (0xFF, 0xFF, 0xFF, 0xFF),
                };
            }
            // log(&format!("{:X?}", crate::frame_buffer));
        }
    }
    fn get_keys(&mut self) -> (u8, u8) {
        unsafe {
            // log(&format!("{:X?}", (!crate::cross_button, !crate::ab_button)));
            (crate::cross_button, crate::ab_button)
        }
    }
    fn is_active(&mut self) -> bool {
        // log("active");
        true
    }
    fn update(&mut self) {
        // log("update");
    }
}

#[wasm_bindgen]
impl Emulator {
    pub fn new(rom: &[u8], frame_buffer: &mut [u8]) -> Emulator {
        let cart = Cartridge::new(rom.to_vec(), vec![0; crate::RAM_SIZE]);
        let hw = Hardware::new();
        Emulator {
            system: System::new(cart, hw),
        }
    }

    pub fn step(&mut self, frame_buffer: &mut [u8]) -> bool {
        unsafe {
            update_flag = false;
            let mut clock = 0;
            while clock < 0xFFFF && !update_flag {
                clock += self.system.step();
            }
            if update_flag {
                copy_buffer(frame_buffer);
            }
            update_flag
        }
    }
}
