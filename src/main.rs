use std::{cell::UnsafeCell, fs::File, io::Read, sync::Arc};

const WIDTH: usize = 144;
const HEIGHT: usize = 160;

fn main() {
    let display = Arc::new(WindowDisplay::new());

    let filename = "./roms/cpu_instrs/cpu_instrs.gb";
    let mut fp = File::open(filename).expect("Should exist");
    let mut data = Vec::new();
    fp.read_to_end(&mut data).expect("Should read");

    let cartridge = emulator::create_cartridge(data);
    let ppu = emulator::PPU::new(display.clone());
    let mmu = emulator::MMU::new(ppu, cartridge);

    let mut cpu = emulator::CPU::new(mmu);

    display.run();
}

struct WindowDisplay {
    buffer: UnsafeCell<Vec<u32>>,
}

impl WindowDisplay {
    pub fn new() -> Self {
        Self {
            buffer: UnsafeCell::new(vec![0; WIDTH * HEIGHT]),
        }
    }

    pub fn run(&self) {
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

        window.set_target_fps(60);

        while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
            let value = unsafe { &*self.buffer.get() };
            window.update_with_buffer(value, WIDTH, HEIGHT).unwrap()
        }
    }

    fn color(color: emulator::Color) -> u32 {
        match color {
            emulator::Color::White => 0xFFFFFF,
            emulator::Color::LightGray => 0x454545,
            emulator::Color::DarkGray => 0xA8A8A8,
            emulator::Color::Black => 0,
        }
    }
}

impl emulator::Renderer for WindowDisplay {
    fn render(&self, buffer: [emulator::Color; 160 * 144]) {
        unsafe {
            let value = self.buffer.get();
            *value = buffer.into_iter().map(Self::color).collect::<Vec<u32>>();
        }
    }
}
