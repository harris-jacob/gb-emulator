mod alu_operations;
use alu_operations::*;

use crate::core::*;

pub struct CPU {
    registers: Registers,
    mmu: MMU,
    clock: u64,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            registers: Registers::new(),
            mmu: MMU::new(),
            clock: 0,
        }
    }
    /// Handles provided opcode, updating the CPU state and returning the number of cycles taken.
    pub fn handle_op(&mut self, opcode: u8) -> u8 {
        match opcode {
            // NOP
            0x00 => 1,
            // LD BC, d16
            0x01 => {
                self.registers.write_sixteen(
                    SixteenBitRegister::BC,
                    self.mmu
                        .read_u16(self.registers.read_sixteen(SixteenBitRegister::PC)),
                );
                4
            }
            // LD (BC), A
            0x02 => {
                let addr = self.registers.read_sixteen(SixteenBitRegister::BC);
                let value = self.registers.read_eight(EightBitRegister::A);
                self.mmu.write_u8(addr, value);
                2
            }
            // INC BC
            0x03 => {
                let value = self
                    .registers
                    .read_sixteen(SixteenBitRegister::BC)
                    .wrapping_add(1);
                self.registers.write_sixteen(SixteenBitRegister::BC, value);
                2
            }
            // INC B
            0x04 => {
                let value = self.registers.read_eight(EightBitRegister::B);
                let result = alu_inc(&mut self.registers, value);
                self.registers.write_eight(EightBitRegister::B, result);
                1
            }
            // DEC B
            0x05 => {
                let value = self.registers.read_eight(EightBitRegister::B);
                let result = value.wrapping_sub(1);
                self.registers.write_eight(EightBitRegister::B, result);
                1
            }

            // LD B, d8
            0x06 => {
                let value = self
                    .mmu
                    .read_u8(self.registers.read_sixteen(SixteenBitRegister::PC));
                self.registers.write_eight(EightBitRegister::B, value);
                2
            }

            // RLCA
            0x07 => {
                rlca(&mut self.registers);
                1
            }

            // LD [a16], SP
            0x08 => {
                let value = self
                    .mmu
                    .read_u16(self.registers.read_sixteen(SixteenBitRegister::PC));

                self.registers.write_sixteen(SixteenBitRegister::SP, value);

                5
            }

            // ADD HL, BC
            0x09 => {
                let value = self
                    .registers
                    .read_sixteen(SixteenBitRegister::HL)
                    .wrapping_add(self.registers.read_sixteen(SixteenBitRegister::BC));

                self.registers.write_sixteen(SixteenBitRegister::HL, value);

                2
            }

            // LD A, [BC]
            0x0A => {
                let value = self
                    .mmu
                    .read_u8(self.registers.read_sixteen(SixteenBitRegister::BC));

                self.registers.write_eight(
                    EightBitRegister::A,
                    self.registers
                        .read_eight(EightBitRegister::A)
                        .wrapping_add(value),
                );

                2
            }

            // DEC BC
            0x0B => {
                self.registers
                    .update_sixteen(SixteenBitRegister::BC, |bc| bc.wrapping_sub(1));

                2
            }

            // Inc C
            0x0C => {
                let c = self.registers.read_eight(EightBitRegister::C);
                let value = alu_inc(&mut self.registers, c);

                self.registers.write_eight(EightBitRegister::C, value);

                2
            }

            _ => {
                panic!("Unknown opcode: {:#04x}", opcode);
            }
        }
    }
}
