use super::*;

pub fn alu_inc(registers: &mut Registers, reg: EightBitRegister) {
    let result = registers.read_eight(reg).wrapping_add(1);

    registers.set_zero_flag(result == 0);
    registers.set_subtract_flag(false);
    registers.set_half_carry_flag((value & 0xf) + 1 > 0xf);

    registers.write_eight(reg, result);
}

pub fn alu_dec(registers: &mut Registers, value: u8) -> u8 {
    let result = value.wrapping_sub(1);

    registers.set_zero_flag(result == 0);
    registers.set_subtract_flag(true);
    registers.set_half_carry_flag((value & 0xf) == 0);

    result
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alu_inc_basic() {
        let mut registers = Registers::new();

        let result = alu_inc(&mut registers, 0x01);

        assert_eq!(result, 0x02);
        assert_eq!(registers.get_zero_flag(), false);
        assert_eq!(registers.get_subtract_flag(), false);
        assert_eq!(registers.get_half_carry_flag(), false);
    }

    #[test]
    fn alu_inc_zero_flag() {
        let mut registers = Registers::new();

        let result = alu_inc(&mut registers, 0xff);

        assert_eq!(result, 0x00);
        assert_eq!(registers.get_zero_flag(), true);
        assert_eq!(registers.get_subtract_flag(), false);
        assert_eq!(registers.get_half_carry_flag(), true);
    }

    #[test]
    fn alu_inc_half_carry_flag() {
        let mut registers = Registers::new();

        let result = alu_inc(&mut registers, 0x0f);

        assert_eq!(result, 0x10);
        assert_eq!(registers.get_zero_flag(), false);
        assert_eq!(registers.get_subtract_flag(), false);
        assert_eq!(registers.get_half_carry_flag(), true);
    }

    #[test]
    fn alu_dec_basic() {
        let mut registers = Registers::new();

        let result = alu_dec(&mut registers, 0x01);

        assert_eq!(result, 0x00);
        assert_eq!(registers.get_zero_flag(), true);
        assert_eq!(registers.get_subtract_flag(), true);
        assert_eq!(registers.get_half_carry_flag(), false);
    }

    #[test]
    fn alu_dec_half_carry_flag() {
        let mut registers = Registers::new();

        let result = alu_dec(&mut registers, 0x10);

        assert_eq!(result, 0x0f);
        assert_eq!(registers.get_zero_flag(), false);
        assert_eq!(registers.get_subtract_flag(), true);
        assert_eq!(registers.get_half_carry_flag(), true);
    }

    #[test]
    fn rlca_cycles_bits_and_sets_flags() {
        let mut registers = Registers::new();
        registers.write_eight(EightBitRegister::A, 0b10001000);

        rlca(&mut registers);
        let result = registers.read_eight(EightBitRegister::A);

        assert_eq!(result, 0b00010001);
        assert_eq!(registers.get_zero_flag(), false);
        assert_eq!(registers.get_subtract_flag(), false);
        assert_eq!(registers.get_half_carry_flag(), false);
        assert_eq!(registers.get_carry_flag(), true);
    }
}
