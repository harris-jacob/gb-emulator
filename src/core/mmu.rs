pub struct MMU {
    addr: [u8; 0x10000],
}

impl MMU {
    pub fn new() -> MMU {
        let mut mmu = MMU { addr: [0; 0x10000] };

        // Pretend we loaded the boot rom values
        mmu.addr[0xff05] = 0;
        mmu.addr[0xff06] = 0;
        mmu.addr[0xff07] = 0;
        mmu.addr[0xff10] = 0x80;
        mmu.addr[0xff11] = 0xbf;
        mmu.addr[0xff14] = 0xbf;
        mmu.addr[0xff16] = 0x3f;
        mmu.addr[0xff19] = 0xbf;
        mmu.addr[0xff1b] = 0xff;
        mmu.addr[0xff1e] = 0xbf;
        mmu.addr[0xff21] = 0;
        mmu.addr[0xff22] = 0;
        mmu.addr[0xff23] = 0xbf;
        mmu.addr[0xff24] = 0x77;
        mmu.addr[0xff25] = 0xF3;
        mmu.addr[0xff26] = 0xF1; // ??
        mmu.addr[0xff40] = 0x91;
        mmu.addr[0xff42] = 0;
        mmu.addr[0xff43] = 0;
        mmu.addr[0xff45] = 0;
        mmu.addr[0xff47] = 0xfc;
        mmu.addr[0xff48] = 0xff;
        mmu.addr[0xff49] = 0xff;
        mmu.addr[0xff4a] = 0;
        mmu.addr[0xff4b] = 0;
        mmu.addr[0xffff] = 0;

        mmu
    }

    pub fn read_u8(&self, addr: u16) -> u8 {
        self.addr[addr as usize]
    }

    pub fn read_u16(&self, addr: u16) -> u16 {
        let low = self.addr[addr as usize] as u16;
        let high = self.addr[(addr + 1) as usize] as u16;

        (high << 8) | low
    }

    pub fn write_u8(&mut self, addr: u16, value: u8) {
        self.addr[addr as usize] = value;
    }

    pub fn write_u16(&mut self, addr: u16, value: u16) {
        let low = value as u8;
        let high = (value >> 8) as u8;

        self.addr[addr as usize] = low;
        self.addr[(addr + 1) as usize] = high;
    }

    pub fn set_vblank_enable(&mut self) {
        self.addr[0xffff] |= 0x1;
    }
    pub fn set_lcd_enable(&mut self) {
        self.addr[0xffff] |= 0x1 << 1;
    }

    pub fn set_timer_enable(&mut self) {
        self.addr[0xffff] |= 0x1 << 2;
    }

    pub fn set_serial_enable(&mut self) {
        self.addr[0xffff] |= 0x1 << 3;
    }

    pub fn set_joypad_enable(&mut self) {
        self.addr[0xffff] |= 0x1 << 4;
    }

    pub fn reset_vblank_enable(&mut self) {
        self.addr[0xffff] &= !(0x1);
    }

    pub fn reset_lcd_enable(&mut self) {
        self.addr[0xffff] &= !(0x1 << 1);
    }

    pub fn reset_timer_enable(&mut self) {
        self.addr[0xffff] &= !(0x1 << 2);
    }

    pub fn reset_serial_enable(&mut self) {
        self.addr[0xffff] &= !(0x1 << 3);
    }

    pub fn reset_joypad_enable(&mut self) {
        self.addr[0xffff] &= !(0x1 << 4);
    }

    pub fn set_vblank(&mut self) {
        self.addr[0xff0f] |= 0x1;
    }

    pub fn set_lcd(&mut self) {
        self.addr[0xff0f] |= 0x1 << 1;
    }

    pub fn set_timer(&mut self) {
        self.addr[0xff0f] |= 0x1 << 2;
    }

    pub fn set_serial(&mut self) {
        self.addr[0xff0f] |= 0x1 << 3;
    }

    pub fn set_joypad(&mut self) {
        self.addr[0xff0f] |= 0x1 << 4;
    }

    pub fn reset_vblank(&mut self) {
        self.addr[0xff0f] &= !(0x1);
    }

    pub fn reset_lcd(&mut self) {
        self.addr[0xff0f] &= !(0x1 << 1);
    }

    pub fn reset_timer(&mut self) {
        self.addr[0xff0f] &= !(0x1 << 2);
    }

    pub fn reset_serial(&mut self) {
        self.addr[0xff0f] &= !(0x1 << 3);
    }

    pub fn reset_joypad(&mut self) {
        self.addr[0xff0f] &= !(0x1 << 4);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_write_u8() {
        let mut mmu = MMU::new();

        mmu.write_u8(0x1234, 0x12);
        assert_eq!(mmu.read_u8(0x1234), 0x12);
    }

    #[test]
    fn read_write_u16() {
        let mut mmu = MMU::new();

        mmu.write_u16(0x1234, 0x1234);
        assert_eq!(mmu.read_u16(0x1234), 0x1234);
    }

    #[test]
    fn initialises_with_boot_rom() {
        let mmu = MMU::new();

        assert_eq!(mmu.read_u8(0xff10), 0x80);
        assert_eq!(mmu.read_u8(0xff11), 0xbf);
        assert_eq!(mmu.read_u8(0xff48), 0xff);
    }

    #[test]
    fn interrupt_enable_flags() {
        let mut mmu = MMU::new();

        mmu.set_vblank_enable();
        assert_eq!(mmu.read_u8(0xffff), 0x1);
        mmu.set_lcd_enable();
        assert_eq!(mmu.read_u8(0xffff), 0x3);
        mmu.set_timer_enable();
        assert_eq!(mmu.read_u8(0xffff), 0x7);
        mmu.set_serial_enable();
        assert_eq!(mmu.read_u8(0xffff), 0xf);

        // reset
        mmu.reset_vblank_enable();
        assert_eq!(mmu.read_u8(0xffff), 0xe);
        mmu.reset_lcd_enable();
        assert_eq!(mmu.read_u8(0xffff), 0xc);
        mmu.reset_timer_enable();
        assert_eq!(mmu.read_u8(0xffff), 0x8);
        mmu.reset_serial_enable();
        assert_eq!(mmu.read_u8(0xffff), 0x0);
    }

    #[test]
    fn interrupt_flags() {
        let mut mmu = MMU::new();

        mmu.set_vblank();
        assert_eq!(mmu.read_u8(0xff0f), 0x1);
        mmu.set_lcd();
        assert_eq!(mmu.read_u8(0xff0f), 0x3);
        mmu.set_timer();
        assert_eq!(mmu.read_u8(0xff0f), 0x7);
        mmu.set_serial();
        assert_eq!(mmu.read_u8(0xff0f), 0xf);

        // reset
        mmu.reset_vblank();
        assert_eq!(mmu.read_u8(0xff0f), 0xe);
        mmu.reset_lcd();
        assert_eq!(mmu.read_u8(0xff0f), 0xc);
        mmu.reset_timer();
        assert_eq!(mmu.read_u8(0xff0f), 0x8);
        mmu.reset_serial();
        assert_eq!(mmu.read_u8(0xff0f), 0x0);
    }
}
