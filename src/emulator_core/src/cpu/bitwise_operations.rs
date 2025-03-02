use super::*;

/// Swap the upper and lower nibbles of a byte value
pub fn swap_value(reg: &mut Registers, value: u8) -> u8 {
    let result = ((value & 0x0F) << 4) | ((value & 0xF0) >> 4);
    reg.set_zero_flag(result == 0);
    reg.set_subtract_flag(false);
    reg.set_half_carry_flag(false);
    reg.set_carry_flag(false);
    result
}

/// Swap the upper and lower nibbles of a register
pub fn swap(reg: &mut Registers, register: EightBitRegister) {
    let value = reg.read_eight(register);
    let result = swap_value(reg, value);
    reg.write_eight(register, result);
}

/// Test bit (bit) of the value, set the zero flag if it is not set
pub fn bit_val(registers: &mut Registers, value: u8, bit: u8) {
    let result = value & (1 << bit);
    registers.set_zero_flag(result == 0);
    registers.set_subtract_flag(false);
    registers.set_half_carry_flag(true);
}

/// Test bit (bit) of the register, set the zero flag if it is not set
pub fn bit(registers: &mut Registers, register: EightBitRegister, bit: u8) {
    let value = registers.read_eight(register);
    bit_val(registers, value, bit);
}

/// Reset bit (bit) of the value
pub fn res_val(value: u8, bit: u8) -> u8 {
    value & !(1 << bit)
}

/// Reset bit (bit) of the register
pub fn res(registers: &mut Registers, register: EightBitRegister, bit: u8) {
    let value = registers.read_eight(register);
    let result = res_val(value, bit);
    registers.write_eight(register, result);
}

/// Set bit (bit) of the value
pub fn set_val(value: u8, bit: u8) -> u8 {
    value | (1 << bit)
}

/// Set bit (bit) of the register
pub fn set(registers: &mut Registers, register: EightBitRegister, bit: u8) {
    let value = registers.read_eight(register);
    let result = set_val(value, bit);
    registers.write_eight(register, result);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_swap_value() {
        let mut reg = Registers::new();
        let value = 0b1010_0110;
        let result = swap_value(&mut reg, value);
        assert_eq!(result, 0b0110_1010);
        assert!(!reg.get_zero_flag());
        assert!(!reg.get_subtract_flag());
        assert!(!reg.get_half_carry_flag());
        assert!(!reg.get_carry_flag());
    }

    #[test]
    fn test_swap_value_zero() {
        let mut reg = Registers::new();
        let value = 0b0000_0000;
        let result = swap_value(&mut reg, value);
        assert_eq!(result, 0b0000_0000);
        assert!(reg.get_zero_flag());
        assert!(!reg.get_subtract_flag());
        assert!(!reg.get_half_carry_flag());
        assert!(!reg.get_carry_flag());
    }

    #[test]
    fn test_swap() {
        let mut reg = Registers::new();
        reg.write_eight(EightBitRegister::B, 0b1110_0110);
        swap(&mut reg, EightBitRegister::B);
        assert_eq!(reg.read_eight(EightBitRegister::B), 0b0110_1110);
        assert!(!reg.get_zero_flag());
        assert!(!reg.get_subtract_flag());
        assert!(!reg.get_half_carry_flag());
        assert!(!reg.get_carry_flag());
    }

    #[test]
    fn test_bit_val_unset() {
        let mut reg = Registers::new();
        let value = 0b1010_1010;
        bit_val(&mut reg, value, 0);
        assert!(reg.get_zero_flag());
        assert!(!reg.get_subtract_flag());
        assert!(reg.get_half_carry_flag());
    }

    #[test]
    fn test_bit_val_set() {
        let mut reg = Registers::new();
        let value = 0b1010_1010;
        bit_val(&mut reg, value, 1);
        assert!(!reg.get_zero_flag());
        assert!(!reg.get_subtract_flag());
        assert!(reg.get_half_carry_flag());
    }

    #[test]
    fn test_bit() {
        let mut reg = Registers::new();
        reg.write_eight(EightBitRegister::B, 0b1010_1010);
        bit(&mut reg, EightBitRegister::B, 0);
        assert!(reg.get_zero_flag());
        assert!(!reg.get_subtract_flag());
        assert!(reg.get_half_carry_flag());
    }

    #[test]
    fn test_res_val_unset_bit() {
        let result = res_val(0b1010_1010, 0);
        assert_eq!(result, 0b1010_1010);
    }

    #[test]
    fn test_res_val_set_bit() {
        let result = res_val(0b1010_1010, 1);
        assert_eq!(result, 0b1010_1000);
    }

    #[test]
    fn test_res() {
        let mut reg = Registers::new();
        reg.write_eight(EightBitRegister::B, 0b1010_1010);
        res(&mut reg, EightBitRegister::B, 1);
        assert_eq!(reg.read_eight(EightBitRegister::B), 0b1010_1000);
    }

    #[test]
    fn test_set_val_unset_bit() {
        let result = set_val(0b1010_1010, 4);
        assert_eq!(result, 0b1011_1010);
    }

    #[test]
    fn test_set_val_set_bit() {
        let result = set_val(0b1010_1010, 1);
        assert_eq!(result, 0b1010_1010 | 0b0000_0010);
    }

    #[test]
    fn test_set() {
        let mut reg = Registers::new();
        reg.write_eight(EightBitRegister::B, 0b1010_1010);
        set(&mut reg, EightBitRegister::B, 1);
        assert_eq!(
            reg.read_eight(EightBitRegister::B),
            0b1010_1010 | 0b0000_0010
        );
    }
}
