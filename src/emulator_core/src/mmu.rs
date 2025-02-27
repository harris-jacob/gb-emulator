mod interrupts;
mod joypad;
pub mod ppu;
mod timer;

use std::sync::Arc;

use ppu::BGMapSelection;
use ppu::SpritePaletteSelection;
use ppu::ViewportRegister;
use ppu::WindowPositionRegister;

use crate::cartridge::Cartridge;
pub use crate::mmu::ppu::Color;
pub use crate::mmu::ppu::Renderer;
pub use crate::mmu::ppu::PPU;
pub use interrupts::Interrupt;
pub use interrupts::Interrupts;
pub use joypad::Button;
pub use joypad::Joypad;

#[cfg(test)]
pub use crate::mmu::ppu::TestRenderer;

pub struct MMU {
    cartridge: Box<dyn Cartridge>,
    empty: [u8; 0x60],
    hram: [u8; 0x80],
    io: [u8; 0x80],
    joypad: Arc<Joypad>,
    pub ppu: PPU,
    pub serial: Vec<char>,
    timer: timer::Timer,
    wrams: [u8; 0x2000],
    pub interrupts: Interrupts,
}

impl MMU {
    pub fn new(ppu: PPU, cartridge: Box<dyn Cartridge>, joypad: Arc<Joypad>) -> MMU {
        let mut mmu = MMU {
            cartridge,
            empty: [0; 0x60],
            hram: [0; 0x80],
            io: [0; 0x80],
            ppu,
            serial: Vec::new(),
            timer: timer::Timer::new(),
            wrams: [0; 0x2000],
            interrupts: Interrupts::new(),
            joypad,
        };

        // Pretend we loaded the boot rom values
        mmu.write_u8(0xff05, 0);
        mmu.write_u8(0xff06, 0);
        mmu.write_u8(0xff07, 0);
        mmu.write_u8(0xff10, 0x80);
        mmu.write_u8(0xff11, 0xbf);
        mmu.write_u8(0xff14, 0xbf);
        mmu.write_u8(0xff16, 0x3f);
        mmu.write_u8(0xff19, 0xbf);
        mmu.write_u8(0xff1b, 0xff);
        mmu.write_u8(0xff1e, 0xbf);
        mmu.write_u8(0xff21, 0);
        mmu.write_u8(0xff22, 0);
        mmu.write_u8(0xff23, 0xbf);
        mmu.write_u8(0xff24, 0x77);
        mmu.write_u8(0xff25, 0xF3);
        mmu.write_u8(0xff26, 0xF1);
        mmu.write_u8(0xff40, 0x91);
        mmu.write_u8(0xff42, 0);
        mmu.write_u8(0xff43, 0);
        mmu.write_u8(0xff45, 0);
        mmu.write_u8(0xff47, 0xfc);
        mmu.write_u8(0xff48, 0xff);
        mmu.write_u8(0xff49, 0xff);
        mmu.write_u8(0xff4a, 0);
        mmu.write_u8(0xff4b, 0);
        mmu.write_u8(0xffff, 0);

        mmu
    }

    pub(crate) fn read_u8(&self, addr: u16) -> u8 {
        match addr {
            0..=0x7FFF => self.cartridge.read_rom(addr),
            0x8000..=0x97FF => self.ppu.read_tiledata(addr - 0x8000),
            0x9800..=0x9BFF => self.ppu.read_bg_map(BGMapSelection::Map0, addr - 0x9800),
            0x9C00..=0x9FFF => self.ppu.read_bg_map(BGMapSelection::Map1, addr - 0x9C00),
            0xA000..=0xBFFF => self.cartridge.read_ram(addr),
            0xC000..=0xDFFF => self.wrams[(addr - 0xC000) as usize],
            0xE000..=0xFDFF => self.wrams[(addr - 0xE000) as usize], // Echo ram
            0xFE00..=0xFE9F => self.ppu.read_oam(addr - 0xFE00),
            0xFEA0..=0xFEFF => self.empty[(addr - 0xFEA0) as usize],
            0xFF00 => self.joypad.read(), // Joypad
            0xFF01..=0xFF02 => self.io[(addr - 0xFF00) as usize], // Serial transfer,
            0xFF03 => 0,                  // Nothing
            0xFF04 => self.timer.read_divider(),
            0xFF05 => self.timer.read_counter(),
            0xFF06 => self.timer.read_modulo(),
            0xFF07 => self.timer.read_control(),
            0xFF08..=0xFF0E => 0, // Nothing
            0xFF0F => self.interrupts.read_interrupt_flag(),
            0xFF10..=0xFF26 => self.io[(addr - 0xFF00) as usize], // Audio
            0xFF27..=0xFF2F => 0,                                 // Nothing
            0xFF30..=0xFF3F => self.io[(addr - 0xFF00) as usize], // Wave pattern
            0xFF40 => self.ppu.read_lcdc(),
            0xFF41 => self.ppu.read_lcd_stat(),
            0xFF42 => self.ppu.read_background_viewport(ViewportRegister::SCY),
            0xFF43 => self.ppu.read_background_viewport(ViewportRegister::SCX),
            0xFF44 => self.ppu.read_ly(),
            0xFF45 => self.ppu.read_lyc(),
            0xFF46 => 0, // DMA transfer
            0xFF47 => self.ppu.read_background_palette(),
            0xFF48 => self
                .ppu
                .read_sprite_palette(SpritePaletteSelection::Palette0),
            0xFF49 => self
                .ppu
                .read_sprite_palette(SpritePaletteSelection::Palette1),
            0xFF4A => self.ppu.read_window_position(WindowPositionRegister::WY),
            0xFF4B => self.ppu.read_window_position(WindowPositionRegister::WX),
            0xFF4C..=0xFF7F => 0, // Nothing
            0xFF80..=0xFFFE => self.hram[(addr - 0xFF80) as usize],
            0xFFFF => self.interrupts.read_interrupt_enabled(),
        }
    }

    pub(crate) fn read_u16(&self, addr: u16) -> u16 {
        let low = self.read_u8(addr) as u16;
        let high = self.read_u8(addr + 1) as u16;

        (high << 8) | low
    }

    pub(crate) fn write_u8(&mut self, addr: u16, value: u8) {
        match addr {
            0..=0x7FFF => self.cartridge.write_rom(addr, value),
            0x8000..=0x97FF => self.ppu.write_tiledata(addr - 0x8000, value),
            0x9800..=0x9BFF => self
                .ppu
                .write_bg_map(BGMapSelection::Map0, addr - 0x9800, value),
            0x9C00..=0x9FFF => self
                .ppu
                .write_bg_map(BGMapSelection::Map1, addr - 0x9C00, value),
            0xA000..=0xBFFF => self.cartridge.write_ram(addr, value),
            0xC000..=0xDFFF => self.wrams[(addr - 0xC000) as usize] = value,
            0xE000..=0xFDFF => self.wrams[(addr - 0xE000) as usize] = value,
            0xFE00..=0xFE9F => self.ppu.write_oam(addr - 0xFE00, value),
            0xFEA0..=0xFEFF => self.empty[(addr - 0xFEA0) as usize] = value,
            0xFF00 => self.joypad.write(value),
            0xFF01 => self.serial.push(value as char),
            0xFF02 => self.io[(addr - 0xFF00) as usize] = value,
            0xFF03 => {} // Nothing
            0xFF04 => self.timer.write_divider(value),
            0xFF05 => self.timer.write_counter(value),
            0xFF06 => self.timer.write_modulo(value),
            0xFF07 => self.timer.write_control(value),
            0xFF08..=0xFF0E => {} // Nothing
            0xFF0F => self.interrupts.write_interrupt_flag(value),
            0xFF10..=0xFF26 => self.io[(addr - 0xFF00) as usize] = value, // Audio
            0xFF27..=0xFF2F => {}                                         // Nothing
            0xFF30..=0xFF3F => self.io[(addr - 0xFF00) as usize] = value, // Wave pattern
            0xFF40 => self.ppu.write_lcdc(value),
            0xFF41 => self.ppu.write_lcd_stat(value),
            0xFF42 => self
                .ppu
                .write_background_viewport(ViewportRegister::SCY, value),
            0xFF43 => self
                .ppu
                .write_background_viewport(ViewportRegister::SCX, value),
            0xFF44 => {} // LY is read-only
            0xFF45 => self.ppu.write_lyc(value),
            0xFF46 => self.dma_transfer(value),
            0xFF47 => self.ppu.write_background_palette(value),
            0xFF48 => self
                .ppu
                .write_sprite_palette(SpritePaletteSelection::Palette0, value),
            0xFF49 => self
                .ppu
                .write_sprite_palette(SpritePaletteSelection::Palette1, value),
            0xFF4A => self
                .ppu
                .write_window_position(WindowPositionRegister::WY, value),
            0xFF4B => self
                .ppu
                .write_window_position(WindowPositionRegister::WX, value),
            0xFF4C..=0xFF7F => {} // Nothing
            0xFF80..0xFFFF => self.hram[(addr - 0xFF80) as usize] = value,
            0xFFFF => self.interrupts.write_interrupt_enabled(value),
        }
    }

    pub(crate) fn write_u16(&mut self, addr: u16, value: u16) {
        let low = value as u8;
        let high = (value >> 8) as u8;

        self.write_u8(addr, low);
        self.write_u8(addr + 1, high);
    }

    pub(crate) fn step(&mut self, m_cycles: u8) {
        self.timer.step(m_cycles);
        self.ppu.step(m_cycles);
        self.cartridge.step(m_cycles);

        if self.timer.interrupt_request {
            self.interrupts.request_interrupt(Interrupt::Timer);
            self.timer.interrupt_request = false;
        }

        if self.ppu.interrupt_request.stat {
            self.interrupts.request_interrupt(Interrupt::LCDStat);
            self.ppu.interrupt_request.stat = false;
        }

        if self.ppu.interrupt_request.vblank {
            self.interrupts.request_interrupt(Interrupt::VBlank);
            self.ppu.interrupt_request.vblank = false;
        }

        if self.joypad.interrupt_requested() {
            self.interrupts.request_interrupt(Interrupt::Joypad);
            self.joypad.reset_interrupt();
        }
    }

    // TODO: is there a better API for this?
    pub fn save(&mut self) {
        self.cartridge.save();
    }

    fn dma_transfer(&mut self, value: u8) {
        let start_address: u16 = (value as u16) << 8;

        for offset in 0..160 {
            self.ppu
                .write_oam(offset, self.read_u8(start_address + offset))
        }
    }
}
