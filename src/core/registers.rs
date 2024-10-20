#[derive(Debug)]
pub struct Registers {
    af: u16,
    bc: u16,
    de: u16,
    hl: u16,
    sp: u16,
    pc: u16,
}

#[derive(Clone, Copy, Debug)]
pub enum EightBitRegister {
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Clone, Copy, Debug)]
pub enum SixteenBitRegister {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            af: 0x01b0,
            bc: 0x0013,
            de: 0x00d8,
            hl: 0x014d,
            sp: 0xfffe,
            pc: 0x0100,
        }
    }

    pub fn read_eight(&self, register: EightBitRegister) -> u8 {
        match register {
            EightBitRegister::A => (self.af >> 8) as u8,
            EightBitRegister::F => self.af as u8,
            EightBitRegister::B => (self.bc >> 8) as u8,
            EightBitRegister::C => self.bc as u8,
            EightBitRegister::D => (self.de >> 8) as u8,
            EightBitRegister::E => self.de as u8,
            EightBitRegister::H => (self.hl >> 8) as u8,
            EightBitRegister::L => self.hl as u8,
        }
    }

    pub fn read_sixteen(&self, register: SixteenBitRegister) -> u16 {
        match register {
            SixteenBitRegister::AF => self.af,
            SixteenBitRegister::BC => self.bc,
            SixteenBitRegister::DE => self.de,
            SixteenBitRegister::HL => self.hl,
            SixteenBitRegister::SP => self.sp,
            SixteenBitRegister::PC => self.pc,
        }
    }

    pub fn write_eight(&mut self, register: EightBitRegister, value: u8) {
        let value = value as u16;
        match register {
            EightBitRegister::A => self.af = (self.af & 0xff) | (value << 8),
            EightBitRegister::F => self.af = (self.af & 0xff00) | value,
            EightBitRegister::B => self.bc = (self.bc & 0xff) | (value << 8),
            EightBitRegister::C => self.bc = (self.bc & 0xff00) | value,
            EightBitRegister::D => self.de = (self.de & 0xff) | (value << 8),
            EightBitRegister::E => self.de = (self.de & 0xff00) | value,
            EightBitRegister::H => self.hl = (self.hl & 0xff) | (value << 8),
            EightBitRegister::L => self.hl = (self.hl & 0xff00) | value,
        }
    }

    pub fn write_sixteen(&mut self, register: SixteenBitRegister, value: u16) {
        match register {
            SixteenBitRegister::AF => self.af = value,
            SixteenBitRegister::BC => self.bc = value,
            SixteenBitRegister::DE => self.de = value,
            SixteenBitRegister::HL => self.hl = value,
            SixteenBitRegister::SP => self.sp = value,
            SixteenBitRegister::PC => self.pc = value,
        }
    }

    pub fn update_eight(&mut self, register: EightBitRegister, updator: impl Fn(u8) -> u8) {
        let value = self.read_eight(register);
        self.write_eight(register, updator(value));
    }

    pub fn update_sixteen(&mut self, register: SixteenBitRegister, updator: impl Fn(u16) -> u16) {
        let value = self.read_sixteen(register);
        self.write_sixteen(register, updator(value));
    }

    pub fn set_zero_flag(&mut self, value: bool) {
        if value {
            self.af |= 0x80;
        } else {
            self.af &= !0x80;
        }
    }

    pub fn get_zero_flag(&self) -> bool {
        (self.af & 0x80) != 0
    }

    pub fn set_subtract_flag(&mut self, value: bool) {
        if value {
            self.af |= 0x40;
        } else {
            self.af &= !0x40;
        }
    }

    pub fn get_subtract_flag(&self) -> bool {
        (self.af & 0x40) != 0
    }

    pub fn set_half_carry_flag(&mut self, value: bool) {
        if value {
            self.af |= 0x20;
        } else {
            self.af &= !0x20;
        }
    }

    pub fn get_half_carry_flag(&self) -> bool {
        (self.af & 0x20) != 0
    }

    pub fn set_carry_flag(&mut self, value: bool) {
        if value {
            self.af |= 0x10;
        } else {
            self.af &= !0x10;
        }
    }

    pub fn get_carry_flag(&self) -> bool {
        (self.af & 0x10) != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialises_with_boot_values() {
        let registers = Registers::new();
        assert_eq!(registers.af, 0x01b0);
        assert_eq!(registers.bc, 0x0013);
        assert_eq!(registers.de, 0x00d8);
        assert_eq!(registers.hl, 0x014d);
        assert_eq!(registers.sp, 0xfffe);
        assert_eq!(registers.pc, 0x0100);
    }

    #[test]
    fn eight_bit_read_and_writes() {
        let mut registers = Registers::new();
        registers.write_eight(EightBitRegister::A, 0x12);
        assert_eq!(registers.read_eight(EightBitRegister::A), 0x12);

        registers.write_eight(EightBitRegister::B, 0x34);
        assert_eq!(registers.read_eight(EightBitRegister::B), 0x34);

        registers.write_eight(EightBitRegister::C, 0x56);
        assert_eq!(registers.read_eight(EightBitRegister::C), 0x56);

        registers.write_eight(EightBitRegister::D, 0x78);
        assert_eq!(registers.read_eight(EightBitRegister::D), 0x78);

        registers.write_eight(EightBitRegister::E, 0x9a);
        assert_eq!(registers.read_eight(EightBitRegister::E), 0x9a);

        registers.write_eight(EightBitRegister::H, 0xbc);
        assert_eq!(registers.read_eight(EightBitRegister::H), 0xbc);

        registers.write_eight(EightBitRegister::L, 0xde);
        assert_eq!(registers.read_eight(EightBitRegister::L), 0xde);
    }

    #[test]
    fn sixteen_but_read_and_writes() {
        let mut registers = Registers::new();
        registers.write_sixteen(SixteenBitRegister::AF, 0x1234);
        assert_eq!(registers.read_sixteen(SixteenBitRegister::AF), 0x1234);

        registers.write_sixteen(SixteenBitRegister::BC, 0x5678);
        assert_eq!(registers.read_sixteen(SixteenBitRegister::BC), 0x5678);

        registers.write_sixteen(SixteenBitRegister::DE, 0x9abc);
        assert_eq!(registers.read_sixteen(SixteenBitRegister::DE), 0x9abc);

        registers.write_sixteen(SixteenBitRegister::HL, 0xdef0);
        assert_eq!(registers.read_sixteen(SixteenBitRegister::HL), 0xdef0);

        registers.write_sixteen(SixteenBitRegister::SP, 0x1234);
        assert_eq!(registers.read_sixteen(SixteenBitRegister::SP), 0x1234);

        registers.write_sixteen(SixteenBitRegister::PC, 0x5678);
        assert_eq!(registers.read_sixteen(SixteenBitRegister::PC), 0x5678);
    }

    #[test]
    fn zero_flag() {
        let mut registers = Registers::new();
        registers.set_zero_flag(false);
        assert_eq!(registers.get_zero_flag(), false);
        assert_eq!(registers.af, 0x01b0 & !0x80);

        registers.set_zero_flag(true);
        assert_eq!(registers.get_zero_flag(), true);
        assert_eq!(registers.af, 0x01b0 | 0x80);
    }

    #[test]
    fn subtract_flag() {
        let mut registers = Registers::new();
        registers.set_subtract_flag(true);
        assert_eq!(registers.get_subtract_flag(), true);
        assert_eq!(registers.af, 0x01b0 | 0x40);

        registers.set_subtract_flag(false);
        assert_eq!(registers.get_subtract_flag(), false);
        assert_eq!(registers.af, 0x01b0 & !0x40);
    }

    #[test]
    fn half_carry_flag() {
        let mut registers = Registers::new();
        registers.set_half_carry_flag(true);
        assert_eq!(registers.get_half_carry_flag(), true);
        assert_eq!(registers.af, 0x01b0 | 0x20);

        registers.set_half_carry_flag(false);
        assert_eq!(registers.get_half_carry_flag(), false);
        assert_eq!(registers.af, 0x01b0 & !0x20)
    }

    #[test]
    fn carry_flag() {
        let mut registers = Registers::new();
        registers.set_carry_flag(true);
        assert_eq!(registers.get_carry_flag(), true);
        assert_eq!(registers.af, 0x01b0 | 0x10);

        registers.set_carry_flag(false);
        assert_eq!(registers.get_carry_flag(), false);
        assert_eq!(registers.af, 0x01b0 & !0x10);
    }
}
