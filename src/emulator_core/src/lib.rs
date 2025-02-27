mod cartridge;
mod cpu;
mod mmu;
mod registers;

pub use cartridge::create_cartridge;
pub use cartridge::Cartridge;
pub use cartridge::CartridgePersistence;
pub use cpu::CPU;
pub use mmu::Button;
pub use mmu::Color;
pub use mmu::Joypad;
pub use mmu::Renderer;
pub use mmu::MMU;
pub use mmu::PPU;

use std::time::Duration;
use std::time::Instant;

pub struct Emulator {
    cpu: CPU,
    limiter: Limiter,
}

pub struct EmulatorHandle {
    shutdown_sender: std::sync::mpsc::Sender<()>,
    join_handle: std::thread::JoinHandle<()>,
}

impl EmulatorHandle {
    pub fn shutdown(self) {
        // Signal shutdown and join the thread.
        let _ = self.shutdown_sender.send(());
        let _ = self.join_handle.join();
    }
}

impl Emulator {
    pub fn new(cpu: CPU) -> Self {
        let limiter = Limiter::new();
        Self { cpu, limiter }
    }

    /// Spawns a new thread that runs the emulator. Returns handle which
    /// can be used to shutdown the emulator.
    pub fn spawn(mut self) -> EmulatorHandle {
        let (shutdown_sender, shutdown_receiver) = std::sync::mpsc::channel::<()>();

        // Move the emulator into its own thread.
        let join_handle = std::thread::spawn(move || {
            // Main loop: execute cycles until shutdown signal is received.
            while shutdown_receiver.try_recv().is_err() {
                self.step();
            }
            // Before exiting, save state.
            self.save();
        });

        EmulatorHandle {
            shutdown_sender,
            join_handle,
        }
    }

    fn step(&mut self) {
        let cycles = self.cpu.step();
        self.limiter.step(cycles);
    }

    fn save(&mut self) {
        self.cpu.mmu.save();
    }
}

/// Limiter designed to keeo the emulator running at the correct clock speed.
/// The limit function does this by counting the number of cycles executed in a
/// frame (assuming a target of 60 FPS) and once the number of cycles reaches the
/// maximum allowed value for the frame, the function blocks until the next frame
/// can begin. This isn't a 'correct' emulation of the CPU speed but its good
/// enough for our purposes.
pub struct Limiter {
    next_frame: Instant,
    frame_cycles: u64,
}

const FPS: u64 = 60;
const CYCLES_PER_SECOND: u64 = 4194304 / 4; // Hz
const CYCLES_PER_FRAME: u64 = CYCLES_PER_SECOND / FPS;
const TARGET_FRAME_DURATION: Duration = Duration::from_millis(1000 / FPS);

impl Limiter {
    pub fn new() -> Self {
        Self {
            next_frame: Instant::now() + TARGET_FRAME_DURATION,
            frame_cycles: 0,
        }
    }

    pub fn step(&mut self, cycles: u8) {
        self.frame_cycles += cycles as u64;

        if self.frame_cycles < CYCLES_PER_FRAME {
            return;
        }

        let now = Instant::now();

        if self.next_frame > now {
            std::thread::sleep(self.next_frame - now);
        }

        self.frame_cycles = 0;
        self.next_frame = now + TARGET_FRAME_DURATION;
    }
}
