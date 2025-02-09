use std::{cell::UnsafeCell, fs::File, io::Read, sync::Arc, thread};

const WIDTH: usize = 160;
const HEIGHT: usize = 144;

fn main() {
    let display = Arc::new(WindowDisplay::new());

    let filename = "./roms/mem_timing.gb";
    let mut fp = File::open(filename).expect("Should exist");
    let mut data = Vec::new();
    fp.read_to_end(&mut data).expect("Should read");

    let cartridge = emulator::create_cartridge(data);
    let ppu = emulator::PPU::new(display.clone());
    let mmu = emulator::MMU::new(ppu, cartridge);

    let cpu = emulator::CPU::new(mmu);
    let mut emulator = emulator::Emulator::new(cpu);

    thread::spawn(move || {
        emulator.run();
    });

    display.run()
}

struct WindowDisplay {
    buffer: UnsafeCell<[u32; WIDTH * HEIGHT]>,
}

unsafe impl Sync for WindowDisplay {}
unsafe impl Send for WindowDisplay {}

impl WindowDisplay {
    pub fn new() -> Self {
        Self {
            buffer: UnsafeCell::new([0; WIDTH * HEIGHT]),
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
}

impl emulator::Renderer for WindowDisplay {
    fn render(&self, buffer: [u32; 160 * 144]) {
        unsafe {
            let value = self.buffer.get();
            *value = buffer;
        }
    }
}
