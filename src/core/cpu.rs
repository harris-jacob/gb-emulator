mod alu_operations;
mod bitwise_operations;
mod cb_instructions;
mod interrupts;
mod jp_operations;
mod shift_operations;
mod stack_operations;
use alu_operations::*;
use jp_operations::*;
use stack_operations::*;

use crate::core::*;

pub struct CPU {
    halted: bool,
    pub mmu: MMU,
    registers: Registers,
    ime: bool,
    stopped: bool,
}

impl CPU {
    pub fn new(mmu: MMU) -> Self {
        CPU {
            halted: false,
            registers: Registers::new(),
            ime: false,
            stopped: false,
            mmu,
        }
    }

    pub fn step(&mut self) -> u8 {
        if self.stopped {
            panic!("CPU is stopped");
        }

        let cycles = if !self.halted {
            let opcode = self.fetch_u8();
            self.handle_op(opcode)
        } else {
            1
        };

        self.mmu.step(cycles);

        let interrupt_cycles = self.interrupt_step();

        cycles + interrupt_cycles
    }

    pub fn debug_output(&self) {
        println!("A:{:02X} F:{:02X} B:{:02X} C:{:02X} D:{:02X} E:{:02X} H:{:02X} L:{:02X} SP:{:04X} PC:{:04X} PCMEM:{:02X},{:02X},{:02X},{:02X}",
                 self.registers.read_eight(EightBitRegister::A),
                 self.registers.read_eight(EightBitRegister::F),
                 self.registers.read_eight(EightBitRegister::B),
                 self.registers.read_eight(EightBitRegister::C),
                 self.registers.read_eight(EightBitRegister::D),
                 self.registers.read_eight(EightBitRegister::E),
                 self.registers.read_eight(EightBitRegister::H),
                 self.registers.read_eight(EightBitRegister::L),
                 self.registers.read_sixteen(SixteenBitRegister::SP),
                 self.registers.read_sixteen(SixteenBitRegister::PC),
                 self.mmu.read_u8(self.registers.read_sixteen(SixteenBitRegister::PC)),
                 self.mmu.read_u8(self.registers.read_sixteen(SixteenBitRegister::PC) + 1),
                 self.mmu.read_u8(self.registers.read_sixteen(SixteenBitRegister::PC) + 2),
                 self.mmu.read_u8(self.registers.read_sixteen(SixteenBitRegister::PC) + 3)
        );
    }

    fn fetch_u8(&mut self) -> u8 {
        let value = self
            .mmu
            .read_u8(self.registers.read_sixteen(SixteenBitRegister::PC));
        self.registers
            .update_sixteen(SixteenBitRegister::PC, |pc| pc.wrapping_add(1));
        value
    }

    fn fetch_u16(&mut self) -> u16 {
        let value = self
            .mmu
            .read_u16(self.registers.read_sixteen(SixteenBitRegister::PC));
        self.registers
            .update_sixteen(SixteenBitRegister::PC, |pc| pc.wrapping_add(2));
        value
    }

    /// Handles provided opcode, updating the CPU state and returning the number of cycles taken.
    fn handle_op(&mut self, opcode: u8) -> u8 {
        match opcode {
            // NOP
            0x00 => 1,
            // LD BC, d16
            0x01 => {
                let value = self.fetch_u16();
                self.registers.write_sixteen(SixteenBitRegister::BC, value);
                3
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
                self.registers
                    .update_sixteen(SixteenBitRegister::BC, |bc| bc.wrapping_add(1));
                2
            }
            // INC B
            0x04 => {
                alu_inc(&mut self.registers, EightBitRegister::B);
                1
            }
            // DEC B
            0x05 => {
                alu_dec(&mut self.registers, EightBitRegister::B);
                1
            }
            // LD B, d8
            0x06 => {
                let value = self.fetch_u8();
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
                let value = self.fetch_u16();
                let sp = self.registers.read_sixteen(SixteenBitRegister::SP);
                self.mmu.write_u16(value, sp);
                5
            }
            // ADD HL, BC
            0x09 => {
                alu_add_16(&mut self.registers, SixteenBitRegister::BC);
                2
            }
            // LD A, [BC]
            0x0A => {
                let value = self
                    .mmu
                    .read_u8(self.registers.read_sixteen(SixteenBitRegister::BC));
                self.registers.write_eight(EightBitRegister::A, value);
                2
            }
            // DEC BC
            0x0B => {
                self.registers
                    .update_sixteen(SixteenBitRegister::BC, |bc| bc.wrapping_sub(1));
                2
            }
            // INC C
            0x0C => {
                alu_inc(&mut self.registers, EightBitRegister::C);
                1
            }
            // DEC C
            0x0D => {
                alu_dec(&mut self.registers, EightBitRegister::C);
                1
            }
            // LD C, d8
            0x0E => {
                let value = self.fetch_u8();
                self.registers.write_eight(EightBitRegister::C, value);
                2
            }
            // RRCA
            0x0F => {
                rrca(&mut self.registers);
                1
            }
            // STOP n8
            0x10 => {
                // self.stopped = true;
                1
            }
            // LD DE, d16
            0x11 => {
                let value = self.fetch_u16();
                self.registers.write_sixteen(SixteenBitRegister::DE, value);
                3
            }
            // LD (DE), A
            0x12 => {
                let addr = self.registers.read_sixteen(SixteenBitRegister::DE);
                let value = self.registers.read_eight(EightBitRegister::A);
                self.mmu.write_u8(addr, value);
                2
            }
            // INC DE
            0x13 => {
                self.registers
                    .update_sixteen(SixteenBitRegister::DE, |de| de.wrapping_add(1));
                2
            }
            // INC D
            0x14 => {
                alu_inc(&mut self.registers, EightBitRegister::D);
                1
            }
            // DEC D
            0x15 => {
                alu_dec(&mut self.registers, EightBitRegister::D);
                1
            }
            // LD D, d8
            0x16 => {
                let value = self.fetch_u8();
                self.registers.write_eight(EightBitRegister::D, value);
                2
            }
            // RLA
            0x17 => {
                rla(&mut self.registers);
                1
            }
            // JR r8
            0x18 => {
                let value = self.fetch_u8();
                jr(&mut self.registers, value);
                1
            }
            // ADD HL, DE
            0x19 => {
                alu_add_16(&mut self.registers, SixteenBitRegister::DE);
                2
            }
            // LD A, [DE]
            0x1A => {
                let value = self
                    .mmu
                    .read_u8(self.registers.read_sixteen(SixteenBitRegister::DE));

                self.registers.write_eight(EightBitRegister::A, value);
                2
            }
            // DEC DE
            0x1B => {
                self.registers
                    .update_sixteen(SixteenBitRegister::DE, |de| de.wrapping_sub(1));
                2
            }
            // INC E
            0x1C => {
                alu_inc(&mut self.registers, EightBitRegister::E);
                1
            }
            // DEC E
            0x1D => {
                alu_dec(&mut self.registers, EightBitRegister::E);
                1
            }
            // LD E, d8
            0x1E => {
                let value = self.fetch_u8();
                self.registers.write_eight(EightBitRegister::E, value);
                2
            }
            // RRA
            0x1F => {
                rra(&mut self.registers);
                1
            }
            // JR NZ, r8
            0x20 => {
                let value = self.fetch_u8();
                match jr_nz(&mut self.registers, value) {
                    JumpResult::Jumped => 3,
                    JumpResult::DidNotJump => 2,
                }
            }
            // LD HL, d16
            0x21 => {
                let value = self.fetch_u16();
                self.registers.write_sixteen(SixteenBitRegister::HL, value);
                3
            }
            // LD (HL+), A
            0x22 => {
                let addr = self.registers.read_sixteen(SixteenBitRegister::HL);
                let value = self.registers.read_eight(EightBitRegister::A);
                self.mmu.write_u8(addr, value);
                self.registers
                    .update_sixteen(SixteenBitRegister::HL, |hl| hl.wrapping_add(1));
                2
            }
            // INC HL
            0x23 => {
                self.registers
                    .update_sixteen(SixteenBitRegister::HL, |hl| hl.wrapping_add(1));
                2
            }
            // INC H
            0x24 => {
                alu_inc(&mut self.registers, EightBitRegister::H);
                1
            }
            // DEC H
            0x25 => {
                alu_dec(&mut self.registers, EightBitRegister::H);
                1
            }
            // LD H, d8
            0x26 => {
                let value = self.fetch_u8();
                self.registers.write_eight(EightBitRegister::H, value);
                2
            }
            // DAA
            0x27 => {
                daa(&mut self.registers);
                1
            }
            // JR Z, r8
            0x28 => {
                let value = self.fetch_u8();
                match jr_z(&mut self.registers, value) {
                    JumpResult::Jumped => 3,
                    JumpResult::DidNotJump => 2,
                }
            }
            // ADD HL, HL
            0x29 => {
                alu_add_16(&mut self.registers, SixteenBitRegister::HL);
                2
            }
            // LD A, (HL+)
            0x2A => {
                let addr = self.registers.read_sixteen(SixteenBitRegister::HL);
                let value = self.mmu.read_u8(addr);
                self.registers.write_eight(EightBitRegister::A, value);
                self.registers
                    .update_sixteen(SixteenBitRegister::HL, |hl| hl.wrapping_add(1));
                2
            }
            // DEC HL
            0x2B => {
                self.registers
                    .update_sixteen(SixteenBitRegister::HL, |hl| hl.wrapping_sub(1));
                2
            }
            // INC L
            0x2C => {
                alu_inc(&mut self.registers, EightBitRegister::L);
                1
            }
            // DEC L
            0x2D => {
                alu_dec(&mut self.registers, EightBitRegister::L);
                1
            }
            // LD L, d8
            0x2E => {
                let value = self.fetch_u8();
                self.registers.write_eight(EightBitRegister::L, value);
                2
            }
            // CPL
            0x2F => {
                cpl(&mut self.registers);
                1
            }
            // JR NC, r8
            0x30 => {
                let value = self.fetch_u8();
                match jr_nc(&mut self.registers, value) {
                    JumpResult::Jumped => 3,
                    JumpResult::DidNotJump => 2,
                }
            }
            // LD SP, d16
            0x31 => {
                let value = self.fetch_u16();
                self.registers.write_sixteen(SixteenBitRegister::SP, value);
                3
            }
            // LD (HL-), A
            0x32 => {
                let addr = self.registers.read_sixteen(SixteenBitRegister::HL);
                let value = self.registers.read_eight(EightBitRegister::A);
                self.mmu.write_u8(addr, value);
                self.registers
                    .update_sixteen(SixteenBitRegister::HL, |hl| hl.wrapping_sub(1));
                2
            }
            // INC SP
            0x33 => {
                self.registers
                    .update_sixteen(SixteenBitRegister::SP, |sp| sp.wrapping_add(1));
                2
            }
            // INC (HL)
            0x34 => {
                let addr = self.registers.read_sixteen(SixteenBitRegister::HL);
                let value = self.mmu.read_u8(addr);
                let result = alu_inc_value(&mut self.registers, value);
                self.mmu.write_u8(addr, result);
                3
            }
            // DEC (HL)
            0x35 => {
                let addr = self.registers.read_sixteen(SixteenBitRegister::HL);
                let value = self.mmu.read_u8(addr);
                let result = alu_dec_value(&mut self.registers, value);
                self.mmu.write_u8(addr, result);
                3
            }
            // LD (HL), d8
            0x36 => {
                let value = self.fetch_u8();
                let addr = self.registers.read_sixteen(SixteenBitRegister::HL);
                self.mmu.write_u8(addr, value);
                3
            }
            // SCF
            0x37 => {
                scf(&mut self.registers);
                1
            }
            // JR C, r8
            0x38 => {
                let value = self.fetch_u8();
                match jr_c(&mut self.registers, value) {
                    JumpResult::Jumped => 3,
                    JumpResult::DidNotJump => 2,
                }
            }
            // ADD HL, SP
            0x39 => {
                alu_add_16(&mut self.registers, SixteenBitRegister::SP);
                2
            }
            // LD A, (HL-)
            0x3A => {
                let addr = self.registers.read_sixteen(SixteenBitRegister::HL);
                let value = self.mmu.read_u8(addr);
                self.registers.write_eight(EightBitRegister::A, value);
                self.registers
                    .update_sixteen(SixteenBitRegister::HL, |hl| hl.wrapping_sub(1));
                2
            }
            // DEC SP
            0x3B => {
                self.registers
                    .update_sixteen(SixteenBitRegister::SP, |sp| sp.wrapping_sub(1));
                2
            }
            // INC A
            0x3C => {
                alu_inc(&mut self.registers, EightBitRegister::A);
                1
            }
            // DEC A
            0x3D => {
                alu_dec(&mut self.registers, EightBitRegister::A);
                1
            }
            // LD A, d8
            0x3E => {
                let value = self.fetch_u8();
                self.registers.write_eight(EightBitRegister::A, value);
                2
            }
            // CCF
            0x3F => {
                ccf(&mut self.registers);
                1
            }
            // LD B, B
            0x40 => {
                self.registers.write_eight(
                    EightBitRegister::B,
                    self.registers.read_eight(EightBitRegister::B),
                );
                1
            }
            // LD B, C
            0x41 => {
                self.registers.write_eight(
                    EightBitRegister::B,
                    self.registers.read_eight(EightBitRegister::C),
                );
                1
            }
            // LD B, D
            0x42 => {
                self.registers.write_eight(
                    EightBitRegister::B,
                    self.registers.read_eight(EightBitRegister::D),
                );
                1
            }

            // LD B, E
            0x43 => {
                self.registers.write_eight(
                    EightBitRegister::B,
                    self.registers.read_eight(EightBitRegister::E),
                );

                1
            }

            // LD B, H
            0x44 => {
                self.registers.write_eight(
                    EightBitRegister::B,
                    self.registers.read_eight(EightBitRegister::H),
                );

                1
            }

            // LD B, L
            0x45 => {
                self.registers.write_eight(
                    EightBitRegister::B,
                    self.registers.read_eight(EightBitRegister::L),
                );

                1
            }

            // LD B, (HL)
            0x46 => {
                let addr = self.registers.read_sixteen(SixteenBitRegister::HL);
                let value = self.mmu.read_u8(addr);

                self.registers.write_eight(EightBitRegister::B, value);

                2
            }

            // LD B, A
            0x47 => {
                self.registers.write_eight(
                    EightBitRegister::B,
                    self.registers.read_eight(EightBitRegister::A),
                );

                1
            }

            // LD C, B
            0x48 => {
                self.registers.write_eight(
                    EightBitRegister::C,
                    self.registers.read_eight(EightBitRegister::B),
                );

                1
            }

            // LD C, C
            0x49 => {
                self.registers.write_eight(
                    EightBitRegister::C,
                    self.registers.read_eight(EightBitRegister::C),
                );

                1
            }

            // LD C, D
            0x4A => {
                self.registers.write_eight(
                    EightBitRegister::C,
                    self.registers.read_eight(EightBitRegister::D),
                );

                1
            }

            // LD C, E
            0x4B => {
                self.registers.write_eight(
                    EightBitRegister::C,
                    self.registers.read_eight(EightBitRegister::E),
                );

                1
            }

            // LD C, H
            0x4C => {
                self.registers.write_eight(
                    EightBitRegister::C,
                    self.registers.read_eight(EightBitRegister::H),
                );

                1
            }

            // LD C, L
            0x4D => {
                self.registers.write_eight(
                    EightBitRegister::C,
                    self.registers.read_eight(EightBitRegister::L),
                );

                1
            }

            // LD C, (HL)
            0x4E => {
                let addr = self.registers.read_sixteen(SixteenBitRegister::HL);
                let value = self.mmu.read_u8(addr);

                self.registers.write_eight(EightBitRegister::C, value);

                2
            }

            // LD C, A
            0x4F => {
                self.registers.write_eight(
                    EightBitRegister::C,
                    self.registers.read_eight(EightBitRegister::A),
                );

                1
            }

            // LD D, B
            0x50 => {
                self.registers.write_eight(
                    EightBitRegister::D,
                    self.registers.read_eight(EightBitRegister::B),
                );

                1
            }

            // LD D, C
            0x51 => {
                self.registers.write_eight(
                    EightBitRegister::D,
                    self.registers.read_eight(EightBitRegister::C),
                );

                1
            }

            // LD D, D
            0x52 => {
                self.registers.write_eight(
                    EightBitRegister::D,
                    self.registers.read_eight(EightBitRegister::D),
                );

                1
            }

            // LD D, E
            0x53 => {
                self.registers.write_eight(
                    EightBitRegister::D,
                    self.registers.read_eight(EightBitRegister::E),
                );

                1
            }

            // LD D, H
            0x54 => {
                self.registers.write_eight(
                    EightBitRegister::D,
                    self.registers.read_eight(EightBitRegister::H),
                );

                1
            }

            // LD D, L
            0x55 => {
                self.registers.write_eight(
                    EightBitRegister::D,
                    self.registers.read_eight(EightBitRegister::L),
                );

                1
            }

            // LD D, (HL)
            0x56 => {
                let addr = self.registers.read_sixteen(SixteenBitRegister::HL);
                let value = self.mmu.read_u8(addr);

                self.registers.write_eight(EightBitRegister::D, value);

                2
            }

            // LD D, A
            0x57 => {
                self.registers.write_eight(
                    EightBitRegister::D,
                    self.registers.read_eight(EightBitRegister::A),
                );

                1
            }

            // LD E, B
            0x58 => {
                self.registers.write_eight(
                    EightBitRegister::E,
                    self.registers.read_eight(EightBitRegister::B),
                );

                1
            }

            // LD E, C
            0x59 => {
                self.registers.write_eight(
                    EightBitRegister::E,
                    self.registers.read_eight(EightBitRegister::C),
                );

                1
            }

            // LD E, D
            0x5A => {
                self.registers.write_eight(
                    EightBitRegister::E,
                    self.registers.read_eight(EightBitRegister::D),
                );

                1
            }

            // LD E, E
            0x5B => {
                self.registers.write_eight(
                    EightBitRegister::E,
                    self.registers.read_eight(EightBitRegister::E),
                );

                1
            }

            // LD E, H
            0x5C => {
                self.registers.write_eight(
                    EightBitRegister::E,
                    self.registers.read_eight(EightBitRegister::H),
                );

                1
            }

            // LD E, L
            0x5D => {
                self.registers.write_eight(
                    EightBitRegister::E,
                    self.registers.read_eight(EightBitRegister::L),
                );

                1
            }

            // LD E, (HL)
            0x5E => {
                let addr = self.registers.read_sixteen(SixteenBitRegister::HL);
                let value = self.mmu.read_u8(addr);

                self.registers.write_eight(EightBitRegister::E, value);

                2
            }

            // LD E, A
            0x5F => {
                self.registers.write_eight(
                    EightBitRegister::E,
                    self.registers.read_eight(EightBitRegister::A),
                );

                1
            }

            // LD H, B
            0x60 => {
                self.registers.write_eight(
                    EightBitRegister::H,
                    self.registers.read_eight(EightBitRegister::B),
                );

                1
            }

            // LD H, C
            0x61 => {
                self.registers.write_eight(
                    EightBitRegister::H,
                    self.registers.read_eight(EightBitRegister::C),
                );

                1
            }

            // LD H, D
            0x62 => {
                self.registers.write_eight(
                    EightBitRegister::H,
                    self.registers.read_eight(EightBitRegister::D),
                );

                1
            }

            // LD H, E
            0x63 => {
                self.registers.write_eight(
                    EightBitRegister::H,
                    self.registers.read_eight(EightBitRegister::E),
                );

                1
            }

            // LD H, H
            0x64 => {
                self.registers.write_eight(
                    EightBitRegister::H,
                    self.registers.read_eight(EightBitRegister::H),
                );

                1
            }

            // LD H, L
            0x65 => {
                self.registers.write_eight(
                    EightBitRegister::H,
                    self.registers.read_eight(EightBitRegister::L),
                );

                1
            }

            // LD H, (HL)
            0x66 => {
                let addr = self.registers.read_sixteen(SixteenBitRegister::HL);
                let value = self.mmu.read_u8(addr);

                self.registers.write_eight(EightBitRegister::H, value);

                2
            }

            // LD H, A
            0x67 => {
                self.registers.write_eight(
                    EightBitRegister::H,
                    self.registers.read_eight(EightBitRegister::A),
                );

                1
            }

            // LD L, B
            0x68 => {
                self.registers.write_eight(
                    EightBitRegister::L,
                    self.registers.read_eight(EightBitRegister::B),
                );

                1
            }

            // LD L, C
            0x69 => {
                self.registers.write_eight(
                    EightBitRegister::L,
                    self.registers.read_eight(EightBitRegister::C),
                );

                1
            }

            // LD L, D
            0x6A => {
                self.registers.write_eight(
                    EightBitRegister::L,
                    self.registers.read_eight(EightBitRegister::D),
                );

                1
            }

            // LD L, E
            0x6B => {
                self.registers.write_eight(
                    EightBitRegister::L,
                    self.registers.read_eight(EightBitRegister::E),
                );

                1
            }

            // LD L, H
            0x6C => {
                self.registers.write_eight(
                    EightBitRegister::L,
                    self.registers.read_eight(EightBitRegister::H),
                );

                1
            }

            // LD L, L
            0x6D => {
                self.registers.write_eight(
                    EightBitRegister::L,
                    self.registers.read_eight(EightBitRegister::L),
                );

                1
            }

            // LD L, (HL)
            0x6E => {
                let addr = self.registers.read_sixteen(SixteenBitRegister::HL);
                let value = self.mmu.read_u8(addr);

                self.registers.write_eight(EightBitRegister::L, value);

                2
            }

            // LD L, A
            0x6F => {
                self.registers.write_eight(
                    EightBitRegister::L,
                    self.registers.read_eight(EightBitRegister::A),
                );

                1
            }

            // LD (HL), B
            0x70 => {
                let addr = self.registers.read_sixteen(SixteenBitRegister::HL);
                let value = self.registers.read_eight(EightBitRegister::B);

                self.mmu.write_u8(addr, value);

                2
            }

            // LD (HL), C
            0x71 => {
                let addr = self.registers.read_sixteen(SixteenBitRegister::HL);
                let value = self.registers.read_eight(EightBitRegister::C);

                self.mmu.write_u8(addr, value);

                2
            }

            // LD (HL), D
            0x72 => {
                let addr = self.registers.read_sixteen(SixteenBitRegister::HL);
                let value = self.registers.read_eight(EightBitRegister::D);

                self.mmu.write_u8(addr, value);

                2
            }

            // LD (HL), E
            0x73 => {
                let addr = self.registers.read_sixteen(SixteenBitRegister::HL);
                let value = self.registers.read_eight(EightBitRegister::E);

                self.mmu.write_u8(addr, value);

                2
            }

            // LD (HL), H
            0x74 => {
                let addr = self.registers.read_sixteen(SixteenBitRegister::HL);
                let value = self.registers.read_eight(EightBitRegister::H);

                self.mmu.write_u8(addr, value);

                2
            }

            // LD (HL), L
            0x75 => {
                let addr = self.registers.read_sixteen(SixteenBitRegister::HL);
                let value = self.registers.read_eight(EightBitRegister::L);

                self.mmu.write_u8(addr, value);

                2
            }

            // HALT
            0x76 => {
                self.halted = true;

                1
            }

            // LD (HL), A
            0x77 => {
                let addr = self.registers.read_sixteen(SixteenBitRegister::HL);
                let value = self.registers.read_eight(EightBitRegister::A);

                self.mmu.write_u8(addr, value);

                2
            }

            // LD A, B
            0x78 => {
                self.registers.write_eight(
                    EightBitRegister::A,
                    self.registers.read_eight(EightBitRegister::B),
                );

                1
            }

            // LD A, C
            0x79 => {
                self.registers.write_eight(
                    EightBitRegister::A,
                    self.registers.read_eight(EightBitRegister::C),
                );

                1
            }

            // LD A, D
            0x7A => {
                self.registers.write_eight(
                    EightBitRegister::A,
                    self.registers.read_eight(EightBitRegister::D),
                );

                1
            }

            // LD A, E
            0x7B => {
                self.registers.write_eight(
                    EightBitRegister::A,
                    self.registers.read_eight(EightBitRegister::E),
                );

                1
            }

            // LD A, H
            0x7C => {
                self.registers.write_eight(
                    EightBitRegister::A,
                    self.registers.read_eight(EightBitRegister::H),
                );

                1
            }

            // LD A, L
            0x7D => {
                self.registers.write_eight(
                    EightBitRegister::A,
                    self.registers.read_eight(EightBitRegister::L),
                );

                1
            }

            // LD A, (HL)
            0x7E => {
                let addr = self.registers.read_sixteen(SixteenBitRegister::HL);
                let value = self.mmu.read_u8(addr);

                self.registers.write_eight(EightBitRegister::A, value);

                2
            }

            // LD A, A
            0x7F => {
                self.registers.write_eight(
                    EightBitRegister::A,
                    self.registers.read_eight(EightBitRegister::A),
                );

                1
            }

            // ADD A, B
            0x80 => {
                alu_add(&mut self.registers, EightBitRegister::B);

                1
            }

            // ADD A, C
            0x81 => {
                alu_add(&mut self.registers, EightBitRegister::C);

                1
            }

            // ADD A, D
            0x82 => {
                alu_add(&mut self.registers, EightBitRegister::D);

                1
            }

            // ADD A, E
            0x83 => {
                alu_add(&mut self.registers, EightBitRegister::E);

                1
            }

            // ADD A, H
            0x84 => {
                alu_add(&mut self.registers, EightBitRegister::H);
                1
            }

            // ADD A, L
            0x85 => {
                alu_add(&mut self.registers, EightBitRegister::L);
                1
            }

            // ADD A, (HL)
            0x86 => {
                let addr = self.registers.read_sixteen(SixteenBitRegister::HL);
                let value = self.mmu.read_u8(addr);
                alu_add_value(&mut self.registers, value);
                2
            }

            // ADD A, A
            0x87 => {
                alu_add(&mut self.registers, EightBitRegister::A);
                1
            }

            // ADC A, B
            0x88 => {
                alu_adc(&mut self.registers, EightBitRegister::B);
                1
            }

            // ADC A, C
            0x89 => {
                alu_adc(&mut self.registers, EightBitRegister::C);
                1
            }

            // ADC A, D
            0x8A => {
                alu_adc(&mut self.registers, EightBitRegister::D);
                1
            }

            // ADC A, E
            0x8B => {
                alu_adc(&mut self.registers, EightBitRegister::E);
                1
            }

            // ADC A, H
            0x8C => {
                alu_adc(&mut self.registers, EightBitRegister::H);
                1
            }

            // ADC A, L
            0x8D => {
                alu_adc(&mut self.registers, EightBitRegister::L);
                1
            }

            // ADC A, (HL)
            0x8E => {
                let addr = self.registers.read_sixteen(SixteenBitRegister::HL);
                let value = self.mmu.read_u8(addr);
                alu_adc_value(&mut self.registers, value);
                2
            }

            // ADC A, A
            0x8F => {
                alu_adc(&mut self.registers, EightBitRegister::A);
                1
            }

            // SUB B
            0x90 => {
                alu_sub(&mut self.registers, EightBitRegister::B);
                1
            }

            // SUB C
            0x91 => {
                alu_sub(&mut self.registers, EightBitRegister::C);
                1
            }

            // SUB D
            0x92 => {
                alu_sub(&mut self.registers, EightBitRegister::D);
                1
            }

            // SUB E
            0x93 => {
                alu_sub(&mut self.registers, EightBitRegister::E);
                1
            }

            // SUB H
            0x94 => {
                alu_sub(&mut self.registers, EightBitRegister::H);
                1
            }

            // SUB L
            0x95 => {
                alu_sub(&mut self.registers, EightBitRegister::L);
                1
            }

            // SUB (HL)
            0x96 => {
                let addr = self.registers.read_sixteen(SixteenBitRegister::HL);
                let value = self.mmu.read_u8(addr);
                alu_sub_value(&mut self.registers, value);
                2
            }

            // SUB A
            0x97 => {
                alu_sub(&mut self.registers, EightBitRegister::A);
                1
            }

            // SBC A, B
            0x98 => {
                alu_sbc(&mut self.registers, EightBitRegister::B);
                1
            }

            // SBC A, C
            0x99 => {
                alu_sbc(&mut self.registers, EightBitRegister::C);
                1
            }

            // SBC A, D
            0x9A => {
                alu_sbc(&mut self.registers, EightBitRegister::D);
                1
            }

            // SBC A, E
            0x9B => {
                alu_sbc(&mut self.registers, EightBitRegister::E);
                1
            }

            // SBC A, H
            0x9C => {
                alu_sbc(&mut self.registers, EightBitRegister::H);
                1
            }

            // SBC A, L
            0x9D => {
                alu_sbc(&mut self.registers, EightBitRegister::L);
                1
            }

            // SBC A, (HL)
            0x9E => {
                let addr = self.registers.read_sixteen(SixteenBitRegister::HL);
                let value = self.mmu.read_u8(addr);
                alu_sbc_value(&mut self.registers, value);
                2
            }

            // SBC A, A
            0x9F => {
                alu_sbc(&mut self.registers, EightBitRegister::A);
                1
            }

            // AND B
            0xA0 => {
                alu_and(&mut self.registers, EightBitRegister::B);
                1
            }

            // AND C
            0xA1 => {
                alu_and(&mut self.registers, EightBitRegister::C);
                1
            }

            // AND D
            0xA2 => {
                alu_and(&mut self.registers, EightBitRegister::D);
                1
            }

            // AND E
            0xA3 => {
                alu_and(&mut self.registers, EightBitRegister::E);
                1
            }

            // AND H
            0xA4 => {
                alu_and(&mut self.registers, EightBitRegister::H);
                1
            }

            // AND L
            0xA5 => {
                alu_and(&mut self.registers, EightBitRegister::L);
                1
            }

            // AND (HL)
            0xA6 => {
                let addr = self.registers.read_sixteen(SixteenBitRegister::HL);
                let value = self.mmu.read_u8(addr);
                alu_and_value(&mut self.registers, value);
                2
            }

            // AND A
            0xA7 => {
                alu_and(&mut self.registers, EightBitRegister::A);
                1
            }

            // XOR B
            0xA8 => {
                alu_xor(&mut self.registers, EightBitRegister::B);
                1
            }

            // XOR C
            0xA9 => {
                alu_xor(&mut self.registers, EightBitRegister::C);
                1
            }

            // XOR D
            0xAA => {
                alu_xor(&mut self.registers, EightBitRegister::D);
                1
            }

            // XOR E
            0xAB => {
                alu_xor(&mut self.registers, EightBitRegister::E);
                1
            }

            // XOR H
            0xAC => {
                alu_xor(&mut self.registers, EightBitRegister::H);
                1
            }

            // XOR L
            0xAD => {
                alu_xor(&mut self.registers, EightBitRegister::L);
                1
            }

            // XOR (HL)
            0xAE => {
                let addr = self.registers.read_sixteen(SixteenBitRegister::HL);
                let value = self.mmu.read_u8(addr);
                alu_xor_value(&mut self.registers, value);
                2
            }

            // XOR A
            0xAF => {
                alu_xor(&mut self.registers, EightBitRegister::A);
                1
            }

            // OR B
            0xB0 => {
                alu_or(&mut self.registers, EightBitRegister::B);
                1
            }

            // OR C
            0xB1 => {
                alu_or(&mut self.registers, EightBitRegister::C);
                1
            }

            // OR D
            0xB2 => {
                alu_or(&mut self.registers, EightBitRegister::D);
                1
            }

            // OR E
            0xB3 => {
                alu_or(&mut self.registers, EightBitRegister::E);
                1
            }

            // OR H
            0xB4 => {
                alu_or(&mut self.registers, EightBitRegister::H);
                1
            }

            // OR L
            0xB5 => {
                alu_or(&mut self.registers, EightBitRegister::L);
                1
            }

            // OR (HL)
            0xB6 => {
                let addr = self.registers.read_sixteen(SixteenBitRegister::HL);
                let value = self.mmu.read_u8(addr);
                alu_or_value(&mut self.registers, value);
                2
            }

            // OR A
            0xB7 => {
                alu_or(&mut self.registers, EightBitRegister::A);
                1
            }

            // CP B
            0xB8 => {
                alu_cp(&mut self.registers, EightBitRegister::B);
                1
            }

            // CP C
            0xB9 => {
                alu_cp(&mut self.registers, EightBitRegister::C);
                1
            }

            // CP D
            0xBA => {
                alu_cp(&mut self.registers, EightBitRegister::D);
                1
            }

            // CP E
            0xBB => {
                alu_cp(&mut self.registers, EightBitRegister::E);
                1
            }

            // CP H
            0xBC => {
                alu_cp(&mut self.registers, EightBitRegister::H);
                1
            }

            // CP L
            0xBD => {
                alu_cp(&mut self.registers, EightBitRegister::L);
                1
            }

            // CP (HL)
            0xBE => {
                let addr = self.registers.read_sixteen(SixteenBitRegister::HL);
                let value = self.mmu.read_u8(addr);
                alu_cp_value(&mut self.registers, value);
                2
            }

            // CP A
            0xBF => {
                alu_cp(&mut self.registers, EightBitRegister::A);
                1
            }

            // RET NZ
            0xC0 => match ret_nz(self) {
                ReturnResult::Returned => 5,
                ReturnResult::DidNotReturn => 2,
            },

            // POP BB
            0xC1 => {
                let value = stack_pop(self);
                self.registers.write_sixteen(SixteenBitRegister::BC, value);
                3
            }

            // JP NZ, a16
            0xC2 => {
                let value = self.fetch_u16();
                match jp_nz(&mut self.registers, value) {
                    JumpResult::Jumped => 4,
                    JumpResult::DidNotJump => 3,
                }
            }

            // JP a16
            0xC3 => {
                let value = self.fetch_u16();
                jp(&mut self.registers, value);
                4
            }

            // CALL NZ, a16
            0xC4 => {
                let value = self.fetch_u16();
                match call_nz(self, value) {
                    CallResult::Called => 6,
                    CallResult::DidNotCall => 3,
                }
            }

            // PUSH BC
            0xC5 => {
                let value = self.registers.read_sixteen(SixteenBitRegister::BC);
                stack_push(self, value);
                4
            }

            // ADD A, d8
            0xC6 => {
                let value = self.fetch_u8();
                alu_add_value(&mut self.registers, value);
                2
            }

            // RST 00H
            0xC7 => {
                call(self, 0x00);
                4
            }

            // RET Z
            0xC8 => match ret_z(self) {
                ReturnResult::Returned => 5,
                ReturnResult::DidNotReturn => 2,
            },

            // RET
            0xC9 => {
                ret(self);
                4
            }

            // JP Z, a16
            0xCA => {
                let value = self.fetch_u16();
                match jp_z(&mut self.registers, value) {
                    JumpResult::Jumped => 4,
                    JumpResult::DidNotJump => 3,
                }
            }

            // PREFIX CB
            0xCB => {
                let opcode = self.fetch_u8();
                cb_instructions::handle_cb_instructions(self, opcode);

                2
            }

            // CALL Z, a16
            0xCC => {
                let value = self.fetch_u16();
                match call_z(self, value) {
                    CallResult::Called => 6,
                    CallResult::DidNotCall => 3,
                }
            }

            // CALL a16
            0xCD => {
                let value = self.fetch_u16();
                call(self, value);
                6
            }

            // ADC A, d8
            0xCE => {
                let value = self.fetch_u8();
                alu_adc_value(&mut self.registers, value);
                2
            }

            // RST 08H
            0xCF => {
                call(self, 0x08);
                4
            }

            // RET NC
            0xD0 => match ret_nc(self) {
                ReturnResult::Returned => 5,
                ReturnResult::DidNotReturn => 2,
            },

            // POP DE
            0xD1 => {
                let value = stack_pop(self);

                self.registers.write_sixteen(SixteenBitRegister::DE, value);

                3
            }

            // JP NC, a16
            0xD2 => {
                let value = self.fetch_u16();

                match jp_nc(&mut self.registers, value) {
                    JumpResult::Jumped => 4,
                    JumpResult::DidNotJump => 3,
                }
            }

            0xD3 => {
                panic!("Unknown opcode: {:#04x}", opcode);
            }

            // CALL NC, a16
            0xD4 => {
                let value = self.fetch_u16();

                match call_nc(self, value) {
                    CallResult::Called => 6,
                    CallResult::DidNotCall => 3,
                }
            }

            // PUSH DE
            0xD5 => {
                let value = self.registers.read_sixteen(SixteenBitRegister::DE);
                stack_push(self, value);

                4
            }

            // SUB d8
            0xD6 => {
                let value = self.fetch_u8();

                alu_sub_value(&mut self.registers, value);

                2
            }

            // RST 10H
            0xD7 => {
                call(self, 0x10);

                4
            }

            // RET C
            0xD8 => match ret_c(self) {
                ReturnResult::Returned => 5,
                ReturnResult::DidNotReturn => 2,
            },

            // RETI
            0xD9 => {
                ret(self);
                self.ime = true;

                4
            }

            // JP C, a16
            0xDA => {
                let value = self.fetch_u16();

                match jp_c(&mut self.registers, value) {
                    JumpResult::Jumped => 4,
                    JumpResult::DidNotJump => 3,
                }
            }

            // CALL C a16
            0xDC => {
                let value = self.fetch_u16();

                call_c(self, value);

                6
            }

            // SBC A, d8
            0xDE => {
                let value = self.fetch_u8();

                alu_sbc_value(&mut self.registers, value);

                2
            }

            // RST 18H
            0xDF => {
                call(self, 0x18);

                4
            }

            // LDH (a8), A
            0xE0 => {
                let offset = self.fetch_u8();

                let addr = 0xFF00 + offset as u16;

                let value = self.registers.read_eight(EightBitRegister::A);

                self.mmu.write_u8(addr, value);

                3
            }

            // POP HL
            0xE1 => {
                let value = stack_pop(self);

                self.registers.write_sixteen(SixteenBitRegister::HL, value);

                3
            }

            // LD (C), A
            0xE2 => {
                let offset = self.registers.read_eight(EightBitRegister::C) as u16;
                let addr = 0xFF00 + offset;
                let value = self.registers.read_eight(EightBitRegister::A);

                self.mmu.write_u8(addr, value);

                2
            }

            // PUSH HL
            0xE5 => {
                let value = self.registers.read_sixteen(SixteenBitRegister::HL);
                stack_push(self, value);

                4
            }

            // AND d8
            0xE6 => {
                let value = self.fetch_u8();

                alu_and_value(&mut self.registers, value);

                2
            }

            // RST 20H
            0xE7 => {
                call(self, 0x20);

                4
            }

            // ADD SP, r8
            0xE8 => {
                let value = self.fetch_u8();
                add_sp_r8(&mut self.registers, value);

                4
            }

            // JP (HL)
            0xE9 => {
                let addr = self.registers.read_sixteen(SixteenBitRegister::HL);

                jp(&mut self.registers, addr);

                1
            }

            // LD (a16), A
            0xEA => {
                let addr = self.fetch_u16();

                let value = self.registers.read_eight(EightBitRegister::A);

                self.mmu.write_u8(addr, value);

                4
            }

            // XOR d8
            0xEE => {
                let value = self.fetch_u8();

                alu_xor_value(&mut self.registers, value);

                2
            }

            // RST 28H
            0xEF => {
                call(self, 0x28);

                4
            }

            // LDH A, (a8)
            0xF0 => {
                let offset = self.fetch_u8() as u16;

                let addr = offset + 0xFF00;
                let value = self.mmu.read_u8(addr);

                self.registers.write_eight(EightBitRegister::A, value);

                2
            }

            // POP AF
            0xF1 => {
                // Why?
                let value = stack_pop(self) & 0xFFF0;
                self.registers.write_sixteen(SixteenBitRegister::AF, value);

                3
            }

            // LD A, (C)
            0xF2 => {
                let offset = self.registers.read_eight(EightBitRegister::C) as u16;
                let addr = 0xFF00 + offset;
                let value = self.mmu.read_u8(addr);

                self.registers.write_eight(EightBitRegister::A, value);

                2
            }

            // DI
            0xF3 => {
                self.ime = false;

                1
            }

            // PUSH AF
            0xF5 => {
                let value = self.registers.read_sixteen(SixteenBitRegister::AF);
                stack_push(self, value);

                4
            }

            // OR d8
            0xF6 => {
                let value = self.fetch_u8();

                alu_or_value(&mut self.registers, value);

                2
            }

            // RST 30H
            0xF7 => {
                call(self, 0x30);

                4
            }

            // LD HP, SP + e8
            0xF8 => {
                let value = self.fetch_u8();
                ld_hl_sp_r8(&mut self.registers, value);

                3
            }

            // LD SP, HL
            0xF9 => {
                let value = self.registers.read_sixteen(SixteenBitRegister::HL);

                self.registers.write_sixteen(SixteenBitRegister::SP, value);

                2
            }

            // LD A, [n16]
            0xFA => {
                let addr = self.fetch_u16();
                let value = self.mmu.read_u8(addr);

                self.registers.write_eight(EightBitRegister::A, value);

                4
            }

            // EI
            0xFB => {
                self.ime = true;

                1
            }

            // CP A, n8
            0xFE => {
                let value = self.fetch_u8();

                alu_cp_value(&mut self.registers, value);

                2
            }

            // RST 38H
            0xFF => {
                call(self, 0x38);

                4
            }

            //
            _ => {
                panic!("Unknown opcode: {:#04x}", opcode);
            }
        }
    }
}
