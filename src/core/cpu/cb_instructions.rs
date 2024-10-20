use super::*;
use bitwise_operations::*;

pub fn handle_cb_instructions(cpu: &mut CPU, opcode: u8) {
    match opcode {
        // RR B
        0x18 => {
            rr(&mut cpu.registers, EightBitRegister::B);
        }
        // RR C
        0x19 => {
            rr(&mut cpu.registers, EightBitRegister::C);
        }
        // RR D
        0x1A => {
            rr(&mut cpu.registers, EightBitRegister::D);
        }
        // RR E
        0x1B => {
            rr(&mut cpu.registers, EightBitRegister::E);
        }
        // RR H
        0x1C => {
            rr(&mut cpu.registers, EightBitRegister::H);
        }
        // RR L
        0x1D => {
            rr(&mut cpu.registers, EightBitRegister::L);
        }
        // RR (HL)
        0x1E => {
            let address = cpu.registers.read_sixteen(SixteenBitRegister::HL);
            let value = cpu.mmu.read_u8(address);
            let result = rr_val(&mut cpu.registers, value);
            cpu.mmu.write_u8(address, result);
        }
        // SRL B
        0x38 => {
            srl(&mut cpu.registers, EightBitRegister::B);
        }

        // SRL C
        0x39 => {
            srl(&mut cpu.registers, EightBitRegister::C);
        }

        // SRL D
        0x3A => {
            srl(&mut cpu.registers, EightBitRegister::D);
        }

        // SRL E
        0x3B => {
            srl(&mut cpu.registers, EightBitRegister::E);
        }

        // SRL H
        0x3C => {
            srl(&mut cpu.registers, EightBitRegister::H);
        }

        // SRL L
        0x3D => {
            srl(&mut cpu.registers, EightBitRegister::L);
        }

        // SRL (HL)
        0x3E => {
            let address = cpu.registers.read_sixteen(SixteenBitRegister::HL);
            let value = cpu.mmu.read_u8(address);
            let result = srl_val(&mut cpu.registers, value);
            cpu.mmu.write_u8(address, result);
        }

        _ => {
            panic!("Unknown CB instruction: 0x{:02X}", opcode);
        }
    }
}
