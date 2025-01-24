mod ppu;
mod timer;

use crate::core::cartridge::Cartridge;
use crate::core::data::Interrupt;
pub use crate::core::mmu::ppu::Pixel;
pub use crate::core::mmu::ppu::Renderer;
pub use crate::core::mmu::ppu::PPU;

#[cfg(test)]
pub use crate::core::mmu::ppu::TestRenderer;

pub struct MMU {
    cartridge: Box<dyn Cartridge>,
    empty: [u8; 0x60],
    hram: [u8; 0x80],
    ie: u8,
    io: [u8; 0x80],
    ppu: PPU,
    pub serial: Vec<char>,
    sprites: [u8; 0xA0],
    timer: timer::Timer,
    vram: [u8; 0x2000],
    wrams: [u8; 0x2000],
}

impl MMU {
    pub fn new(ppu: PPU, cartridge: Box<dyn Cartridge>) -> MMU {
        let mut mmu = MMU {
            cartridge,
            empty: [0; 0x60],
            hram: [0; 0x80],
            ie: 0,
            io: [0; 0x80],
            ppu,
            serial: Vec::new(),
            sprites: [0; 0xA0],
            timer: timer::Timer::new(),
            vram: [0; 0x2000],
            wrams: [0; 0x2000],
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
            0..0x8000 => self.cartridge.read_rom(addr),
            0x8000..0xA000 => self.vram[(addr - 0x8000) as usize],
            0xA000..0xC000 => self.cartridge.read_ram(addr),
            0xC000..0xE000 => self.wrams[(addr - 0xC000) as usize],
            0xE000..0xFE00 => self.wrams[(addr - 0xE000) as usize],
            0xFE00..0xFEA0 => self.sprites[(addr - 0xFE00) as usize],
            0xFEA0..0xFF00 => self.empty[(addr - 0xFEA0) as usize],
            0xFF00..0xFF80 => match addr {
                0xFF04..0xFF08 => self.timer.read(addr),
                // TODO: temp while we don't have an LCD
                0xFF44 => 0x90,
                _ => self.io[(addr - 0xFF00) as usize],
            },
            0xFF80..0xFFFF => self.hram[(addr - 0xFF80) as usize],
            0xFFFF => self.ie,
        }
    }

    pub(crate) fn read_u16(&self, addr: u16) -> u16 {
        let low = self.read_u8(addr) as u16;
        let high = self.read_u8(addr + 1) as u16;

        (high << 8) | low
    }

    pub(crate) fn write_u8(&mut self, addr: u16, value: u8) {
        match addr {
            0..0x8000 => self.cartridge.write_rom(addr, value),
            0x8000..0xA000 => self.vram[(addr - 0x8000) as usize] = value,
            0xA000..0xC000 => self.cartridge.write_ram(addr, value),
            0xC000..0xE000 => self.wrams[(addr - 0xC000) as usize] = value,
            0xE000..0xFE00 => self.wrams[(addr - 0xE000) as usize] = value,
            0xFE00..0xFEA0 => self.sprites[(addr - 0xFE00) as usize] = value,
            0xFEA0..0xFF00 => self.empty[(addr - 0xFEA0) as usize] = value,
            // TODO: review
            0xFF00..0xFF80 => match addr {
                0xFF01 => self.serial.push(value as char),
                0xFF04..0xFF08 => self.timer.write(addr, value),
                _ => self.io[(addr - 0xFF00) as usize] = value,
            },
            0xFF80..0xFFFF => self.hram[(addr - 0xFF80) as usize] = value,
            0xFFFF => self.ie = value,
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
    }

    pub(crate) fn interrupts_requested(&self) -> Option<Interrupt> {
        if self.timer.interrupt_request {
            return Some(Interrupt::Timer);
        } else {
            return None;
        }
    }
}
