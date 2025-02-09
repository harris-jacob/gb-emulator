mod core;

pub use core::create_cartridge;
pub use core::Color;
pub use core::EightBitRegister;
pub use core::Renderer;
pub use core::CPU;
pub use core::MMU;
pub use core::PPU;
use std::time::Duration;
use std::time::Instant;

pub struct Emulator {
    cpu: CPU,
}

impl Emulator {
    pub fn new(cpu: CPU) -> Self {
        Self { cpu }
    }

    pub fn run(&mut self) {
        let mut limiter = Limiter::new();

        loop {
            let cycles = self.cpu.step();
            limiter.step(cycles);
        }
    }

    pub fn dump_tilset(&mut self) -> Vec<u32> {
        let mut limiter = Limiter::new();
        let mut clock = 0;

        while clock < 4000000 {
            let cycles = self.cpu.step();
            clock += cycles as u32;
            limiter.step(cycles);
        }

        return self.cpu.mmu.ppu.dump_tileset();
    }
}

/// Limiter designed to keeo the emulator running at the correct clock speed.
/// The limit function does this by counting the number of cycles executed in a
/// frame (assuming a target of 60 FPS) and once the number of cycles reaches the
/// maximum allowed value for the frame, the function blocks until the next frame
/// can begin. This isn't a 'correct' emulation of the CPU speed but its good
/// enough for our purposes.
pub struct Limiter {
    frame_start: Instant,
    frame_cycles: u64,
}

const FPS: u64 = 60;
const CYCLES_PER_SECOND: u64 = 4194304; // Hz
const CYCLES_PER_FRAME: u64 = CYCLES_PER_SECOND / FPS;
const TARGET_FRAME_DURATION: Duration = Duration::from_millis(1000 / FPS);

impl Limiter {
    pub fn new() -> Self {
        Self {
            frame_start: Instant::now(),
            frame_cycles: 0,
        }
    }

    pub fn step(&mut self, cycles: u8) {
        self.frame_cycles += cycles as u64;

        if self.frame_cycles < CYCLES_PER_FRAME {
            return;
        }

        let now = Instant::now();
        let frame_duration = now - self.frame_start;

        let sleep_for = TARGET_FRAME_DURATION
            .checked_sub(frame_duration)
            .unwrap_or_else(|| Duration::from_secs(0));

        std::thread::sleep(sleep_for);
        self.next_frame();
    }

    fn next_frame(&mut self) {
        self.frame_cycles = 0;
        self.frame_start = Instant::now();
    }
}
