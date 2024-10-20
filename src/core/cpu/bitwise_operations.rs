use super::*;

pub fn srl_val(registers: &mut Registers, value: u8) -> u8 {
    let carry = value & 0b0000_0001 == 0b0000_0001;

    let result = value >> 1;
    registers.set_carry_flag(carry);
    registers.set_half_carry_flag(false);
    registers.set_subtract_flag(false);
    registers.set_zero_flag(result == 0);

    result
}

pub fn srl(registers: &mut Registers, reg: EightBitRegister) {
    let value = registers.read_eight(reg);
    let result = srl_val(registers, value);
    registers.write_eight(reg, result);
}

pub fn rr_val(registers: &mut Registers, value: u8) -> u8 {
    let carry = value & 0b0000_0001 == 0b0000_0001;

    let mut result = value >> 1;
    if registers.get_carry_flag() {
        result |= 0b1000_0000;
    }

    registers.set_carry_flag(carry);
    registers.set_half_carry_flag(false);
    registers.set_subtract_flag(false);
    registers.set_zero_flag(result == 0);

    result
}

pub fn rr(registers: &mut Registers, reg: EightBitRegister) {
    let value = registers.read_eight(reg);
    let result = rr_val(registers, value);
    registers.write_eight(reg, result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_srl_val_basic() {
        let mut registers = Registers::new();
        let value = 0b1010_1010;
        let result = srl_val(&mut registers, value);
        assert_eq!(result, 0b0101_0101);
        assert_eq!(registers.get_carry_flag(), false);
        assert_eq!(registers.get_half_carry_flag(), false);
        assert_eq!(registers.get_subtract_flag(), false);
        assert_eq!(registers.get_zero_flag(), false);
    }

    #[test]
    fn test_srl_val_zero() {
        let mut registers = Registers::new();
        let value = 0b0000_0001;
        let result = srl_val(&mut registers, value);
        assert_eq!(result, 0b0000_0000);
        assert_eq!(registers.get_carry_flag(), true);
        assert_eq!(registers.get_half_carry_flag(), false);
        assert_eq!(registers.get_subtract_flag(), false);
        assert_eq!(registers.get_zero_flag(), true);
    }

    #[test]
    fn test_srl_val_carry() {
        let mut registers = Registers::new();
        let value = 0b0000_0001;
        let result = srl_val(&mut registers, value);
        assert_eq!(result, 0b0000_0000);
        assert_eq!(registers.get_carry_flag(), true);
        assert_eq!(registers.get_half_carry_flag(), false);
        assert_eq!(registers.get_subtract_flag(), false);
        assert_eq!(registers.get_zero_flag(), true);
    }

    #[test]
    fn test_srl_basic() {
        let mut registers = Registers::new();
        registers.write_eight(EightBitRegister::B, 0b1010_1010);
        srl(&mut registers, EightBitRegister::B);
        assert_eq!(registers.read_eight(EightBitRegister::B), 0b0101_0101);
        assert_eq!(registers.get_carry_flag(), false);
        assert_eq!(registers.get_half_carry_flag(), false);
        assert_eq!(registers.get_subtract_flag(), false);
        assert_eq!(registers.get_zero_flag(), false);
    }

    #[test]
    fn test_rr_val_basic() {
        let mut registers = Registers::new();
        let value = 0b1010_1010;
        let result = rr_val(&mut registers, value);
        assert_eq!(result, 0b1101_0101);
        assert_eq!(registers.get_carry_flag(), false);
        assert_eq!(registers.get_half_carry_flag(), false);
        assert_eq!(registers.get_subtract_flag(), false);
        assert_eq!(registers.get_zero_flag(), false);
    }

    #[test]
    fn test_rr_val_carry() {
        let mut registers = Registers::new();
        let value = 0b1010_1010;
        registers.set_carry_flag(true);
        let result = rr_val(&mut registers, value);
        assert_eq!(result, 0b1101_0101);
        assert_eq!(registers.get_carry_flag(), false);
        assert_eq!(registers.get_half_carry_flag(), false);
        assert_eq!(registers.get_subtract_flag(), false);
        assert_eq!(registers.get_zero_flag(), false);
    }

    #[test]
    fn test_rr_val_zero() {
        let mut registers = Registers::new();
        let value = 0b0000_0001;
        registers.set_carry_flag(false);
        let result = rr_val(&mut registers, value);
        assert_eq!(result, 0b0000_0000);
        assert_eq!(registers.get_carry_flag(), true);
        assert_eq!(registers.get_half_carry_flag(), false);
        assert_eq!(registers.get_subtract_flag(), false);
        assert_eq!(registers.get_zero_flag(), true);
    }

    #[test]
    fn test_rr_basic() {
        let mut registers = Registers::new();
        registers.write_eight(EightBitRegister::B, 0b1010_1010);
        rr(&mut registers, EightBitRegister::B);
        assert_eq!(registers.read_eight(EightBitRegister::B), 0b1101_0101);
        assert_eq!(registers.get_carry_flag(), false);
        assert_eq!(registers.get_half_carry_flag(), false);
        assert_eq!(registers.get_subtract_flag(), false);
        assert_eq!(registers.get_zero_flag(), false);
    }
}
