use super::*;

pub fn alu_inc_value(registers: &mut Registers, value: u8) -> u8 {
    let result = value.wrapping_add(1);

    registers.set_zero_flag(result == 0);
    registers.set_subtract_flag(false);
    registers.set_half_carry_flag((value & 0xf) == 0xf);

    result
}

pub fn alu_inc(registers: &mut Registers, reg: EightBitRegister) {
    let value = registers.read_eight(reg);
    let result = alu_inc_value(registers, value);

    registers.write_eight(reg, result);
}

pub fn alu_dec_value(registers: &mut Registers, value: u8) -> u8 {
    let result = value.wrapping_sub(1);

    registers.set_zero_flag(result == 0);
    registers.set_subtract_flag(true);
    registers.set_half_carry_flag((value & 0xf) == 0x00);

    result
}

pub fn alu_dec(registers: &mut Registers, reg: EightBitRegister) {
    let value = registers.read_eight(reg);
    let result = alu_dec_value(registers, value);

    registers.write_eight(reg, result);
}

pub fn alu_add_value(registers: &mut Registers, value: u8) {
    let a = registers.read_eight(EightBitRegister::A);
    let result = a.wrapping_add(value);

    registers.set_zero_flag(result == 0);
    registers.set_subtract_flag(false);
    registers.set_half_carry_flag((a & 0xf) + (value & 0xf) > 0xf);
    registers.set_carry_flag((a as u16) + (value as u16) > 0xff);

    registers.write_eight(EightBitRegister::A, result);
}

pub fn alu_add(registers: &mut Registers, reg: EightBitRegister) {
    let value = registers.read_eight(reg);
    alu_add_value(registers, value);
}

/// Add the value to the A register, with the carry flag.
pub fn alu_adc_value(registers: &mut Registers, value: u8) {
    let a = registers.read_eight(EightBitRegister::A);
    let carry = registers.get_carry_flag() as u8;
    let result = a.wrapping_add(value).wrapping_add(carry);

    registers.set_zero_flag(result == 0);
    registers.set_subtract_flag(false);
    registers.set_half_carry_flag((a & 0xf) + (value & 0xf) + carry > 0xf);
    registers.set_carry_flag((a as u16) + (value as u16) + (carry as u16) > 0xff);

    registers.write_eight(EightBitRegister::A, result);
}

/// Add the value of a register to the A register, with the carry flag.
pub fn alu_adc(registers: &mut Registers, reg: EightBitRegister) {
    let value = registers.read_eight(reg);
    alu_adc_value(registers, value);
}

pub fn alu_sub_value(registers: &mut Registers, value: u8) {
    let a = registers.read_eight(EightBitRegister::A);
    let result = a.wrapping_sub(value);

    registers.set_zero_flag(result == 0);
    registers.set_subtract_flag(true);
    registers.set_half_carry_flag((a & 0xf) < (value & 0xf));
    registers.set_carry_flag(a < value);

    registers.write_eight(EightBitRegister::A, result);
}

pub fn alu_sub(registers: &mut Registers, reg: EightBitRegister) {
    let value = registers.read_eight(reg);
    alu_sub_value(registers, value);
}

/// Subtract the value from the A register, with the carry flag.
pub fn alu_sbc_value(registers: &mut Registers, value: u8) {
    let a = registers.read_eight(EightBitRegister::A);
    let carry = registers.get_carry_flag() as u8;
    let result = a.wrapping_sub(value).wrapping_sub(carry);

    registers.set_zero_flag(result == 0);
    registers.set_subtract_flag(true);
    registers.set_half_carry_flag((a & 0xf) < (value & 0xf) + carry);
    registers.set_carry_flag((a as u16) < (value as u16) + (carry as u16));

    registers.write_eight(EightBitRegister::A, result);
}

/// Subtract the value of a register from the A register, with the carry flag.
pub fn alu_sbc(registers: &mut Registers, reg: EightBitRegister) {
    let value = registers.read_eight(reg);
    alu_sbc_value(registers, value);
}

/// Logical AND the value with the A register.
pub fn alu_and_value(registers: &mut Registers, value: u8) {
    let a = registers.read_eight(EightBitRegister::A);
    let result = a & value;

    registers.set_zero_flag(result == 0);
    registers.set_subtract_flag(false);
    registers.set_half_carry_flag(true);
    registers.set_carry_flag(false);

    registers.write_eight(EightBitRegister::A, result);
}

// TODO: test
pub fn ld_hl_sp_r8(registers: &mut Registers, value: u8) {
    let sp = registers.read_sixteen(SixteenBitRegister::SP);
    let result = sp.wrapping_add(value as i8 as i16 as u16);

    registers.write_sixteen(SixteenBitRegister::HL, result);
    registers.set_zero_flag(false);
    registers.set_subtract_flag(false);
    registers.set_half_carry_flag((sp & 0xf) + (value as u16 & 0xf) > 0xf);
    registers.set_carry_flag((sp & 0xff) + (value as u16 & 0xff) > 0xff);
}

// TODO: test
pub fn alu_add_16(registers: &mut Registers, reg: SixteenBitRegister) {
    let value = registers.read_sixteen(reg);
    let hl = registers.read_sixteen(SixteenBitRegister::HL);
    let result = hl.wrapping_add(value);

    registers.write_sixteen(SixteenBitRegister::HL, result);
    registers.set_subtract_flag(false);
    registers.set_half_carry_flag((hl & 0xfff) + (value & 0xfff) > 0xfff);
    registers.set_carry_flag(hl > 0xffff - value);
}

// TODO: test
pub fn add_sp_r8(registers: &mut Registers, value: u8) {
    let sp = registers.read_sixteen(SixteenBitRegister::SP);
    let result = sp.wrapping_add(value as i8 as i16 as u16);

    registers.write_sixteen(SixteenBitRegister::SP, result);
    registers.set_zero_flag(false);
    registers.set_subtract_flag(false);
    registers.set_half_carry_flag((sp & 0xf) + (value as u16 & 0xf) > 0xf);
    registers.set_carry_flag((sp & 0xff) + (value as u16 & 0xff) > 0xff);
}

/// Logical AND the value of a register with the A register.
pub fn alu_and(registers: &mut Registers, reg: EightBitRegister) {
    let value = registers.read_eight(reg);
    alu_and_value(registers, value);
}

/// Logical OR the value with the A register.
pub fn alu_or_value(registers: &mut Registers, value: u8) {
    let a = registers.read_eight(EightBitRegister::A);
    let result = a | value;

    registers.set_zero_flag(result == 0);
    registers.set_subtract_flag(false);
    registers.set_half_carry_flag(false);
    registers.set_carry_flag(false);

    registers.write_eight(EightBitRegister::A, result);
}

/// Logical OR the value of a register with the A register.
pub fn alu_or(registers: &mut Registers, reg: EightBitRegister) {
    let value = registers.read_eight(reg);
    alu_or_value(registers, value);
}

/// Logical XOR the value with the A register.
pub fn alu_xor_value(registers: &mut Registers, value: u8) {
    let a = registers.read_eight(EightBitRegister::A);
    let result = a ^ value;

    registers.set_zero_flag(result == 0);
    registers.set_subtract_flag(false);
    registers.set_half_carry_flag(false);
    registers.set_carry_flag(false);

    registers.write_eight(EightBitRegister::A, result);
}

/// Logical XOR the value of a register with the A register.
pub fn alu_xor(registers: &mut Registers, reg: EightBitRegister) {
    let value = registers.read_eight(reg);
    alu_xor_value(registers, value);
}

/// Compare the value with the A register.
pub fn alu_cp_value(registers: &mut Registers, value: u8) {
    let a = registers.read_eight(EightBitRegister::A);
    let result = a.wrapping_sub(value);

    registers.set_zero_flag(result == 0);
    registers.set_subtract_flag(true);
    registers.set_half_carry_flag((a & 0xf) < (value & 0xf));
    registers.set_carry_flag(a < value);
}

/// Compare the value of a register with the A register.
pub fn alu_cp(registers: &mut Registers, reg: EightBitRegister) {
    let value = registers.read_eight(reg);
    alu_cp_value(registers, value);
}

// Rotate A left. Old bit 7 also coppied to carry flag.
pub fn rlca(registers: &mut Registers) {
    let value = registers.read_eight(EightBitRegister::A);
    let carry = value & 0x80 != 0;
    let result = (value << 1) | (value >> 7);

    registers.write_eight(EightBitRegister::A, result);
    registers.set_zero_flag(false);
    registers.set_subtract_flag(false);
    registers.set_half_carry_flag(false);
    registers.set_carry_flag(carry);
}

// Rotate A right. Old bit 0 also coppied to carry flag.
pub fn rrca(registers: &mut Registers) {
    let value = registers.read_eight(EightBitRegister::A);
    let carry = value & 0x01 != 0;
    let result = (value >> 1) | (value << 7);

    registers.write_eight(EightBitRegister::A, result);
    registers.set_zero_flag(false);
    registers.set_subtract_flag(false);
    registers.set_half_carry_flag(false);
    registers.set_carry_flag(carry);
}

// Rotate A right through carry flag.
pub fn rra(registers: &mut Registers) {
    let value = registers.read_eight(EightBitRegister::A);
    let carry = registers.get_carry_flag();
    let result = (value >> 1) | ((carry as u8) << 7);

    registers.write_eight(EightBitRegister::A, result);
    registers.set_zero_flag(false);
    registers.set_subtract_flag(false);
    registers.set_half_carry_flag(false);
    registers.set_carry_flag(value & 0x01 != 0);
}

// Rotate A left through carry flag.
pub fn rla(registers: &mut Registers) {
    let value = registers.read_eight(EightBitRegister::A);
    let carry = registers.get_carry_flag();
    let result = (value << 1) | carry as u8;

    registers.write_eight(EightBitRegister::A, result);
    registers.set_zero_flag(false);
    registers.set_subtract_flag(false);
    registers.set_half_carry_flag(false);
    registers.set_carry_flag(value & 0x80 != 0);
}

// Jump to relative address, taking the value as a signed byte.
pub fn jr(registers: &mut Registers, value: u8) {
    let signed_value = value as i8;
    let unsigned_value = signed_value.unsigned_abs() as u16;

    let pc = registers.read_sixteen(SixteenBitRegister::PC);

    if signed_value.is_negative() {
        registers.write_sixteen(SixteenBitRegister::PC, pc.wrapping_sub(unsigned_value));
    } else {
        registers.write_sixteen(SixteenBitRegister::PC, pc.wrapping_add(unsigned_value));
    }
}

// TODO: test and what the hell is this?
pub fn daa(registers: &mut Registers) {
    let mut a = registers.read_eight(EightBitRegister::A);
    let mut adjust = if registers.get_carry_flag() {
        0x60
    } else {
        0x00
    };
    if registers.get_half_carry_flag() || (!registers.get_subtract_flag() && (a & 0x0f) > 0x09) {
        adjust |= 0x06;
    }
    if registers.get_carry_flag() || (!registers.get_subtract_flag() && a > 0x99) {
        adjust |= 0x60;
    }

    if registers.get_subtract_flag() {
        a = a.wrapping_sub(adjust);
    } else {
        a = a.wrapping_add(adjust);
    }

    registers.set_zero_flag(a == 0);
    registers.set_half_carry_flag(false);
    registers.set_carry_flag(adjust >= 0x60);
    registers.write_eight(EightBitRegister::A, a);
}

// Flip all bits in A.
pub fn cpl(registers: &mut Registers) {
    registers.update_eight(EightBitRegister::A, |value| !value);
    registers.set_subtract_flag(true);
    registers.set_half_carry_flag(true);
}

// Set carry flag.
pub fn scf(registers: &mut Registers) {
    registers.set_carry_flag(true);
    registers.set_subtract_flag(false);
    registers.set_half_carry_flag(false);
}

// Complement carry flag.
pub fn ccf(registers: &mut Registers) {
    let carry = registers.get_carry_flag();
    registers.set_carry_flag(!carry);
    registers.set_subtract_flag(false);
    registers.set_half_carry_flag(false);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alu_inc_value_basic() {
        let mut registers = Registers::new();

        let value = alu_inc_value(&mut registers, 0x01);

        assert_eq!(value, 0x02);
        assert!(!registers.get_zero_flag());
        assert!(!registers.get_subtract_flag());
        assert!(!registers.get_half_carry_flag());
    }

    #[test]
    fn alu_inc_value_zero_flag_and_carry() {
        let mut registers = Registers::new();
        let value = alu_inc_value(&mut registers, 0xff);

        assert_eq!(value, 0x00);
        assert!(registers.get_zero_flag());
        assert!(!registers.get_subtract_flag());
        assert!(registers.get_half_carry_flag());
    }

    #[test]
    fn alu_inc_half_carry_flag() {
        let mut registers = Registers::new();
        let value = alu_inc_value(&mut registers, 0x0f);

        assert_eq!(value, 0x10);
        assert!(!registers.get_zero_flag());
        assert!(!registers.get_subtract_flag());
        assert!(registers.get_half_carry_flag());
    }

    #[test]
    fn alu_inc_basic() {
        let mut registers = Registers::new();
        registers.write_eight(EightBitRegister::A, 0x01);

        alu_inc(&mut registers, EightBitRegister::A);

        assert_eq!(registers.read_eight(EightBitRegister::A), 0x02);
        assert!(!registers.get_zero_flag());
        assert!(!registers.get_subtract_flag());
        assert!(!registers.get_half_carry_flag());
    }

    #[test]
    fn alu_dec_value_basic() {
        let mut registers = Registers::new();
        let value = alu_dec_value(&mut registers, 0x01);

        assert_eq!(value, 0x00);
        assert!(registers.get_zero_flag());
        assert!(registers.get_subtract_flag());
        assert!(!registers.get_half_carry_flag());
    }

    #[test]
    fn alu_dec_value_half_carry_flag() {
        let mut registers = Registers::new();
        let value = alu_dec_value(&mut registers, 0x10);

        assert_eq!(value, 0x0f);
        assert!(!registers.get_zero_flag());
        assert!(registers.get_subtract_flag());
        assert!(registers.get_half_carry_flag());
    }

    #[test]
    fn alu_dec_basic() {
        let mut registers = Registers::new();
        registers.write_eight(EightBitRegister::A, 0x01);

        alu_dec(&mut registers, EightBitRegister::A);

        assert_eq!(registers.read_eight(EightBitRegister::A), 0x00);
        assert!(registers.get_zero_flag());
        assert!(registers.get_subtract_flag());
        assert!(!registers.get_half_carry_flag());
    }

    #[test]
    fn rlca_cycles_bits_and_sets_flags() {
        let mut registers = Registers::new();
        set_flags(&mut registers);
        registers.write_eight(EightBitRegister::A, 0b10001000);

        rlca(&mut registers);
        registers.read_eight(EightBitRegister::A);

        assert_eq!(registers.read_eight(EightBitRegister::A), 0b00010001);
        assert!(!registers.get_zero_flag());
        assert!(!registers.get_subtract_flag());
        assert!(!registers.get_half_carry_flag());
        assert!(registers.get_carry_flag());
    }

    #[test]
    fn rrca_cycles_bits_and_sets_flags() {
        let mut registers = Registers::new();
        set_flags(&mut registers);
        registers.write_eight(EightBitRegister::A, 0b00010001);

        rrca(&mut registers);
        registers.read_eight(EightBitRegister::A);

        assert_eq!(registers.read_eight(EightBitRegister::A), 0b10001000);
        assert!(!registers.get_zero_flag());
        assert!(!registers.get_subtract_flag());
        assert!(!registers.get_half_carry_flag());
        assert!(registers.get_carry_flag());
    }

    #[test]
    fn rra_cycles_bits_and_sets_flags() {
        let mut registers = Registers::new();
        set_flags(&mut registers);
        registers.write_eight(EightBitRegister::A, 0b10001000);
        registers.set_carry_flag(true);

        rra(&mut registers);
        registers.read_eight(EightBitRegister::A);

        assert_eq!(registers.read_eight(EightBitRegister::A), 0b11000100);
        assert!(!registers.get_zero_flag());
        assert!(!registers.get_subtract_flag());
        assert!(!registers.get_half_carry_flag());
        assert!(!registers.get_carry_flag());
    }

    #[test]
    fn cpl_flips_all_bits_and_sets_flags() {
        let mut registers = Registers::new();
        reset_flags(&mut registers);
        registers.write_eight(EightBitRegister::A, 0b10101010);

        cpl(&mut registers);

        assert_eq!(registers.read_eight(EightBitRegister::A), 0b01010101);
        assert!(!registers.get_zero_flag());
        assert!(registers.get_subtract_flag());
        assert!(registers.get_half_carry_flag());
        assert!(!registers.get_carry_flag());
    }

    #[test]
    fn scf_sets_carry_flag() {
        let mut registers = Registers::new();
        reset_flags(&mut registers);

        scf(&mut registers);

        assert!(!registers.get_zero_flag());
        assert!(!registers.get_subtract_flag());
        assert!(!registers.get_half_carry_flag());
        assert!(registers.get_carry_flag());
    }

    #[test]
    fn alu_add_value_basic() {
        let mut registers = Registers::new();
        registers.write_eight(EightBitRegister::A, 0x01);

        alu_add_value(&mut registers, 0x01);

        assert_eq!(registers.read_eight(EightBitRegister::A), 0x02);
        assert!(!registers.get_zero_flag());
        assert!(!registers.get_subtract_flag());
        assert!(!registers.get_half_carry_flag());
        assert!(!registers.get_carry_flag());
    }

    #[test]
    fn alu_add_value_half_carry() {
        let mut registers = Registers::new();
        registers.write_eight(EightBitRegister::A, 0x0f);

        alu_add_value(&mut registers, 0x01);

        assert_eq!(registers.read_eight(EightBitRegister::A), 0x10);
        assert!(!registers.get_zero_flag());
        assert!(!registers.get_subtract_flag());
        assert!(registers.get_half_carry_flag());
        assert!(!registers.get_carry_flag());
    }

    #[test]
    fn alu_add_value_carry() {
        let mut registers = Registers::new();
        registers.write_eight(EightBitRegister::A, 0xff);

        alu_add_value(&mut registers, 0x01);

        assert_eq!(registers.read_eight(EightBitRegister::A), 0x00);
        assert!(registers.get_zero_flag());
        assert!(!registers.get_subtract_flag());
        assert!(registers.get_half_carry_flag());
        assert!(registers.get_carry_flag());
    }

    #[test]
    fn alu_add_basic() {
        let mut registers = Registers::new();
        registers.write_eight(EightBitRegister::A, 0x01);
        registers.write_eight(EightBitRegister::B, 0x01);

        alu_add(&mut registers, EightBitRegister::B);

        assert_eq!(registers.read_eight(EightBitRegister::A), 0x02);
        assert!(!registers.get_zero_flag());
        assert!(!registers.get_subtract_flag());
        assert!(!registers.get_half_carry_flag());
        assert!(!registers.get_carry_flag());
    }

    #[test]
    fn alu_adc_value_basic() {
        let mut registers = Registers::new();
        registers.write_eight(EightBitRegister::A, 0x01);
        registers.set_carry_flag(true);

        alu_adc_value(&mut registers, 0x01);

        assert_eq!(registers.read_eight(EightBitRegister::A), 0x03);
        assert!(!registers.get_zero_flag());
        assert!(!registers.get_subtract_flag());
        assert!(!registers.get_half_carry_flag());
        assert!(!registers.get_carry_flag());
    }

    #[test]
    fn alu_adc_value_half_carry() {
        let mut registers = Registers::new();
        registers.write_eight(EightBitRegister::A, 0x0f);
        registers.set_carry_flag(true);

        alu_adc_value(&mut registers, 0x01);

        assert_eq!(registers.read_eight(EightBitRegister::A), 0x11);
        assert!(!registers.get_zero_flag());
        assert!(!registers.get_subtract_flag());
        assert!(registers.get_half_carry_flag());
        assert!(!registers.get_carry_flag());
    }

    #[test]
    fn alu_adc_value_carry() {
        let mut registers = Registers::new();
        registers.write_eight(EightBitRegister::A, 0xff);
        registers.set_carry_flag(true);

        alu_adc_value(&mut registers, 0x01);

        assert_eq!(registers.read_eight(EightBitRegister::A), 0x01);
        assert!(!registers.get_zero_flag());
        assert!(!registers.get_subtract_flag());
        assert!(registers.get_half_carry_flag());
        assert!(registers.get_carry_flag());
    }

    #[test]
    fn alu_adc_basic() {
        let mut registers = Registers::new();
        registers.write_eight(EightBitRegister::A, 0x01);
        registers.write_eight(EightBitRegister::B, 0x01);
        registers.set_carry_flag(true);

        alu_adc(&mut registers, EightBitRegister::B);

        assert_eq!(registers.read_eight(EightBitRegister::A), 0x03);
        assert!(!registers.get_zero_flag());
        assert!(!registers.get_subtract_flag());
        assert!(!registers.get_half_carry_flag());
        assert!(!registers.get_carry_flag());
    }

    #[test]
    fn alu_sub_value_basic() {
        let mut registers = Registers::new();
        registers.write_eight(EightBitRegister::A, 0x02);

        alu_sub_value(&mut registers, 0x01);

        assert_eq!(registers.read_eight(EightBitRegister::A), 0x01);
        assert!(!registers.get_zero_flag());
        assert!(registers.get_subtract_flag());
        assert!(!registers.get_half_carry_flag());
        assert!(!registers.get_carry_flag());
    }

    #[test]
    fn alu_sub_value_carry_and_half_carry() {
        let mut registers = Registers::new();
        registers.write_eight(EightBitRegister::A, 0x00);

        alu_sub_value(&mut registers, 0x01);

        assert_eq!(registers.read_eight(EightBitRegister::A), 0xff);
        assert!(!registers.get_zero_flag());
        assert!(registers.get_subtract_flag());
        assert!(registers.get_half_carry_flag());
        assert!(registers.get_carry_flag());
    }

    #[test]
    fn alu_sub_basic() {
        let mut registers = Registers::new();
        registers.write_eight(EightBitRegister::A, 0x02);
        registers.write_eight(EightBitRegister::B, 0x01);

        alu_sub(&mut registers, EightBitRegister::B);

        assert_eq!(registers.read_eight(EightBitRegister::A), 0x01);
        assert!(!registers.get_zero_flag());
        assert!(registers.get_subtract_flag());
        assert!(!registers.get_half_carry_flag());
        assert!(!registers.get_carry_flag());
    }

    #[test]
    fn alu_sbc_value_basic() {
        let mut registers = Registers::new();
        registers.write_eight(EightBitRegister::A, 0x02);
        registers.set_carry_flag(true);

        alu_sbc_value(&mut registers, 0x01);

        assert_eq!(registers.read_eight(EightBitRegister::A), 0x00);
        assert!(registers.get_zero_flag());
        assert!(registers.get_subtract_flag());
        assert!(!registers.get_half_carry_flag());
        assert!(!registers.get_carry_flag());
    }

    #[test]
    fn alu_sbc_value_carry_and_half_carry() {
        let mut registers = Registers::new();
        registers.write_eight(EightBitRegister::A, 0x00);
        registers.set_carry_flag(true);

        alu_sbc_value(&mut registers, 0x01);

        assert_eq!(registers.read_eight(EightBitRegister::A), 0xfe);
        assert!(!registers.get_zero_flag());
        assert!(registers.get_subtract_flag());
        assert!(registers.get_half_carry_flag());
        assert!(registers.get_carry_flag());
    }

    #[test]
    fn alu_sbc_basic() {
        let mut registers = Registers::new();
        registers.write_eight(EightBitRegister::A, 0x02);
        registers.write_eight(EightBitRegister::B, 0x01);
        registers.set_carry_flag(true);

        alu_sbc(&mut registers, EightBitRegister::B);

        assert_eq!(registers.read_eight(EightBitRegister::A), 0x00);
        assert!(registers.get_zero_flag());
        assert!(registers.get_subtract_flag());
        assert!(!registers.get_half_carry_flag());
        assert!(!registers.get_carry_flag());
    }

    #[test]
    fn alu_and_value_basic() {
        let mut registers = Registers::new();
        registers.write_eight(EightBitRegister::A, 0b10101010);

        alu_and_value(&mut registers, 0b11001100);

        assert_eq!(registers.read_eight(EightBitRegister::A), 0b10001000);
        assert!(!registers.get_zero_flag());
        assert!(!registers.get_subtract_flag());
        assert!(registers.get_half_carry_flag());
        assert!(!registers.get_carry_flag());
    }

    #[test]
    fn alu_and_basic() {
        let mut registers = Registers::new();
        registers.write_eight(EightBitRegister::A, 0b10101010);
        registers.write_eight(EightBitRegister::B, 0b11001100);

        alu_and(&mut registers, EightBitRegister::B);

        assert_eq!(registers.read_eight(EightBitRegister::A), 0b10001000);
        assert!(!registers.get_zero_flag());
        assert!(!registers.get_subtract_flag());
        assert!(registers.get_half_carry_flag());
        assert!(!registers.get_carry_flag());
    }

    #[test]
    fn alu_or_value_basic() {
        let mut registers = Registers::new();
        registers.write_eight(EightBitRegister::A, 0b10101010);

        alu_or_value(&mut registers, 0b11001100);

        assert_eq!(registers.read_eight(EightBitRegister::A), 0b11101110);
        assert!(!registers.get_zero_flag());
        assert!(!registers.get_subtract_flag());
        assert!(!registers.get_half_carry_flag());
        assert!(!registers.get_carry_flag());
    }

    #[test]
    fn alu_or_basic() {
        let mut registers = Registers::new();
        registers.write_eight(EightBitRegister::A, 0b10101010);
        registers.write_eight(EightBitRegister::B, 0b11001100);

        alu_or(&mut registers, EightBitRegister::B);

        assert_eq!(registers.read_eight(EightBitRegister::A), 0b11101110);
        assert!(!registers.get_zero_flag());
        assert!(!registers.get_subtract_flag());
        assert!(!registers.get_half_carry_flag());
        assert!(!registers.get_carry_flag());
    }

    #[test]
    fn alu_xor_value_basic() {
        let mut registers = Registers::new();
        registers.write_eight(EightBitRegister::A, 0b10101010);

        alu_xor_value(&mut registers, 0b11001100);

        assert_eq!(registers.read_eight(EightBitRegister::A), 0b01100110);
        assert!(!registers.get_zero_flag());
        assert!(!registers.get_subtract_flag());
        assert!(!registers.get_half_carry_flag());
        assert!(!registers.get_carry_flag());
    }

    #[test]
    fn alu_xor_basic() {
        let mut registers = Registers::new();
        registers.write_eight(EightBitRegister::A, 0b10101010);
        registers.write_eight(EightBitRegister::B, 0b11001100);

        alu_xor(&mut registers, EightBitRegister::B);

        assert_eq!(registers.read_eight(EightBitRegister::A), 0b01100110);
        assert!(!registers.get_zero_flag());
        assert!(!registers.get_subtract_flag());
        assert!(!registers.get_half_carry_flag());
        assert!(!registers.get_carry_flag());
    }

    #[test]
    fn alu_cp_value_basic() {
        let mut registers = Registers::new();
        registers.write_eight(EightBitRegister::A, 0x02);

        alu_cp_value(&mut registers, 0x01);

        assert!(!registers.get_zero_flag());
        assert!(registers.get_subtract_flag());
        assert!(!registers.get_half_carry_flag());
        assert!(!registers.get_carry_flag());
    }

    #[test]
    fn alu_cp_basic() {
        let mut registers = Registers::new();
        registers.write_eight(EightBitRegister::A, 0x02);
        registers.write_eight(EightBitRegister::B, 0x01);

        alu_cp(&mut registers, EightBitRegister::B);

        assert!(!registers.get_zero_flag());
        assert!(registers.get_subtract_flag());
        assert!(!registers.get_half_carry_flag());
        assert!(!registers.get_carry_flag());
    }

    fn reset_flags(registers: &mut Registers) {
        registers.set_zero_flag(false);
        registers.set_subtract_flag(false);
        registers.set_half_carry_flag(false);
        registers.set_carry_flag(false);
    }

    fn set_flags(registers: &mut Registers) {
        registers.set_zero_flag(true);
        registers.set_subtract_flag(true);
        registers.set_half_carry_flag(true);
        registers.set_carry_flag(true);
    }
}
