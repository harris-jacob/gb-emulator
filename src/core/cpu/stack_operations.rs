use std::ops::Sub;

use super::*;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum CallResult {
    Called,
    DidNotCall,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ReturnResult {
    Returned,
    DidNotReturn,
}

/// Call a subroutine at the given address.
pub fn call(cpu: &mut CPU, address: u16) {
    stack_push(cpu, cpu.registers.read_sixteen(SixteenBitRegister::PC));
    cpu.registers.write_sixteen(SixteenBitRegister::PC, address);
}

pub fn call_nz(cpu: &mut CPU, address: u16) -> CallResult {
    if !cpu.registers.get_zero_flag() {
        call(cpu, address);
        CallResult::Called
    } else {
        CallResult::DidNotCall
    }
}

pub fn call_z(cpu: &mut CPU, address: u16) -> CallResult {
    if cpu.registers.get_zero_flag() {
        call(cpu, address);
        CallResult::Called
    } else {
        CallResult::DidNotCall
    }
}

pub fn call_nc(cpu: &mut CPU, address: u16) -> CallResult {
    if !cpu.registers.get_carry_flag() {
        call(cpu, address);
        CallResult::Called
    } else {
        CallResult::DidNotCall
    }
}

pub fn call_c(cpu: &mut CPU, address: u16) -> CallResult {
    if cpu.registers.get_carry_flag() {
        call(cpu, address);
        CallResult::Called
    } else {
        CallResult::DidNotCall
    }
}

/// Return from a subroutine.
pub fn ret(cpu: &mut CPU) {
    let address = stack_pop(cpu);
    cpu.registers.write_sixteen(SixteenBitRegister::PC, address);
}

/// Return from a subroutine if the zero flag is not set.
pub fn ret_nz(cpu: &mut CPU) -> ReturnResult {
    if !cpu.registers.get_zero_flag() {
        ret(cpu);
        ReturnResult::Returned
    } else {
        ReturnResult::DidNotReturn
    }
}

/// Return from a subroutine if the zero flag is set.
pub fn ret_z(cpu: &mut CPU) -> ReturnResult {
    if cpu.registers.get_zero_flag() {
        ret(cpu);
        ReturnResult::Returned
    } else {
        ReturnResult::DidNotReturn
    }
}

/// Return from a subroutine if the carry flag is not set.
pub fn ret_nc(cpu: &mut CPU) -> ReturnResult {
    if !cpu.registers.get_carry_flag() {
        ret(cpu);
        ReturnResult::Returned
    } else {
        ReturnResult::DidNotReturn
    }
}

/// Return from a subroutine if the carry flag is set.
pub fn ret_c(cpu: &mut CPU) -> ReturnResult {
    if cpu.registers.get_carry_flag() {
        ret(cpu);
        ReturnResult::Returned
    } else {
        ReturnResult::DidNotReturn
    }
}

pub fn stack_push(cpu: &mut CPU, value: u16) {
    let sp = cpu.registers.read_sixteen(SixteenBitRegister::SP).sub(2);
    cpu.mmu.write_u16(sp, value);
    cpu.registers.write_sixteen(SixteenBitRegister::SP, sp);
}

pub fn stack_pop(cpu: &mut CPU) -> u16 {
    let sp = cpu.registers.read_sixteen(SixteenBitRegister::SP);
    let value = cpu.mmu.read_u16(sp);

    cpu.registers
        .update_sixteen(SixteenBitRegister::SP, |sp| sp + 2);

    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stack_push_and_pop() {
        let mut cpu = CPU::new();
        cpu.registers.write_sixteen(SixteenBitRegister::SP, 0xFFFE);
        stack_push(&mut cpu, 0x1234);
        stack_push(&mut cpu, 0x5678);
        stack_push(&mut cpu, 0x9ABC);
        assert_eq!(stack_pop(&mut cpu), 0x9ABC);
        assert_eq!(stack_pop(&mut cpu), 0x5678);
        assert_eq!(stack_pop(&mut cpu), 0x1234);
    }

    #[test]
    fn call_and_return() {
        let mut cpu = CPU::new();
        cpu.registers.write_sixteen(SixteenBitRegister::PC, 0x1234);
        call(&mut cpu, 0x5678);
        assert_eq!(cpu.registers.read_sixteen(SixteenBitRegister::PC), 0x5678);
        ret(&mut cpu);
        assert_eq!(cpu.registers.read_sixteen(SixteenBitRegister::PC), 0x1234);
    }

    #[test]
    fn call_nz_when_not_zero() {
        let mut cpu = CPU::new();
        cpu.registers.set_zero_flag(false);
        cpu.registers.write_sixteen(SixteenBitRegister::PC, 0x1234);
        let result = call_nz(&mut cpu, 0x5678);
        assert_eq!(result, CallResult::Called);
        assert_eq!(cpu.registers.read_sixteen(SixteenBitRegister::PC), 0x5678);
    }

    #[test]
    fn call_nz_when_zero() {
        let mut cpu = CPU::new();
        cpu.registers.set_zero_flag(true);
        cpu.registers.write_sixteen(SixteenBitRegister::PC, 0x1234);
        let result = call_nz(&mut cpu, 0x5678);
        assert_eq!(result, CallResult::DidNotCall);
        assert_eq!(cpu.registers.read_sixteen(SixteenBitRegister::PC), 0x1234);
    }

    #[test]
    fn call_z_when_zero() {
        let mut cpu = CPU::new();
        cpu.registers.set_zero_flag(true);
        cpu.registers.write_sixteen(SixteenBitRegister::PC, 0x1234);
        let result = call_z(&mut cpu, 0x5678);
        assert_eq!(result, CallResult::Called);
        assert_eq!(cpu.registers.read_sixteen(SixteenBitRegister::PC), 0x5678);
    }

    #[test]
    fn call_z_when_not_zero() {
        let mut cpu = CPU::new();
        cpu.registers.set_zero_flag(false);
        cpu.registers.write_sixteen(SixteenBitRegister::PC, 0x1234);
        let result = call_z(&mut cpu, 0x5678);
        assert_eq!(result, CallResult::DidNotCall);
        assert_eq!(cpu.registers.read_sixteen(SixteenBitRegister::PC), 0x1234);
    }

    #[test]
    fn call_nc_when_not_carry() {
        let mut cpu = CPU::new();
        cpu.registers.set_carry_flag(false);
        cpu.registers.write_sixteen(SixteenBitRegister::PC, 0x1234);
        let result = call_nc(&mut cpu, 0x5678);
        assert_eq!(result, CallResult::Called);
        assert_eq!(cpu.registers.read_sixteen(SixteenBitRegister::PC), 0x5678);
    }

    #[test]
    fn call_nc_when_carry() {
        let mut cpu = CPU::new();
        cpu.registers.set_carry_flag(true);
        cpu.registers.write_sixteen(SixteenBitRegister::PC, 0x1234);
        let result = call_nc(&mut cpu, 0x5678);
        assert_eq!(result, CallResult::DidNotCall);
        assert_eq!(cpu.registers.read_sixteen(SixteenBitRegister::PC), 0x1234);
    }

    #[test]
    fn call_c_when_not_zero() {
        let mut cpu = CPU::new();
        cpu.registers.set_carry_flag(false);
        cpu.registers.write_sixteen(SixteenBitRegister::PC, 0x1234);
        let result = call_c(&mut cpu, 0x5678);
        assert_eq!(result, CallResult::DidNotCall);
        assert_eq!(cpu.registers.read_sixteen(SixteenBitRegister::PC), 0x1234);
    }

    #[test]
    fn call_c_when_zero() {
        let mut cpu = CPU::new();
        cpu.registers.set_carry_flag(true);
        cpu.registers.write_sixteen(SixteenBitRegister::PC, 0x1234);
        let result = call_c(&mut cpu, 0x5678);
        assert_eq!(result, CallResult::Called);
        assert_eq!(cpu.registers.read_sixteen(SixteenBitRegister::PC), 0x5678);
    }

    #[test]
    fn ret_nz_when_not_zero() {
        let mut cpu = CPU::new();
        cpu.registers.set_zero_flag(false);
        cpu.registers.write_sixteen(SixteenBitRegister::PC, 0x1234);
        stack_push(&mut cpu, 0x5678);
        let result = ret_nz(&mut cpu);
        assert_eq!(result, ReturnResult::Returned);
        assert_eq!(cpu.registers.read_sixteen(SixteenBitRegister::PC), 0x5678);
    }
}
