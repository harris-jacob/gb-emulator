use super::*;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum JumpResult {
    Jumped,
    DidNotJump,
}

// Performs a relative jump if the zero flag is not set.
// otherwise, does nothing.
pub fn jr_nz(registers: &mut Registers, value: u8) -> JumpResult {
    if !registers.get_zero_flag() {
        jr(registers, value);
        JumpResult::Jumped
    } else {
        JumpResult::DidNotJump
    }
}

// Performs a relative jump if the zero flag is set.
pub fn jr_z(registers: &mut Registers, value: u8) -> JumpResult {
    if registers.get_zero_flag() {
        jr(registers, value);
        JumpResult::Jumped
    } else {
        JumpResult::DidNotJump
    }
}

// Performs a relative jump if the carry flag is not set.
pub fn jr_nc(registers: &mut Registers, value: u8) -> JumpResult {
    if !registers.get_carry_flag() {
        jr(registers, value);
        JumpResult::Jumped
    } else {
        JumpResult::DidNotJump
    }
}

// Performs a relative jump if the carry flag is set.
pub fn jr_c(registers: &mut Registers, value: u8) -> JumpResult {
    if registers.get_carry_flag() {
        jr(registers, value);
        JumpResult::Jumped
    } else {
        JumpResult::DidNotJump
    }
}


    #[test]
    fn jr_positive() {
        let mut registers = Registers::new();
        registers.write_sixteen(SixteenBitRegister::PC, 0x20);

        jr(&mut registers, 0x01);

        assert_eq!(registers.read_sixteen(SixteenBitRegister::PC), 0x0021);
    }

    #[test]
    fn jr_negative() {
        let mut registers = Registers::new();
        registers.write_sixteen(SixteenBitRegister::PC, 0x20);

        jr(&mut registers, 0xff);

        assert_eq!(registers.read_sixteen(SixteenBitRegister::PC), 0x001f);
    }

    #[test]
    fn jr_nz_not_zero() {
        let mut registers = Registers::new();
        registers.set_zero_flag(false);
        registers.write_sixteen(SixteenBitRegister::PC, 0x20);

        let result = jr_nz(&mut registers, 0x01);

        assert_eq!(registers.read_sixteen(SixteenBitRegister::PC), 0x0021);
        assert!(result == JumpResult::Jumped);
    }

    #[test]
    fn jr_nz_zero() {
        let mut registers = Registers::new();
        registers.set_zero_flag(true);
        registers.write_sixteen(SixteenBitRegister::PC, 0x20);

        let result = jr_nz(&mut registers, 0x01);

        assert_eq!(registers.read_sixteen(SixteenBitRegister::PC), 0x0020);
        assert!(result == JumpResult::DidNotJump);
    }

    #[test]
    fn jr_z_zero() {
        let mut registers = Registers::new();
        registers.set_zero_flag(true);
        registers.write_sixteen(SixteenBitRegister::PC, 0x20);

        let result = jr_z(&mut registers, 0x01);

        assert_eq!(registers.read_sixteen(SixteenBitRegister::PC), 0x0021);
        assert!(result == JumpResult::Jumped);
    }

    #[test]
    fn jr_z_not_zero() {
        let mut registers = Registers::new();
        registers.set_zero_flag(false);
        registers.write_sixteen(SixteenBitRegister::PC, 0x20);

        let result = jr_z(&mut registers, 0x01);

        assert_eq!(registers.read_sixteen(SixteenBitRegister::PC), 0x0020);
        assert!(result == JumpResult::DidNotJump);
    }

    #[test]
    fn jr_nc_no_carry() {
        let mut registers = Registers::new();
        registers.set_carry_flag(false);
        registers.write_sixteen(SixteenBitRegister::PC, 0x20);

        let result = jr_nc(&mut registers, 0x01);

        assert_eq!(registers.read_sixteen(SixteenBitRegister::PC), 0x0021);
        assert!(result == JumpResult::Jumped);
    }

    #[test]
    fn jr_nc_carry() {
        let mut registers = Registers::new();
        registers.set_carry_flag(true);
        registers.write_sixteen(SixteenBitRegister::PC, 0x20);

        let result = jr_nc(&mut registers, 0x01);

        assert_eq!(registers.read_sixteen(SixteenBitRegister::PC), 0x0020);
        assert!(result == JumpResult::DidNotJump);
    }
