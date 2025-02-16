use super::*;
use bitwise_operations::*;
use shift_operations::*;

pub fn handle_cb_instructions(cpu: &mut CPU, opcode: u8) -> u8 {
    match opcode {
        // RLC B
        0x00 => {
            rlc(&mut cpu.registers, EightBitRegister::B);

            2
        }
        // RLC C
        0x01 => {
            rlc(&mut cpu.registers, EightBitRegister::C);

            2
        }
        // RLC D
        0x02 => {
            rlc(&mut cpu.registers, EightBitRegister::D);

            2
        }
        // RLC E
        0x03 => {
            rlc(&mut cpu.registers, EightBitRegister::E);

            2
        }
        // RLC H
        0x04 => {
            rlc(&mut cpu.registers, EightBitRegister::H);

            2
        }
        // RLC L
        0x05 => {
            rlc(&mut cpu.registers, EightBitRegister::L);

            2
        }
        // RLC (HL)
        0x06 => {
            let address = cpu.registers.read_sixteen(SixteenBitRegister::HL);
            let value = cpu.mmu.read_u8(address);
            let result = rlc_val(&mut cpu.registers, value);
            cpu.mmu.write_u8(address, result);

            4
        }
        // RLC A
        0x07 => {
            rlc(&mut cpu.registers, EightBitRegister::A);

            2
        }
        // RRC B
        0x08 => {
            rrc(&mut cpu.registers, EightBitRegister::B);

            2
        }
        // RRC C
        0x09 => {
            rrc(&mut cpu.registers, EightBitRegister::C);

            2
        }
        // RRC D
        0x0A => {
            rrc(&mut cpu.registers, EightBitRegister::D);

            2
        }
        // RRC E
        0x0B => {
            rrc(&mut cpu.registers, EightBitRegister::E);

            2
        }
        // RRC H
        0x0C => {
            rrc(&mut cpu.registers, EightBitRegister::H);

            2
        }
        // RRC L
        0x0D => {
            rrc(&mut cpu.registers, EightBitRegister::L);

            2
        }
        // RRC (HL)
        0x0E => {
            let address = cpu.registers.read_sixteen(SixteenBitRegister::HL);
            let value = cpu.mmu.read_u8(address);
            let result = rrc_val(&mut cpu.registers, value);
            cpu.mmu.write_u8(address, result);

            4
        }
        // RRC A
        0x0F => {
            rrc(&mut cpu.registers, EightBitRegister::A);

            2
        }
        // RL B
        0x10 => {
            rl(&mut cpu.registers, EightBitRegister::B);

            2
        }
        // RL C
        0x11 => {
            rl(&mut cpu.registers, EightBitRegister::C);

            2
        }
        // RL D
        0x12 => {
            rl(&mut cpu.registers, EightBitRegister::D);

            2
        }
        // RL E
        0x13 => {
            rl(&mut cpu.registers, EightBitRegister::E);

            2
        }
        // RL H
        0x14 => {
            rl(&mut cpu.registers, EightBitRegister::H);

            2
        }
        // RL L
        0x15 => {
            rl(&mut cpu.registers, EightBitRegister::L);

            2
        }
        // RL (HL)
        0x16 => {
            let address = cpu.registers.read_sixteen(SixteenBitRegister::HL);
            let value = cpu.mmu.read_u8(address);
            let result = rl_val(&mut cpu.registers, value);
            cpu.mmu.write_u8(address, result);

            4
        }
        // RL A
        0x17 => {
            rl(&mut cpu.registers, EightBitRegister::A);

            2
        }
        // RR B
        0x18 => {
            rr(&mut cpu.registers, EightBitRegister::B);

            2
        }
        // RR C
        0x19 => {
            rr(&mut cpu.registers, EightBitRegister::C);

            2
        }
        // RR D
        0x1A => {
            rr(&mut cpu.registers, EightBitRegister::D);

            2
        }
        // RR E
        0x1B => {
            rr(&mut cpu.registers, EightBitRegister::E);

            2
        }
        // RR H
        0x1C => {
            rr(&mut cpu.registers, EightBitRegister::H);

            2
        }
        // RR L
        0x1D => {
            rr(&mut cpu.registers, EightBitRegister::L);

            2
        }
        // RR (HL)
        0x1E => {
            let address = cpu.registers.read_sixteen(SixteenBitRegister::HL);
            let value = cpu.mmu.read_u8(address);
            let result = rr_val(&mut cpu.registers, value);
            cpu.mmu.write_u8(address, result);

            4
        }
        // RR A
        0x1F => {
            rr(&mut cpu.registers, EightBitRegister::A);

            2
        }
        // SLA B
        0x20 => {
            sla(&mut cpu.registers, EightBitRegister::B);

            2
        }
        // SLA C
        0x21 => {
            sla(&mut cpu.registers, EightBitRegister::C);

            2
        }
        // SLA D
        0x22 => {
            sla(&mut cpu.registers, EightBitRegister::D);

            2
        }
        // SLA E
        0x23 => {
            sla(&mut cpu.registers, EightBitRegister::E);

            2
        }
        // SLA H
        0x24 => {
            sla(&mut cpu.registers, EightBitRegister::H);

            2
        }
        // SLA L
        0x25 => {
            sla(&mut cpu.registers, EightBitRegister::L);

            2
        }
        // SLA (HL)
        0x26 => {
            let address = cpu.registers.read_sixteen(SixteenBitRegister::HL);
            let value = cpu.mmu.read_u8(address);
            let result = sla_val(&mut cpu.registers, value);
            cpu.mmu.write_u8(address, result);

            4
        }
        // SLA A
        0x27 => {
            sla(&mut cpu.registers, EightBitRegister::A);

            2
        }
        // SRA B
        0x28 => {
            sra(&mut cpu.registers, EightBitRegister::B);

            2
        }
        // SRA C
        0x29 => {
            sra(&mut cpu.registers, EightBitRegister::C);

            2
        }
        // SRA D
        0x2A => {
            sra(&mut cpu.registers, EightBitRegister::D);

            2
        }
        // SRA E
        0x2B => {
            sra(&mut cpu.registers, EightBitRegister::E);

            2
        }
        // SRA H
        0x2C => {
            sra(&mut cpu.registers, EightBitRegister::H);

            2
        }
        // SRA L
        0x2D => {
            sra(&mut cpu.registers, EightBitRegister::L);

            2
        }
        // SRA (HL)
        0x2E => {
            let address = cpu.registers.read_sixteen(SixteenBitRegister::HL);
            let value = cpu.mmu.read_u8(address);
            let result = sra_val(&mut cpu.registers, value);
            cpu.mmu.write_u8(address, result);

            4
        }
        // SRA A
        0x2F => {
            sra(&mut cpu.registers, EightBitRegister::A);

            2
        }
        // SWAP B
        0x30 => {
            swap(&mut cpu.registers, EightBitRegister::B);

            2
        }
        // SWAP C
        0x31 => {
            swap(&mut cpu.registers, EightBitRegister::C);

            2
        }
        // SWAP D
        0x32 => {
            swap(&mut cpu.registers, EightBitRegister::D);

            2
        }
        // SWAP E
        0x33 => {
            swap(&mut cpu.registers, EightBitRegister::E);

            2
        }
        // SWAP H
        0x34 => {
            swap(&mut cpu.registers, EightBitRegister::H);

            2
        }
        // SWAP L
        0x35 => {
            swap(&mut cpu.registers, EightBitRegister::L);

            2
        }
        // SWAP (HL)
        0x36 => {
            let address = cpu.registers.read_sixteen(SixteenBitRegister::HL);
            let value = cpu.mmu.read_u8(address);
            let result = swap_value(&mut cpu.registers, value);
            cpu.mmu.write_u8(address, result);

            4
        }
        // SWAP A
        0x37 => {
            swap(&mut cpu.registers, EightBitRegister::A);

            2
        }
        // SRL B
        0x38 => {
            srl(&mut cpu.registers, EightBitRegister::B);

            2
        }

        // SRL C
        0x39 => {
            srl(&mut cpu.registers, EightBitRegister::C);

            2
        }

        // SRL D
        0x3A => {
            srl(&mut cpu.registers, EightBitRegister::D);

            2
        }

        // SRL E
        0x3B => {
            srl(&mut cpu.registers, EightBitRegister::E);

            2
        }

        // SRL H
        0x3C => {
            srl(&mut cpu.registers, EightBitRegister::H);

            2
        }

        // SRL L
        0x3D => {
            srl(&mut cpu.registers, EightBitRegister::L);

            2
        }
        // SRL (HL)
        0x3E => {
            let address = cpu.registers.read_sixteen(SixteenBitRegister::HL);
            let value = cpu.mmu.read_u8(address);
            let result = srl_val(&mut cpu.registers, value);
            cpu.mmu.write_u8(address, result);

            4
        }
        // SRL A
        0x3F => {
            srl(&mut cpu.registers, EightBitRegister::A);

            2
        }
        // BIT 0, B
        0x40 => {
            bit(&mut cpu.registers, EightBitRegister::B, 0);

            2
        }
        // BIT 0, C
        0x41 => {
            bit(&mut cpu.registers, EightBitRegister::C, 0);

            2
        }
        // BIT 0, D
        0x42 => {
            bit(&mut cpu.registers, EightBitRegister::D, 0);

            2
        }
        // BIT 0, E
        0x43 => {
            bit(&mut cpu.registers, EightBitRegister::E, 0);

            2
        }
        // BIT 0, H
        0x44 => {
            bit(&mut cpu.registers, EightBitRegister::H, 0);

            2
        }
        // BIT 0, L
        0x45 => {
            bit(&mut cpu.registers, EightBitRegister::L, 0);

            2
        }
        // BIT 0, (HL)
        0x46 => {
            let address = cpu.registers.read_sixteen(SixteenBitRegister::HL);
            let value = cpu.mmu.read_u8(address);
            bit_val(&mut cpu.registers, value, 0);

            3
        }
        // BIT 0, A
        0x47 => {
            bit(&mut cpu.registers, EightBitRegister::A, 0);

            2
        }
        // BIT 1, B
        0x48 => {
            bit(&mut cpu.registers, EightBitRegister::B, 1);

            2
        }
        // BIT 1, C
        0x49 => {
            bit(&mut cpu.registers, EightBitRegister::C, 1);

            2
        }
        // BIT 1, D
        0x4A => {
            bit(&mut cpu.registers, EightBitRegister::D, 1);

            2
        }
        // BIT 1, E
        0x4B => {
            bit(&mut cpu.registers, EightBitRegister::E, 1);

            2
        }
        // BIT 1, H
        0x4C => {
            bit(&mut cpu.registers, EightBitRegister::H, 1);

            2
        }
        // BIT 1, L
        0x4D => {
            bit(&mut cpu.registers, EightBitRegister::L, 1);

            2
        }
        // BIT 1, (HL)
        0x4E => {
            let address = cpu.registers.read_sixteen(SixteenBitRegister::HL);
            let value = cpu.mmu.read_u8(address);
            bit_val(&mut cpu.registers, value, 1);

            3
        }
        // BIT 1, A
        0x4F => {
            bit(&mut cpu.registers, EightBitRegister::A, 1);

            2
        }
        // BIT 2, B
        0x50 => {
            bit(&mut cpu.registers, EightBitRegister::B, 2);

            2
        }
        // BIT 2, C
        0x51 => {
            bit(&mut cpu.registers, EightBitRegister::C, 2);

            2
        }
        // BIT 2, D
        0x52 => {
            bit(&mut cpu.registers, EightBitRegister::D, 2);

            2
        }
        // BIT 2, E
        0x53 => {
            bit(&mut cpu.registers, EightBitRegister::E, 2);

            2
        }
        // BIT 2, H
        0x54 => {
            bit(&mut cpu.registers, EightBitRegister::H, 2);

            2
        }
        // BIT 2, L
        0x55 => {
            bit(&mut cpu.registers, EightBitRegister::L, 2);

            2
        }
        // BIT 2, (HL)
        0x56 => {
            let address = cpu.registers.read_sixteen(SixteenBitRegister::HL);
            let value = cpu.mmu.read_u8(address);
            bit_val(&mut cpu.registers, value, 2);

            3
        }
        // BIT 2, A
        0x57 => {
            bit(&mut cpu.registers, EightBitRegister::A, 2);

            2
        }
        // BIT 3, B
        0x58 => {
            bit(&mut cpu.registers, EightBitRegister::B, 3);

            2
        }
        // BIT 3, C
        0x59 => {
            bit(&mut cpu.registers, EightBitRegister::C, 3);

            2
        }
        // BIT 3, D
        0x5a => {
            bit(&mut cpu.registers, EightBitRegister::D, 3);

            2
        }
        // BIT 3, E
        0x5b => {
            bit(&mut cpu.registers, EightBitRegister::E, 3);

            2
        }
        // BIT 3, H
        0x5c => {
            bit(&mut cpu.registers, EightBitRegister::H, 3);

            2
        }
        // BIT 3, L
        0x5d => {
            bit(&mut cpu.registers, EightBitRegister::L, 3);

            2
        }
        // BIT 3, (HL)
        0x5e => {
            let address = cpu.registers.read_sixteen(SixteenBitRegister::HL);
            let value = cpu.mmu.read_u8(address);
            bit_val(&mut cpu.registers, value, 3);

            3
        }
        // BIT 3, A
        0x5f => {
            bit(&mut cpu.registers, EightBitRegister::A, 3);

            2
        }
        // BIT 4, B
        0x60 => {
            bit(&mut cpu.registers, EightBitRegister::B, 4);

            2
        }
        // BIT 4, C
        0x61 => {
            bit(&mut cpu.registers, EightBitRegister::C, 4);

            2
        }
        // BIT 4, D
        0x62 => {
            bit(&mut cpu.registers, EightBitRegister::D, 4);

            2
        }
        // BIT 4, E
        0x63 => {
            bit(&mut cpu.registers, EightBitRegister::E, 4);

            2
        }
        // BIT 4, H
        0x64 => {
            bit(&mut cpu.registers, EightBitRegister::H, 4);

            2
        }
        // BIT 4, L
        0x65 => {
            bit(&mut cpu.registers, EightBitRegister::L, 4);

            2
        }
        // BIT 4, (HL)
        0x66 => {
            let address = cpu.registers.read_sixteen(SixteenBitRegister::HL);
            let value = cpu.mmu.read_u8(address);
            bit_val(&mut cpu.registers, value, 4);

            3
        }
        // BIT 4, A
        0x67 => {
            bit(&mut cpu.registers, EightBitRegister::A, 4);

            2
        }
        // BIT 5, B
        0x68 => {
            bit(&mut cpu.registers, EightBitRegister::B, 5);

            2
        }
        // BIT 5, C
        0x69 => {
            bit(&mut cpu.registers, EightBitRegister::C, 5);

            2
        }
        // BIT 5, D
        0x6a => {
            bit(&mut cpu.registers, EightBitRegister::D, 5);

            2
        }
        // BIT 5, E
        0x6b => {
            bit(&mut cpu.registers, EightBitRegister::E, 5);

            2
        }
        // BIT 5, H
        0x6c => {
            bit(&mut cpu.registers, EightBitRegister::H, 5);

            2
        }
        // BIT 5, L
        0x6d => {
            bit(&mut cpu.registers, EightBitRegister::L, 5);

            2
        }
        // BIT 5, (HL)
        0x6e => {
            let address = cpu.registers.read_sixteen(SixteenBitRegister::HL);
            let value = cpu.mmu.read_u8(address);
            bit_val(&mut cpu.registers, value, 5);

            3
        }
        // BIT 5, A
        0x6f => {
            bit(&mut cpu.registers, EightBitRegister::A, 5);

            2
        }
        // BIT 6, B
        0x70 => {
            bit(&mut cpu.registers, EightBitRegister::B, 6);

            2
        }
        // BIT 6, C
        0x71 => {
            bit(&mut cpu.registers, EightBitRegister::C, 6);

            2
        }
        // BIT 6, D
        0x72 => {
            bit(&mut cpu.registers, EightBitRegister::D, 6);

            2
        }
        // BIT 6, E
        0x73 => {
            bit(&mut cpu.registers, EightBitRegister::E, 6);

            2
        }
        // BIT 6, H
        0x74 => {
            bit(&mut cpu.registers, EightBitRegister::H, 6);

            2
        }
        // BIT 6, L
        0x75 => {
            bit(&mut cpu.registers, EightBitRegister::L, 6);

            2
        }
        // BIT 6, (HL)
        0x76 => {
            let address = cpu.registers.read_sixteen(SixteenBitRegister::HL);
            let value = cpu.mmu.read_u8(address);
            bit_val(&mut cpu.registers, value, 6);

            3
        }
        // BIT 6, A
        0x77 => {
            bit(&mut cpu.registers, EightBitRegister::A, 6);

            2
        }
        // BIT 7, B
        0x78 => {
            bit(&mut cpu.registers, EightBitRegister::B, 7);

            2
        }
        // BIT 7, C
        0x79 => {
            bit(&mut cpu.registers, EightBitRegister::C, 7);

            2
        }
        // BIT 7, D
        0x7a => {
            bit(&mut cpu.registers, EightBitRegister::D, 7);

            2
        }
        // BIT 7, E
        0x7b => {
            bit(&mut cpu.registers, EightBitRegister::E, 7);

            2
        }
        // BIT 7, H
        0x7c => {
            bit(&mut cpu.registers, EightBitRegister::H, 7);

            2
        }
        // BIT 7, L
        0x7d => {
            bit(&mut cpu.registers, EightBitRegister::L, 7);

            2
        }
        // BIT 7, (HL)
        0x7e => {
            let address = cpu.registers.read_sixteen(SixteenBitRegister::HL);
            let value = cpu.mmu.read_u8(address);
            bit_val(&mut cpu.registers, value, 7);

            3
        }
        // BIT 7, A
        0x7f => {
            bit(&mut cpu.registers, EightBitRegister::A, 7);

            2
        }
        // RES 0, B
        0x80 => {
            res(&mut cpu.registers, EightBitRegister::B, 0);

            2
        }
        // RES 0, C
        0x81 => {
            res(&mut cpu.registers, EightBitRegister::C, 0);

            2
        }
        // RES 0, D
        0x82 => {
            res(&mut cpu.registers, EightBitRegister::D, 0);

            2
        }
        // RES 0, E
        0x83 => {
            res(&mut cpu.registers, EightBitRegister::E, 0);

            2
        }

        // RES 0, H
        0x84 => {
            res(&mut cpu.registers, EightBitRegister::H, 0);

            2
        }
        // RES 0, L
        0x85 => {
            res(&mut cpu.registers, EightBitRegister::L, 0);

            2
        }
        // RES 0, (HL)
        0x86 => {
            let address = cpu.registers.read_sixteen(SixteenBitRegister::HL);
            let value = cpu.mmu.read_u8(address);
            let result = res_val(value, 0);
            cpu.mmu.write_u8(address, result);

            4
        }
        // RES 0, A
        0x87 => {
            res(&mut cpu.registers, EightBitRegister::A, 0);

            2
        }
        // RES 1, B
        0x88 => {
            res(&mut cpu.registers, EightBitRegister::B, 1);

            2
        }
        // RES 1, C
        0x89 => {
            res(&mut cpu.registers, EightBitRegister::C, 1);

            2
        }
        // RES 1, D
        0x8A => {
            res(&mut cpu.registers, EightBitRegister::D, 1);

            2
        }
        // RES 1, E
        0x8B => {
            res(&mut cpu.registers, EightBitRegister::E, 1);

            2
        }
        // RES 1, H
        0x8C => {
            res(&mut cpu.registers, EightBitRegister::H, 1);

            2
        }
        // RES 1, L
        0x8D => {
            res(&mut cpu.registers, EightBitRegister::L, 1);

            2
        }
        // RES 1, (HL)
        0x8E => {
            let address = cpu.registers.read_sixteen(SixteenBitRegister::HL);
            let value = cpu.mmu.read_u8(address);
            let result = res_val(value, 1);
            cpu.mmu.write_u8(address, result);

            4
        }
        // RE 1, A
        0x8F => {
            res(&mut cpu.registers, EightBitRegister::A, 1);

            2
        }
        // RES 2, B
        0x90 => {
            res(&mut cpu.registers, EightBitRegister::B, 2);

            2
        }
        // RES 2, C
        0x91 => {
            res(&mut cpu.registers, EightBitRegister::C, 2);

            2
        }
        // RES 2, D
        0x92 => {
            res(&mut cpu.registers, EightBitRegister::D, 2);

            2
        }
        // RES 2, E
        0x93 => {
            res(&mut cpu.registers, EightBitRegister::E, 2);

            2
        }
        // RES 2, H
        0x94 => {
            res(&mut cpu.registers, EightBitRegister::H, 2);

            2
        }
        // RES 2, L
        0x95 => {
            res(&mut cpu.registers, EightBitRegister::L, 2);

            2
        }
        // RES 2, (HL)
        0x96 => {
            let address = cpu.registers.read_sixteen(SixteenBitRegister::HL);
            let value = cpu.mmu.read_u8(address);
            let result = res_val(value, 2);
            cpu.mmu.write_u8(address, result);

            4
        }
        // RE 2, A
        0x97 => {
            res(&mut cpu.registers, EightBitRegister::A, 2);

            2
        }
        // RES 3, B
        0x98 => {
            res(&mut cpu.registers, EightBitRegister::B, 3);

            2
        }
        // RES 3, C
        0x99 => {
            res(&mut cpu.registers, EightBitRegister::C, 3);

            2
        }
        // RES 3, D
        0x9a => {
            res(&mut cpu.registers, EightBitRegister::D, 3);

            2
        }
        // RES 3, E
        0x9b => {
            res(&mut cpu.registers, EightBitRegister::E, 3);

            2
        }
        // RES 3, H
        0x9c => {
            res(&mut cpu.registers, EightBitRegister::H, 3);

            2
        }
        // RES 3, L
        0x9d => {
            res(&mut cpu.registers, EightBitRegister::L, 3);

            2
        }
        // RES 3, (HL)
        0x9e => {
            let address = cpu.registers.read_sixteen(SixteenBitRegister::HL);
            let value = cpu.mmu.read_u8(address);
            let result = res_val(value, 3);
            cpu.mmu.write_u8(address, result);

            4
        }
        // RE 3, A
        0x9f => {
            res(&mut cpu.registers, EightBitRegister::A, 3);

            2
        }
        // RES 4, B
        0xa0 => {
            res(&mut cpu.registers, EightBitRegister::B, 4);

            2
        }
        // RES 4, C
        0xa1 => {
            res(&mut cpu.registers, EightBitRegister::C, 4);

            2
        }
        // RES 4, D
        0xa2 => {
            res(&mut cpu.registers, EightBitRegister::D, 4);

            2
        }
        // RES 4, E
        0xa3 => {
            res(&mut cpu.registers, EightBitRegister::E, 4);

            2
        }
        // RES 4, H
        0xa4 => {
            res(&mut cpu.registers, EightBitRegister::H, 4);

            2
        }
        // RES 4, L
        0xa5 => {
            res(&mut cpu.registers, EightBitRegister::L, 4);

            2
        }
        // RES 4, (HL)
        0xa6 => {
            let address = cpu.registers.read_sixteen(SixteenBitRegister::HL);
            let value = cpu.mmu.read_u8(address);
            let result = res_val(value, 4);
            cpu.mmu.write_u8(address, result);

            4
        }

        // RE 4, A
        0xa7 => {
            res(&mut cpu.registers, EightBitRegister::A, 4);

            2
        }
        // RES 5, B
        0xa8 => {
            res(&mut cpu.registers, EightBitRegister::B, 5);

            2
        }
        // RES 5, C
        0xa9 => {
            res(&mut cpu.registers, EightBitRegister::C, 5);

            2
        }
        // RES 5, D
        0xaa => {
            res(&mut cpu.registers, EightBitRegister::D, 5);

            2
        }
        // RES 5, E
        0xab => {
            res(&mut cpu.registers, EightBitRegister::E, 5);

            2
        }
        // RES 5, H
        0xac => {
            res(&mut cpu.registers, EightBitRegister::H, 5);

            2
        }
        // RES 5, L
        0xad => {
            res(&mut cpu.registers, EightBitRegister::L, 5);

            2
        }
        // RES 5, (HL)
        0xae => {
            let address = cpu.registers.read_sixteen(SixteenBitRegister::HL);
            let value = cpu.mmu.read_u8(address);
            let result = res_val(value, 5);
            cpu.mmu.write_u8(address, result);

            4
        }
        // RE 5, A
        0xaf => {
            res(&mut cpu.registers, EightBitRegister::A, 5);

            2
        }
        // RES 6, B
        0xb0 => {
            res(&mut cpu.registers, EightBitRegister::B, 6);

            2
        }
        // RES 6, C
        0xb1 => {
            res(&mut cpu.registers, EightBitRegister::C, 6);

            2
        }
        // RES 6, D
        0xb2 => {
            res(&mut cpu.registers, EightBitRegister::D, 6);

            2
        }
        // RES 6, E
        0xb3 => {
            res(&mut cpu.registers, EightBitRegister::E, 6);

            2
        }
        // RES 6, H
        0xb4 => {
            res(&mut cpu.registers, EightBitRegister::H, 6);

            2
        }
        // RES 6, L
        0xb5 => {
            res(&mut cpu.registers, EightBitRegister::L, 6);

            2
        }
        // RES 6, (HL)
        0xb6 => {
            let address = cpu.registers.read_sixteen(SixteenBitRegister::HL);
            let value = cpu.mmu.read_u8(address);
            let result = res_val(value, 6);
            cpu.mmu.write_u8(address, result);

            4
        }
        // RE 6, A
        0xb7 => {
            res(&mut cpu.registers, EightBitRegister::A, 6);

            2
        }
        // RES 7, B
        0xb8 => {
            res(&mut cpu.registers, EightBitRegister::B, 7);

            2
        }
        // RES 7, C
        0xb9 => {
            res(&mut cpu.registers, EightBitRegister::C, 7);

            2
        }
        // RES 7, D
        0xba => {
            res(&mut cpu.registers, EightBitRegister::D, 7);

            2
        }
        // RES 7, E
        0xbb => {
            res(&mut cpu.registers, EightBitRegister::E, 7);

            2
        }
        // RES 7, H
        0xbc => {
            res(&mut cpu.registers, EightBitRegister::H, 7);

            2
        }
        // RES 7, L
        0xbd => {
            res(&mut cpu.registers, EightBitRegister::L, 7);

            2
        }
        // RES 7, (HL)
        0xbe => {
            let address = cpu.registers.read_sixteen(SixteenBitRegister::HL);
            let value = cpu.mmu.read_u8(address);
            let result = res_val(value, 7);
            cpu.mmu.write_u8(address, result);

            4
        }
        // RE 7, A
        0xbf => {
            res(&mut cpu.registers, EightBitRegister::A, 7);

            2
        }
        // SET 0, B
        0xc0 => {
            set(&mut cpu.registers, EightBitRegister::B, 0);

            2
        }
        // SET 0, C
        0xc1 => {
            set(&mut cpu.registers, EightBitRegister::C, 0);

            2
        }
        // SET 0, D
        0xc2 => {
            set(&mut cpu.registers, EightBitRegister::D, 0);

            2
        }
        // SET 0, E
        0xc3 => {
            set(&mut cpu.registers, EightBitRegister::E, 0);

            2
        }
        // SET 0, H
        0xc4 => {
            set(&mut cpu.registers, EightBitRegister::H, 0);

            2
        }
        // SET 0, L
        0xc5 => {
            set(&mut cpu.registers, EightBitRegister::L, 0);

            2
        }
        // SET 0, (HL)
        0xc6 => {
            let address = cpu.registers.read_sixteen(SixteenBitRegister::HL);
            let value = cpu.mmu.read_u8(address);
            let result = set_val(value, 0);
            cpu.mmu.write_u8(address, result);

            4
        }
        // SET 0, A
        0xc7 => {
            set(&mut cpu.registers, EightBitRegister::A, 0);

            2
        }
        // SET 1, B
        0xc8 => {
            set(&mut cpu.registers, EightBitRegister::B, 1);

            2
        }
        // SET 1, C
        0xc9 => {
            set(&mut cpu.registers, EightBitRegister::C, 1);

            2
        }
        // SET 1, D
        0xca => {
            set(&mut cpu.registers, EightBitRegister::D, 1);

            2
        }
        // SET 1, E
        0xcb => {
            set(&mut cpu.registers, EightBitRegister::E, 1);

            2
        }
        // SET 1, H
        0xcc => {
            set(&mut cpu.registers, EightBitRegister::H, 1);

            2
        }
        // SET 1, L
        0xcd => {
            set(&mut cpu.registers, EightBitRegister::L, 1);

            2
        }
        // SET 1, (HL)
        0xce => {
            let address = cpu.registers.read_sixteen(SixteenBitRegister::HL);
            let value = cpu.mmu.read_u8(address);
            let result = set_val(value, 1);
            cpu.mmu.write_u8(address, result);

            4
        }
        // SET 1, A
        0xcf => {
            set(&mut cpu.registers, EightBitRegister::A, 1);

            2
        }
        // SET 2, B
        0xd0 => {
            set(&mut cpu.registers, EightBitRegister::B, 2);

            2
        }
        // SET 2, C
        0xd1 => {
            set(&mut cpu.registers, EightBitRegister::C, 2);

            2
        }
        // SET 2, D
        0xd2 => {
            set(&mut cpu.registers, EightBitRegister::D, 2);

            2
        }
        // SET 2, E
        0xd3 => {
            set(&mut cpu.registers, EightBitRegister::E, 2);

            2
        }
        // SET 2, H
        0xd4 => {
            set(&mut cpu.registers, EightBitRegister::H, 2);

            2
        }
        // SET 2, L
        0xd5 => {
            set(&mut cpu.registers, EightBitRegister::L, 2);

            2
        }
        // SET 2, (HL)
        0xd6 => {
            let address = cpu.registers.read_sixteen(SixteenBitRegister::HL);
            let value = cpu.mmu.read_u8(address);
            let result = set_val(value, 2);
            cpu.mmu.write_u8(address, result);

            4
        }
        // SET 2, A
        0xd7 => {
            set(&mut cpu.registers, EightBitRegister::A, 2);

            2
        }
        // SET 3, B
        0xd8 => {
            set(&mut cpu.registers, EightBitRegister::B, 3);

            2
        }
        // SET 3, C
        0xd9 => {
            set(&mut cpu.registers, EightBitRegister::C, 3);

            2
        }
        // SET 3, D
        0xda => {
            set(&mut cpu.registers, EightBitRegister::D, 3);

            2
        }
        // SET 3, E
        0xdb => {
            set(&mut cpu.registers, EightBitRegister::E, 3);

            2
        }
        // SET 3, H
        0xdc => {
            set(&mut cpu.registers, EightBitRegister::H, 3);

            2
        }
        // SET 3, L
        0xdd => {
            set(&mut cpu.registers, EightBitRegister::L, 3);

            2
        }
        // SET 3, (HL)
        0xde => {
            let address = cpu.registers.read_sixteen(SixteenBitRegister::HL);
            let value = cpu.mmu.read_u8(address);
            let result = set_val(value, 3);
            cpu.mmu.write_u8(address, result);

            4
        }
        // SET 4, A
        0xdf => {
            set(&mut cpu.registers, EightBitRegister::A, 3);

            2
        }
        // SET 4, B
        0xe0 => {
            set(&mut cpu.registers, EightBitRegister::B, 4);

            2
        }
        // SET 4, C
        0xe1 => {
            set(&mut cpu.registers, EightBitRegister::C, 4);

            2
        }
        // SET 4, D
        0xe2 => {
            set(&mut cpu.registers, EightBitRegister::D, 4);

            2
        }
        // SET 4, E
        0xe3 => {
            set(&mut cpu.registers, EightBitRegister::E, 4);

            2
        }
        // SET 3, H
        0xe4 => {
            set(&mut cpu.registers, EightBitRegister::H, 4);

            2
        }
        // SET 4, L
        0xe5 => {
            set(&mut cpu.registers, EightBitRegister::L, 4);

            2
        }
        // SET 4, (HL)
        0xe6 => {
            let address = cpu.registers.read_sixteen(SixteenBitRegister::HL);
            let value = cpu.mmu.read_u8(address);
            let result = set_val(value, 4);
            cpu.mmu.write_u8(address, result);

            4
        }

        // SET 4, A
        0xe7 => {
            set(&mut cpu.registers, EightBitRegister::A, 4);

            2
        }
        // SET 5, B
        0xe8 => {
            set(&mut cpu.registers, EightBitRegister::B, 5);

            2
        }
        // SET 5, C
        0xe9 => {
            set(&mut cpu.registers, EightBitRegister::C, 5);

            2
        }
        // SET 5, D
        0xea => {
            set(&mut cpu.registers, EightBitRegister::D, 5);

            2
        }
        // SET 5, E
        0xeb => {
            set(&mut cpu.registers, EightBitRegister::E, 5);

            2
        }
        // SET 5, H
        0xec => {
            set(&mut cpu.registers, EightBitRegister::H, 5);

            2
        }
        // SET 5, L
        0xed => {
            set(&mut cpu.registers, EightBitRegister::L, 5);

            2
        }
        // SET 5, (HL)
        0xee => {
            let address = cpu.registers.read_sixteen(SixteenBitRegister::HL);
            let value = cpu.mmu.read_u8(address);
            let result = set_val(value, 5);
            cpu.mmu.write_u8(address, result);

            4
        }
        // SET 5, A
        0xef => {
            set(&mut cpu.registers, EightBitRegister::A, 5);

            2
        }
        // SET 6, B
        0xf0 => {
            set(&mut cpu.registers, EightBitRegister::B, 6);

            2
        }
        // SET 6, C
        0xf1 => {
            set(&mut cpu.registers, EightBitRegister::C, 6);

            2
        }
        // SET 6, D
        0xf2 => {
            set(&mut cpu.registers, EightBitRegister::D, 6);

            2
        }
        // SET 6, E
        0xf3 => {
            set(&mut cpu.registers, EightBitRegister::E, 6);

            2
        }
        // SET 6, H
        0xf4 => {
            set(&mut cpu.registers, EightBitRegister::H, 6);

            2
        }
        // SET 6, L
        0xf5 => {
            set(&mut cpu.registers, EightBitRegister::L, 6);

            2
        }
        // SET 6, (HL)
        0xf6 => {
            let address = cpu.registers.read_sixteen(SixteenBitRegister::HL);
            let value = cpu.mmu.read_u8(address);
            let result = set_val(value, 6);
            cpu.mmu.write_u8(address, result);

            4
        }
        // SET 6, A
        0xf7 => {
            set(&mut cpu.registers, EightBitRegister::A, 6);

            2
        }
        // SET 7, B
        0xf8 => {
            set(&mut cpu.registers, EightBitRegister::B, 7);

            2
        }
        // SET 7, C
        0xf9 => {
            set(&mut cpu.registers, EightBitRegister::C, 7);

            2
        }
        // SET 7, D
        0xfa => {
            set(&mut cpu.registers, EightBitRegister::D, 7);

            2
        }
        // SET 7, E
        0xfb => {
            set(&mut cpu.registers, EightBitRegister::E, 7);

            2
        }
        // SET 7, H
        0xfc => {
            set(&mut cpu.registers, EightBitRegister::H, 7);

            2
        }
        // SET 7, L
        0xfd => {
            set(&mut cpu.registers, EightBitRegister::L, 7);

            2
        }
        // SET 7, (HL)
        0xfe => {
            let address = cpu.registers.read_sixteen(SixteenBitRegister::HL);
            let value = cpu.mmu.read_u8(address);
            let result = set_val(value, 7);
            cpu.mmu.write_u8(address, result);

            4
        }
        // SET 7, A
        0xff => {
            set(&mut cpu.registers, EightBitRegister::A, 7);

            2
        }
    }
}
