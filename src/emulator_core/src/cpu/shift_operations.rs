use super::*;

/// Logically shift a value right, setting the carry flag to the least significant bit
pub fn srl_val(registers: &mut Registers, value: u8) -> u8 {
    let carry = value & 0b0000_0001 == 0b0000_0001;

    let result = value >> 1;
    registers.set_carry_flag(carry);
    registers.set_half_carry_flag(false);
    registers.set_subtract_flag(false);
    registers.set_zero_flag(result == 0);

    result
}

/// Logically shift a register right, setting the carry flag to the least significant bit
pub fn srl(registers: &mut Registers, reg: EightBitRegister) {
    let value = registers.read_eight(reg);
    let result = srl_val(registers, value);
    registers.write_eight(reg, result);
}

/// Rotate a value right, setting the carry flag to the most significant bit
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

/// Rotate right through carry. i.e. carry goes into bit 0 and bit 7 goes into carry
pub fn rr(registers: &mut Registers, reg: EightBitRegister) {
    let value = registers.read_eight(reg);
    let result = rr_val(registers, value);
    registers.write_eight(reg, result);
}

/// Rotate a value left, setting the carry flag to the most significant bit
pub fn rlc_val(registers: &mut Registers, value: u8) -> u8 {
    let carry = value & 0b1000_0000 == 0b1000_0000;

    let result = value << 1 | (value >> 7);
    registers.set_carry_flag(carry);
    registers.set_half_carry_flag(false);
    registers.set_subtract_flag(false);
    registers.set_zero_flag(result == 0);

    result
}

/// Rotate left through carry. i.e. carry goes into bit 7 and bit 7 goes into carry
pub fn rlc(registers: &mut Registers, reg: EightBitRegister) {
    let value = registers.read_eight(reg);
    let result = rlc_val(registers, value);
    registers.write_eight(reg, result);
}

/// Rotate a value left, setting the carry flag to the most significant bit
pub fn rrc_val(registers: &mut Registers, value: u8) -> u8 {
    let carry = value & 0b0000_0001 == 0b0000_0001;

    let result = value >> 1 | (value << 7);
    registers.set_carry_flag(carry);
    registers.set_half_carry_flag(false);
    registers.set_subtract_flag(false);
    registers.set_zero_flag(result == 0);

    result
}

/// Rotate a value right, setting the carry flag to the least significant bit
pub fn rrc(registers: &mut Registers, reg: EightBitRegister) {
    let value = registers.read_eight(reg);
    let result = rrc_val(registers, value);
    registers.write_eight(reg, result);
}

/// Rotate a value left through the carry flag, i.e. carry goes into bit 0 and bit 7 goes into
/// carry
pub fn rl_val(registers: &mut Registers, value: u8) -> u8 {
    let carry = value & 0b1000_0000 == 0b1000_0000;

    let result = value << 1
        | if registers.get_carry_flag() {
            0b0000_0001
        } else {
            0b0000_0000
        };
    registers.set_carry_flag(carry);
    registers.set_half_carry_flag(false);
    registers.set_subtract_flag(false);
    registers.set_zero_flag(result == 0);

    result
}

/// Rotate a value left through the carry flag, i.e. carry goes into bit 0 and bit 7 goes into
/// carry
pub fn rl(registers: &mut Registers, reg: EightBitRegister) {
    let value = registers.read_eight(reg);
    let result = rl_val(registers, value);
    registers.write_eight(reg, result);
}

/// Arithmetic shift left a value, setting the carry flag to the most significant bit
pub fn sla_val(registers: &mut Registers, value: u8) -> u8 {
    let carry = value & 0b1000_0000 == 0b1000_0000;

    let result = value << 1;
    registers.set_carry_flag(carry);
    registers.set_half_carry_flag(false);
    registers.set_subtract_flag(false);
    registers.set_zero_flag(result == 0);

    result
}

/// Arithmetic shift left a register, setting the carry flag to the most significant bit
pub fn sla(registers: &mut Registers, reg: EightBitRegister) {
    let value = registers.read_eight(reg);
    let result = sla_val(registers, value);
    registers.write_eight(reg, result);
}

pub fn sra_val(registers: &mut Registers, value: u8) -> u8 {
    let carry = value & 0b0000_0001 == 0b0000_0001;

    let result = (value >> 1) | (value & 0b1000_0000);
    registers.set_carry_flag(carry);
    registers.set_half_carry_flag(false);
    registers.set_subtract_flag(false);
    registers.set_zero_flag(result == 0);

    result
}

pub fn sra(registers: &mut Registers, reg: EightBitRegister) {
    let value = registers.read_eight(reg);
    let result = sra_val(registers, value);
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
        assert!(!registers.get_carry_flag());
        assert!(!registers.get_half_carry_flag());
        assert!(!registers.get_subtract_flag());
        assert!(!registers.get_zero_flag());
    }

    #[test]
    fn test_srl_val_zero() {
        let mut registers = Registers::new();
        let value = 0b0000_0001;
        let result = srl_val(&mut registers, value);
        assert_eq!(result, 0b0000_0000);
        assert!(registers.get_carry_flag());
        assert!(!registers.get_half_carry_flag());
        assert!(!registers.get_subtract_flag());
        assert!(registers.get_zero_flag());
    }

    #[test]
    fn test_srl_val_carry() {
        let mut registers = Registers::new();
        let value = 0b0000_0001;
        let result = srl_val(&mut registers, value);
        assert_eq!(result, 0b0000_0000);
        assert!(registers.get_carry_flag());
        assert!(!registers.get_half_carry_flag());
        assert!(!registers.get_subtract_flag());
        assert!(registers.get_zero_flag());
    }

    #[test]
    fn test_srl_basic() {
        let mut registers = Registers::new();
        registers.write_eight(EightBitRegister::B, 0b1010_1010);
        srl(&mut registers, EightBitRegister::B);
        assert_eq!(registers.read_eight(EightBitRegister::B), 0b0101_0101);
        assert!(!registers.get_carry_flag());
        assert!(!registers.get_half_carry_flag());
        assert!(!registers.get_subtract_flag());
        assert!(!registers.get_zero_flag());
    }

    #[test]
    fn test_rr_val_basic() {
        let mut registers = Registers::new();
        let value = 0b1010_1010;
        let result = rr_val(&mut registers, value);
        assert_eq!(result, 0b1101_0101);
        assert!(!registers.get_carry_flag());
        assert!(!registers.get_half_carry_flag());
        assert!(!registers.get_subtract_flag());
        assert!(!registers.get_zero_flag());
    }

    #[test]
    fn test_rr_val_carry() {
        let mut registers = Registers::new();
        let value = 0b1010_1010;
        registers.set_carry_flag(true);
        let result = rr_val(&mut registers, value);
        assert_eq!(result, 0b1101_0101);
        assert!(!registers.get_carry_flag());
        assert!(!registers.get_half_carry_flag());
        assert!(!registers.get_subtract_flag());
        assert!(!registers.get_zero_flag());
    }

    #[test]
    fn test_rr_val_zero() {
        let mut registers = Registers::new();
        let value = 0b0000_0001;
        registers.set_carry_flag(false);
        let result = rr_val(&mut registers, value);
        assert_eq!(result, 0b0000_0000);
        assert!(registers.get_carry_flag());
        assert!(!registers.get_half_carry_flag());
        assert!(!registers.get_subtract_flag());
        assert!(registers.get_zero_flag());
    }

    #[test]
    fn test_rr_basic() {
        let mut registers = Registers::new();
        registers.write_eight(EightBitRegister::B, 0b1010_1010);
        rr(&mut registers, EightBitRegister::B);
        assert_eq!(registers.read_eight(EightBitRegister::B), 0b1101_0101);
        assert!(!registers.get_carry_flag());
        assert!(!registers.get_half_carry_flag());
        assert!(!registers.get_subtract_flag());
        assert!(!registers.get_zero_flag());
    }

    #[test]
    fn test_rlc_value_basic() {
        let mut registers = Registers::new();
        let value = 0b1010_1010;
        let result = rlc_val(&mut registers, value);
        assert_eq!(result, 0b0101_0101);
        assert!(registers.get_carry_flag());
        assert!(!registers.get_half_carry_flag());
        assert!(!registers.get_subtract_flag());
        assert!(!registers.get_zero_flag());
    }

    #[test]
    fn test_rlc_value_zero() {
        let mut registers = Registers::new();
        let value = 0b0000_0000;
        let result = rlc_val(&mut registers, value);
        assert_eq!(result, 0b0000_0000);
        assert!(!registers.get_carry_flag());
        assert!(!registers.get_half_carry_flag());
        assert!(!registers.get_subtract_flag());
        assert!(registers.get_zero_flag());
    }

    #[test]
    fn test_rlc_basic() {
        let mut registers = Registers::new();
        registers.write_eight(EightBitRegister::B, 0b1010_1010);
        rlc(&mut registers, EightBitRegister::B);
        assert_eq!(registers.read_eight(EightBitRegister::B), 0b0101_0101);
        assert!(registers.get_carry_flag());
        assert!(!registers.get_half_carry_flag());
        assert!(!registers.get_subtract_flag());
        assert!(!registers.get_zero_flag());
    }

    #[test]
    fn test_rrc_value_basic() {
        let mut registers = Registers::new();
        let value = 0b1010_1001;
        let result = rrc_val(&mut registers, value);
        assert_eq!(result, 0b1101_0100);
        assert!(registers.get_carry_flag());
        assert!(!registers.get_half_carry_flag());
        assert!(!registers.get_subtract_flag());
        assert!(!registers.get_zero_flag());
    }

    #[test]
    fn test_rrc_value_zero() {
        let mut registers = Registers::new();
        let value = 0b0000_0000;
        let result = rrc_val(&mut registers, value);
        assert_eq!(result, 0b0000_0000);
        assert!(!registers.get_carry_flag());
        assert!(!registers.get_half_carry_flag());
        assert!(!registers.get_subtract_flag());
        assert!(registers.get_zero_flag());
    }

    #[test]
    fn test_rrc_basic() {
        let mut registers = Registers::new();
        registers.write_eight(EightBitRegister::B, 0b1010_1001);
        rrc(&mut registers, EightBitRegister::B);
        assert_eq!(registers.read_eight(EightBitRegister::B), 0b1101_0100);
        assert!(registers.get_carry_flag());
        assert!(!registers.get_half_carry_flag());
        assert!(!registers.get_subtract_flag());
        assert!(!registers.get_zero_flag());
    }

    #[test]
    fn test_rl_value_basic() {
        let mut registers = Registers::new();
        let value = 0b1010_1010;
        let result = rl_val(&mut registers, value);
        assert_eq!(result, 0b0101_0101);
        assert!(registers.get_carry_flag());
        assert!(!registers.get_half_carry_flag());
        assert!(!registers.get_subtract_flag());
        assert!(!registers.get_zero_flag());
    }

    #[test]
    fn test_rl_value_zero() {
        let mut registers = Registers::new();
        registers.set_carry_flag(false);
        let value = 0b0000_0000;
        let result = rl_val(&mut registers, value);
        assert_eq!(result, 0b0000_0000);
        assert!(!registers.get_carry_flag());
        assert!(!registers.get_half_carry_flag());
        assert!(!registers.get_subtract_flag());
        assert!(registers.get_zero_flag());
    }

    #[test]
    fn test_rl_basic() {
        let mut registers = Registers::new();
        registers.write_eight(EightBitRegister::B, 0b1010_1010);
        rl(&mut registers, EightBitRegister::B);
        assert_eq!(registers.read_eight(EightBitRegister::B), 0b0101_0101);
        assert!(registers.get_carry_flag());
        assert!(!registers.get_half_carry_flag());
        assert!(!registers.get_subtract_flag());
        assert!(!registers.get_zero_flag());
    }

    #[test]
    fn test_sla_value_basic() {
        let mut registers = Registers::new();
        let value = 0b1010_1010;
        let result = sla_val(&mut registers, value);
        assert_eq!(result, 0b0101_0100);
        assert!(registers.get_carry_flag());
        assert!(!registers.get_half_carry_flag());
        assert!(!registers.get_subtract_flag());
        assert!(!registers.get_zero_flag());
    }

    #[test]
    fn test_sla_value_zero() {
        let mut registers = Registers::new();
        let value = 0b0000_0000;
        let result = sla_val(&mut registers, value);
        assert_eq!(result, 0b0000_0000);
        assert!(!registers.get_carry_flag());
        assert!(!registers.get_half_carry_flag());
        assert!(!registers.get_subtract_flag());
        assert!(registers.get_zero_flag());
    }

    #[test]
    fn test_sla_basic() {
        let mut registers = Registers::new();
        registers.write_eight(EightBitRegister::B, 0b1010_1010);
        sla(&mut registers, EightBitRegister::B);
        assert_eq!(registers.read_eight(EightBitRegister::B), 0b0101_0100);
        assert!(registers.get_carry_flag());
        assert!(!registers.get_half_carry_flag());
        assert!(!registers.get_subtract_flag());
        assert!(!registers.get_zero_flag());
    }

    #[test]
    fn test_sra_value_basic() {
        let mut registers = Registers::new();
        let value = 0b1010_1010;
        let result = sra_val(&mut registers, value);
        assert_eq!(result, 0b1101_0101);
        assert!(!registers.get_carry_flag());
        assert!(!registers.get_half_carry_flag());
        assert!(!registers.get_subtract_flag());
        assert!(!registers.get_zero_flag());
    }

    #[test]
    fn test_sra_value_zero() {
        let mut registers = Registers::new();
        let value = 0b0000_0000;
        let result = sra_val(&mut registers, value);
        assert_eq!(result, 0b0000_0000);
        assert!(!registers.get_carry_flag());
        assert!(!registers.get_half_carry_flag());
        assert!(!registers.get_subtract_flag());
        assert!(registers.get_zero_flag());
    }

    #[test]
    fn test_sra_basic() {
        let mut registers = Registers::new();
        registers.write_eight(EightBitRegister::B, 0b1010_1010);
        sra(&mut registers, EightBitRegister::B);
        assert_eq!(registers.read_eight(EightBitRegister::B), 0b1101_0101);
        assert!(!registers.get_carry_flag());
        assert!(!registers.get_half_carry_flag());
        assert!(!registers.get_subtract_flag());
        assert!(!registers.get_zero_flag());
    }
}
