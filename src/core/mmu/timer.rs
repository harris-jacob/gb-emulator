/// Implements the timer hardware, keeps track o the divider, counter, modulo, and control
/// registers and updates them based on the number of cycles that have passed.
pub struct Timer {
    /// DIV: Divider Register
    /// Counts up at a fixed rate of 16384 Hz (64 M-cycles)
    /// resets when overflows or if written to.
    /// still counts when the timer is disabled
    divider: u8,
    /// TIMA: Timer Counter
    /// Counts up at a variable rate based on the control register
    /// triggers a timer interrupt when overflows (resets to modulo)
    /// does not count when the timer is disabled
    counter: u8,
    /// TMA: Timer Modulo
    /// When the counter overflows, it is reset to this value
    modulo: u8,
    /// TAC: Timer Control
    /// Control register for the timer
    /// Bit 2: 1 = Timer enabled, 0 = Timer disabled
    /// Bit 0-1: Timer frequency (see ClockFrequency enum)
    control: u8,

    /// When the divider register overflows, the timer requests an interrupt
    /// this flag is set to true when the divider overflows and is reset when read
    pub interrupt_request: bool,

    internal_divider: u16,
    internal_counter: u16,
}

/// The frequency of the timer
/// 00: 4096 Hz (256 M-cycles)
/// 01: 262144 Hz (4 M-cycles)
/// 10: 65536 Hz (16 M-cycles)
/// 11: 16384 Hz (64 M-cycles)
/// NOTE: M-cycles are machine cycles i.e. a quarter of a clock cycle
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ClockFrequency {
    M256 = 0b00,
    M4 = 0b01,
    M16 = 0b10,
    M64 = 0b11,
}

impl Timer {
    /// Create a new timer with all registers set to 0
    pub fn new() -> Timer {
        Timer {
            divider: 0,
            counter: 0,
            modulo: 0,
            control: 0,
            interrupt_request: false,
            internal_divider: 0,
            internal_counter: 0,
        }
    }

    //// Read the value of the given address (should only be used to read a timer register
    /// i.e. 0xFF04, 0xFF05, 0xFF06, 0xFF07). Panics if the address is invalid.
    pub fn read(&self, address: u16) -> u8 {
        match address {
            0xFF04 => self.divider,
            0xFF05 => self.counter,
            0xFF06 => self.modulo,
            0xFF07 => self.control,
            _ => panic!("Invalid timer address: {:04X}", address),
        }
    }

    /// Write the given value to the given address (should only be used to write to a timer
    /// register i.e. 0xFF04, 0xFF05, 0xFF06, 0xFF07). Panics if the address is invalid.
    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0xFF04 => self.internal_divider = 0,
            0xFF05 => self.counter = value,
            0xFF06 => self.modulo = value,
            0xFF07 => self.control = value,
            _ => panic!("Invalid timer address: {:04X}", address),
        }
    }

    /// Step the timer by the given number of cycles
    /// This will update the timer's internal state
    /// based on the number of M-cycles that have passed
    /// since the last step.
    ///
    pub fn step(&mut self, m_cycles: u8) {
        if self.enabled() {
            self.update_counter(m_cycles);
        }
        self.update_divider(m_cycles);
    }

    fn update_divider(&mut self, m_cycles: u8) {
        self.internal_divider += m_cycles as u16;
        if self.internal_divider >= 64 {
            self.divider = self.divider.wrapping_add(1);
            self.internal_divider -= 64;
        }
    }

    fn update_counter(&mut self, m_cycles: u8) {
        let frequency = self.clock_select();

        let cycles = match frequency {
            ClockFrequency::M256 => 256,
            ClockFrequency::M4 => 4,
            ClockFrequency::M16 => 16,
            ClockFrequency::M64 => 64,
        };

        self.internal_counter += m_cycles as u16;
        if self.internal_counter >= cycles {
            self.counter = self.counter.wrapping_add(1);
            if self.counter == 0 {
                self.counter = self.modulo;
                self.interrupt_request = true;
            }
            self.internal_counter -= cycles;
        };
    }

    fn enabled(&self) -> bool {
        self.control & 0b0000_0100 == 0b0000_0100
    }

    fn clock_select(&self) -> ClockFrequency {
        match self.control & 0b0000_0011 {
            0b00 => ClockFrequency::M256,
            0b01 => ClockFrequency::M4,
            0b10 => ClockFrequency::M16,
            0b11 => ClockFrequency::M64,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_write_timer() {
        let mut timer = Timer::new();

        timer.write(0xFF04, 0x12);
        assert_eq!(timer.read(0xFF04), 0x00);

        timer.write(0xFF05, 0x34);
        assert_eq!(timer.read(0xFF05), 0x34);

        timer.write(0xFF06, 0x56);
        assert_eq!(timer.read(0xFF06), 0x56);

        timer.write(0xFF07, 0x78);
        assert_eq!(timer.read(0xFF07), 0x78);
    }

    #[test]
    fn divider_register_increments_at_64m() {
        let mut timer = Timer::new();

        for _ in 0..63 {
            timer.step(1);
            assert_eq!(timer.read(0xFF04), 0);
        }

        timer.step(1);

        assert_eq!(timer.read(0xFF04), 1);
    }

    #[test]
    fn divider_register_overflow() {
        let mut timer = Timer::new();

        for _ in 0..256 {
            timer.step(1 * 64);
        }

        assert_eq!(timer.read(0xFF04), 0);
    }

    #[test]
    fn timer_register_increments_according_to_tac_rate() {
        counter_register_increment(ClockFrequency::M256);
        counter_register_increment(ClockFrequency::M4);
        counter_register_increment(ClockFrequency::M16);
        counter_register_increment(ClockFrequency::M64);
    }

    #[test]
    fn timer_register_does_not_increment_when_disabled() {
        let mut timer = Timer::new();
        timer.write(0xFF07, 0b0000_0000);

        for _ in 0..rate_to_cycles(ClockFrequency::M256) {
            timer.step(1);
            assert_eq!(timer.read(0xFF05), 0);
        }
    }

    #[test]
    fn timer_register_overflows_and_resets_to_modulo() {
        let mut timer = Timer::new();
        timer.write(0xFF07, 0b0000_0100);
        timer.write(0xFF06, 0x10);
        timer.write(0xFF05, 0xFF);

        for _ in 0..rate_to_cycles(ClockFrequency::M256) {
            timer.step(1);
        }

        assert_eq!(timer.read(0xFF05), 0x10);
    }

    #[test]
    fn timer_register_overflow_requests_interrupt() {
        let mut timer = Timer::new();
        timer.write(0xFF07, 0b0000_0100);
        timer.write(0xFF06, 0x10);
        timer.write(0xFF05, 0xFF);

        for _ in 0..rate_to_cycles(ClockFrequency::M256) {
            timer.step(1);
        }

        assert_eq!(timer.interrupt_request, true);
    }

    fn counter_register_increment(rate: ClockFrequency) {
        let mut timer = Timer::new();
        timer.write(0xFF07, rate as u8 | 0b0000_0100);

        for _ in 0..rate_to_cycles(rate) - 1 {
            timer.step(1);
            assert_eq!(timer.read(0xFF05), 0);
        }

        timer.step(1);
        assert_eq!(timer.read(0xFF05), 1);
    }

    fn rate_to_cycles(rate: ClockFrequency) -> u16 {
        match rate {
            ClockFrequency::M256 => 256,
            ClockFrequency::M4 => 4,
            ClockFrequency::M16 => 16,
            ClockFrequency::M64 => 64,
        }
    }
}
