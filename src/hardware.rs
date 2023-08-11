use rustygb::{self, Cartridge, System};
use wasm_bindgen::prelude::*;

pub struct Hardware;

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

impl rustygb::Hardware for Hardware {
    fn draw_framebuffer(&mut self, frame_buffer: &rustygb::FrameBuffer) {
        log("draw");
        for idx in 0..crate::FRAME_SIZE {
            unsafe {
                crate::frame_buffer[idx] = frame_buffer.pixels[idx];
            }
        }
    }
    fn get_keys(&mut self) -> (u8, u8) {
        log("control");
        (255, 255)
    }
    fn is_active(&mut self) -> bool {
        log("active");
        true
    }
    fn update(&mut self) {
        log("update");
    }
}

#[wasm_bindgen]
impl Emulator {
    pub fn new(rom: Vec<u8>) -> Emulator {
        let cart = Cartridge::new(rom, vec![0; crate::RAM_SIZE]);
        let hw = Hardware::new();
        Emulator {
            system: System::new(cart, hw),
        }
    }

    pub fn step(&mut self) {
        let value = self.system.step();
        log(&format!("{}", value));
    }
}
