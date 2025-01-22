use super::*;
use crate::core::data::Interrupt;

impl CPU {
    pub fn request_interrupt(&mut self, interrupt: Interrupt) {
        let interrupt_flag = self.mmu.read_u8(0xFF0F);
        self.mmu
            .write_u8(0xFF0F, interrupt_flag | (1 << interrupt as u8));
    }
    pub fn interrupt_step(&mut self) -> u8 {
        match (self.ime, self.halted) {
            (false, false) => 0,
            (false, true) => self.halted_pending_interrupt(),
            (true, false) => self.interrupt(),
            (true, true) => self.halted_interrupt(),
        }
    }

    fn halted_pending_interrupt(&mut self) -> u8 {
        if self.read_interrupts() == 0 {
            return 0;
        }

        self.halted = false;
        return 0;
    }

    fn halted_interrupt(&mut self) -> u8 {
        let interrupts = self.read_interrupts();
        if interrupts == 0 {
            return 0;
        }

        self.halted = false;
        self.interrupt()
    }

    fn interrupt(&mut self) -> u8 {
        let interrupts = self.read_interrupts();

        for i in 0..5 {
            if interrupts & (1 << i) != 0 {
                self.ime = false;
                self.mmu
                    .write_u8(0xFF0F, self.mmu.read_u8(0xFF0F) & !(1 << i));
                stack_push(self, self.registers.read_sixteen(SixteenBitRegister::PC));
                self.interrupt_jump(i);
                return 4;
            }
        }

        return 0;
    }

    fn read_interrupts(&self) -> u8 {
        self.mmu.read_u8(0xFF0F) & self.mmu.read_u8(0xFFFF)
    }

    fn interrupt_jump(&mut self, interrupt: u8) {
        match interrupt {
            0 => self.registers.write_sixteen(SixteenBitRegister::PC, 0x40),
            1 => self.registers.write_sixteen(SixteenBitRegister::PC, 0x48),
            2 => self.registers.write_sixteen(SixteenBitRegister::PC, 0x50),
            3 => self.registers.write_sixteen(SixteenBitRegister::PC, 0x58),
            4 => self.registers.write_sixteen(SixteenBitRegister::PC, 0x60),
            _ => unreachable!(),
        }
    }
}
