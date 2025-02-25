use app::{JoypadManager, WindowBuffer, HEIGHT, WIDTH};
use emulator_core::Cartridge;
use std::{fs::File, io::Read, sync::Arc, thread};

pub fn main() {
    // Buffer written to by PPU and rendered by window
    let window_buffer = Arc::new(WindowBuffer::new());

    // Setup Emulator
    let cartridge = cartridge_from_filepath("./roms/pokemon-red.gb");
    let joypad = Arc::new(emulator_core::Joypad::new());
    let ppu = emulator_core::PPU::new(window_buffer.clone());
    let mmu = emulator_core::MMU::new(ppu, cartridge, joypad.clone());
    let cpu = emulator_core::CPU::new(mmu);

    let mut emulator = emulator_core::Emulator::new(cpu);

    let mut window = minifb::Window::new(
        "GB Emulator",
        WIDTH,
        HEIGHT,
        minifb::WindowOptions {
            scale: minifb::Scale::X4,
            ..minifb::WindowOptions::default()
        },
    )
    .expect("Should create window");

    // Joypad manager is registered to the window. Callbacks triggered when
    // buttons are pressed/released and routed to the gameboy's joypad.
    let joypad_manager = JoypadManager::new(joypad);
    window.set_input_callback(Box::new(joypad_manager));

    window.set_target_fps(60);

    thread::spawn(move || {
        emulator.run();
    });

    // Start the Window and update with the current value of the buffer
    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        window
            .update_with_buffer(window_buffer.buffer(), WIDTH, HEIGHT)
            .expect("Should update")
    }
}

fn cartridge_from_filepath(filepath: &str) -> Box<dyn Cartridge> {
    let mut fp = File::open(filepath).expect("Should exist");
    let mut data = Vec::new();
    fp.read_to_end(&mut data).expect("Should read");

    emulator_core::create_cartridge(data)
}
