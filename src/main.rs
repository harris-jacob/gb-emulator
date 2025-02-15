use std::{cell::UnsafeCell, fs::File, io::Read, sync::Arc, thread};

use emulator::Joypad;
use minifb::InputCallback;

const WIDTH: usize = 160;
const HEIGHT: usize = 144;

fn main() {
    let display = Arc::new(WindowDisplay::new());

    let filename = "./roms/dr-mario.gb";
    let mut fp = File::open(filename).expect("Should exist");
    let mut data = Vec::new();
    fp.read_to_end(&mut data).expect("Should read");

    let cartridge = emulator::create_cartridge(data);
    let ppu = emulator::PPU::new(display.clone());
    let joypad = Arc::new(emulator::Joypad::new());
    let mmu = emulator::MMU::new(ppu, cartridge, joypad.clone());

    let cpu = emulator::CPU::new(mmu);
    let mut emulator = emulator::Emulator::new(cpu);

    thread::spawn(move || {
        emulator.run();
    });

    display.run(joypad)
}

struct WindowDisplay {
    buffer: UnsafeCell<[u32; WIDTH * HEIGHT]>,
}

unsafe impl Sync for WindowDisplay {}
unsafe impl Send for WindowDisplay {}

struct InputTest {
    joypad: Arc<Joypad>,
}

impl InputTest {
    pub fn new(joypad: Arc<Joypad>) -> Self {
        Self { joypad }
    }
}

impl InputCallback for InputTest {
    fn add_char(&mut self, _uni_char: u32) {
        ()
    }

    fn set_key_state(&mut self, key: minifb::Key, state: bool) {
        match (key, state) {
            (minifb::Key::A, true) => {
                println!("Pressing Start");
                self.joypad.button_down(emulator::Button::Start)
            },
            (minifb::Key::A, false) => self.joypad.button_release(emulator::Button::A),
            (_, _) => {}
        }
    }
}

impl WindowDisplay {
    pub fn new() -> Self {
        Self {
            buffer: UnsafeCell::new([0; WIDTH * HEIGHT]),
        }
    }

    pub fn run(&self, joypad: Arc<Joypad>) {
        let mut window = minifb::Window::new(
            "GB Emulator",
            WIDTH,
            HEIGHT,
            minifb::WindowOptions {
                scale: minifb::Scale::X4,
                ..minifb::WindowOptions::default()
            },
        )
        .unwrap();

        window.set_input_callback(Box::new(InputTest::new(joypad)));

        window.set_target_fps(60);

        while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
            let value = unsafe { &*self.buffer.get() };
            window.update_with_buffer(value, WIDTH, HEIGHT).unwrap()
        }
    }
}

impl emulator::Renderer for WindowDisplay {
    fn render(&self, buffer: [u32; 160 * 144]) {
        unsafe {
            let value = self.buffer.get();
            *value = buffer;
        }
    }
}
