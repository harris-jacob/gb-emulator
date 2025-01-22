mod timer;
mod ppu;

use crate::core::cartridge::Cartridge;
use crate::core::data::Interrupt;

pub struct MMU {
    cartridge: Box<dyn Cartridge>,
    vram: [u8; 0x2000],
    wrams: [u8; 0x2000],
    sprites: [u8; 0xA0],
    empty: [u8; 0x60],
    // TODO: this will be replaced by the individual registers
    io: [u8; 0x80],
    hram: [u8; 0x80],
    ie: u8,
    timer: timer::Timer,
    pub serial: Vec<char>,
}

impl MMU {
    pub fn new(cartridge: Box<dyn Cartridge>) -> MMU {
        let mut mmu = MMU {
            cartridge,
            vram: [0; 0x2000],
            wrams: [0; 0x2000],
            sprites: [0; 0xA0],
            empty: [0; 0x60],
            io: [0; 0x80],
            hram: [0; 0x80],
            ie: 0,
            timer: timer::Timer::new(),
            serial: Vec::new(),
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

    pub fn read_u8(&self, addr: u16) -> u8 {
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

    pub fn read_u16(&self, addr: u16) -> u16 {
        let low = self.read_u8(addr) as u16;
        let high = self.read_u8(addr + 1) as u16;

        (high << 8) | low
    }

    pub fn write_u8(&mut self, addr: u16, value: u8) {
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

    pub fn write_u16(&mut self, addr: u16, value: u16) {
        let low = value as u8;
        let high = (value >> 8) as u8;

        self.write_u8(addr, low);
        self.write_u8(addr + 1, high);
    }

    pub fn step(&mut self, m_cycles: u8) {
        self.timer.step(m_cycles);
    }

    pub fn interrupts_requested(&self) -> Option<Interrupt> {
        if self.timer.interrupt_request {
            return Some(Interrupt::Timer);
        } else {
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::cartridge::MockCartridge;

    use super::*;

    #[test]
    fn read_rom_space() {
        let mut cartridge = MockCartridge::new();

        cartridge
            .expect_read_rom()
            .times(0x8000 * 3 - 2)
            .returning(|_| 0);

        cartridge
            .expect_write_rom()
            .times(0x8000 * 3 - 2)
            .returning(|_, _| ());

        let mut mmu = MMU::new(Box::new(cartridge));

        for i in 0..0x8000 {
            mmu.write_u8(i, 0);
            assert_eq!(mmu.read_u8(i), 0);
        }

        for i in 0..0x7FFF {
            mmu.write_u16(i, 0);
            assert_eq!(mmu.read_u16(i), 0);
        }
    }

    #[test]
    fn read_vram_space() {
        let mut mmu = mock_mmu();

        for i in 0x8000..0xA000 {
            mmu.write_u8(i, 0);
            assert_eq!(mmu.read_u8(i), 0);
        }

        for i in 0x8000..0x9FFF {
            mmu.write_u16(i, 0xFFFF);
            assert_eq!(mmu.read_u16(i), 0xFFFF);
        }
    }

    #[test]
    fn read_iram_space() {
        let mut cartridge = MockCartridge::new();

        cartridge
            .expect_read_ram()
            .times(0x2000 * 3 - 2)
            .returning(|_| 0);

        cartridge
            .expect_write_ram()
            .times(0x2000 * 3 - 2)
            .returning(|_, _| ());

        let mut mmu = MMU::new(Box::new(cartridge));

        for i in 0xA000..0xC000 {
            mmu.write_u8(i, 0);
            assert_eq!(mmu.read_u8(i), 0);
        }

        for i in 0xA000..0xBFFF {
            mmu.write_u16(i, 0);
            assert_eq!(mmu.read_u16(i), 0);
        }
    }

    #[test]
    fn read_wram_space() {
        let mut mmu = mock_mmu();

        for i in 0xC000..0xE000 {
            mmu.write_u8(i, 0);
            assert_eq!(mmu.read_u8(i), 0);
        }

        for i in 0xC000..0xDFFF {
            mmu.write_u16(i, 0xFFFF);
            assert_eq!(mmu.read_u16(i), 0xFFFF);
        }
    }

    #[test]
    fn read_wram_echo_space() {
        let mut mmu = mock_mmu();

        for i in 0xE000..0xFE00 {
            mmu.write_u8(i, 0);
            assert_eq!(mmu.read_u8(i), 0);
        }

        for i in 0xE000..0xFDFF {
            mmu.write_u16(i, 0xFFFF);
            assert_eq!(mmu.read_u16(i), 0xFFFF);
        }
    }

    #[test]
    fn read_sprite_space() {
        let mut mmu = mock_mmu();

        for i in 0xFE00..0xFEA0 {
            mmu.write_u8(i, 0);
            assert_eq!(mmu.read_u8(i), 0);
        }

        for i in 0xFE00..0xFE9F {
            mmu.write_u16(i, 0xFFFF);
            assert_eq!(mmu.read_u16(i), 0xFFFF);
        }
    }

    #[test]
    fn read_empty_space() {
        let mut mmu = mock_mmu();

        for i in 0xFEA0..0xFF00 {
            mmu.write_u8(i, 0);
            assert_eq!(mmu.read_u8(i), 0);
        }

        for i in 0xFEA0..0xFEFF {
            mmu.write_u16(i, 0xFFFF);
            assert_eq!(mmu.read_u16(i), 0xFFFF);
        }
    }

    #[test]
    fn read_timer() {
        let mut mmu = mock_mmu();

        mmu.write_u8(0xFF04, 0x10);
        assert_eq!(mmu.read_u8(0xFF04), 0);

        for i in 0xFF05..0xFF08 {
            mmu.write_u8(i, 0x10);
            assert_eq!(mmu.read_u8(i), 0x10);
        }
    }

    #[test]
    fn read_hram_space() {
        let mut mmu = mock_mmu();

        for i in 0xFF80..0xFFFF {
            mmu.write_u8(i, 0);
            assert_eq!(mmu.read_u8(i), 0);
        }

        for i in 0xFF80..0xFFFF {
            mmu.write_u16(i, 0xFFFF);
            assert_eq!(mmu.read_u16(i), 0xFFFF);
        }
    }

    #[test]
    fn read_ie() {
        let mut mmu = mock_mmu();

        mmu.write_u8(0xFFFF, 0x10);
        assert_eq!(mmu.read_u8(0xFFFF), 0x10);
    }

    #[test]
    fn initialises_with_boot_rom() {
        let mmu = mock_mmu();

        assert_eq!(mmu.read_u8(0xff10), 0x80);
        assert_eq!(mmu.read_u8(0xff11), 0xbf);
        assert_eq!(mmu.read_u8(0xff48), 0xff);
    }

    #[test]
    fn step_steps_timer() {
        let mut mmu = mock_mmu();
        mmu.write_u8(0xFF07, 0b0000_0111);

        for _ in 0..=64 {
            mmu.step(1);
        }

        assert_eq!(mmu.read_u8(0xFF05), 1);
        assert_eq!(mmu.read_u8(0xFF04), 1);
    }

    #[test]
    fn timer_interrupt_request() {
        let mut mmu = mock_mmu();
        mmu.write_u8(0xFF07, 0b0000_0101);
        mmu.write_u8(0xFF05, 0xFF);

        for _ in 0..4 {
            mmu.step(1);
        }

        assert_eq!(mmu.interrupts_requested(), Some(Interrupt::Timer));
    }

    pub fn mock_mmu() -> MMU {
        let cartridge = MockCartridge::new();
        let mmu = MMU::new(Box::new(cartridge));

        mmu
    }
}
