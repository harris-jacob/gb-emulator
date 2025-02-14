#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Interrupt {
    VBlank,
    LCDStat,
    Timer,
    Serial,
    Joypad,
}

pub struct Interrupts {
    interrupt_enabled: u8,
    interrupt_flag: u8,
}

impl Interrupts {
    pub fn new() -> Self {
        Self {
            interrupt_flag: 0,
            interrupt_enabled: 0,
        }
    }

    /// Request an interrupt. alters the interrupt flag. Note the
    /// interrupt will not be serviced unless corresponding ie bit
    /// is also set.
    pub fn request_interrupt(&mut self, interrupt: Interrupt) {
        let interrupt: u8 = interrupt.into();
        self.interrupt_flag |= interrupt;
    }

    /// To be called once an interrupt is serviced by the CPU.
    /// Resets the corresponding but in the interrupt flag.
    pub fn interrupt_service(&mut self, interrupt: Interrupt) {
        if interrupt == Interrupt::VBlank {
        }
        let interrupt: u8 = interrupt.into();
        self.interrupt_flag &= !interrupt;
    }

    /// Compares interrupt flag register with interrupt enable register
    pub fn interrupt_mask(&self) -> u8 {
        self.interrupt_flag & self.interrupt_enabled
    }

    pub fn has_interrupt(&self) -> bool {
        self.interrupt_mask() != 0
    }

    pub(super) fn write_interrupt_enabled(&mut self, value: u8) {
        self.interrupt_enabled = value;
    }

    pub(super) fn write_interrupt_flag(&mut self, value: u8) {
        self.interrupt_flag = value;
    }

    pub(super) fn read_interrupt_enabled(&self) -> u8 {
        self.interrupt_enabled
    }

    pub(super) fn read_interrupt_flag(&self) -> u8 {
        self.interrupt_flag
    }
}

impl Into<u8> for Interrupt {
    fn into(self) -> u8 {
        match self {
            Interrupt::VBlank => 0b1,
            Interrupt::LCDStat => 0b10,
            Interrupt::Timer => 0b100,
            Interrupt::Serial => 0b1000,
            Interrupt::Joypad => 0b10000,
        }
    }
}

impl Into<Interrupt> for u8 {
    fn into(self) -> Interrupt {
        match self {
            0b1 => Interrupt::VBlank,
            0b10 => Interrupt::LCDStat,
            0b100 => Interrupt::Timer,
            0b1000 => Interrupt::Serial,
            0b10000 => Interrupt::Joypad,
            _ => panic!("Invalid interrupt integer"),
        }
    }
}
