use crate::mmu::Interrupt;

use super::*;

impl CPU {
    pub fn interrupt_step(&mut self) -> u8 {
        match (self.ime, self.halted) {
            (false, false) => 0,
            (false, true) => self.halted_pending_interrupt(),
            (true, false) => self.interrupt(),
            (true, true) => self.halted_interrupt(),
        }
    }

    fn halted_pending_interrupt(&mut self) -> u8 {
        if self.mmu.interrupts.interrupt_mask() == 0 {
            return 0;
        }

        self.halted = false;
        return 0;
    }

    fn halted_interrupt(&mut self) -> u8 {
        if !self.mmu.interrupts.has_interrupt() {
            return 0;
        }

        self.halted = false;
        self.interrupt()
    }

    fn interrupt(&mut self) -> u8 {
        let interrupts = self.mmu.interrupts.interrupt_mask();

        for i in 0..5 {
            if interrupts & (1 << i) != 0 {
                let interrupt: Interrupt = (1 << i).into();
                self.ime = false;
                self.mmu.interrupts.interrupt_service(interrupt);
                stack_push(self, self.registers.read_sixteen(SixteenBitRegister::PC));
                self.interrupt_jump(i);
                return 4;
            }
        }

        return 0;
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
