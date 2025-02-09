/// Implements the timer hardware, keeps track o the divider, counter, modulo, and control
/// registers and updates them based on the number of cycles that have passed.
#[derive(Debug)]
pub struct Timer {
    /// DIV: Divider Register
    /// Counts up at a fixed rate of 16384 Hz (64 M-cycles/256 T-Cycles)
    /// Internally is a 16-bit counter which is incremented every T-Cycle
    /// only the upper 8 bits are mapped to memory, This register can be read
    /// at any point in time. Writing to this register resets the whole thing
    /// to zero.
    divider: u16,
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

    previous_and_result: bool,

    /// When the divider register overflows, the timer requests an interrupt
    /// this flag is set to true when the divider overflows and is reset when read
    pub interrupt_request: bool,

    has_overflowed: bool,
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
            previous_and_result: false,
            has_overflowed: false,
        }
    }

    /// Read the 8 mapped bits of the divider register.
    /// Should be available at 0xFF04.
    pub fn read_divider(&self) -> u8 {
        let divider = self.divider >> 8;

        divider as u8
    }

    /// Writing to the divider register resets the whole thing to 0
    /// Should be available at 0xFF04.
    pub fn write_divider(&mut self, _: u8) {
        self.divider = 0;
    }

    /// Read the timer counter register
    /// Should be available at 0xFF05.
    pub fn read_counter(&self) -> u8 {
        self.counter
    }

    /// Write to the timer counter register
    /// Should be available at 0xFF05.
    pub fn write_counter(&mut self, value: u8) {
        self.counter = value;
    }

    /// Read the timer modulo register
    /// Should be available at 0xFF06.
    pub fn read_modulo(&self) -> u8 {
        self.modulo
    }

    /// Write to the timer modulo register
    /// Should be available at 0xFF06.
    pub fn write_modulo(&mut self, value: u8) {
        self.modulo = value;
    }

    /// Read the timer control register
    /// Should be available at 0xFF07.
    pub fn read_control(&self) -> u8 {
        self.control
    }

    /// Write to the timer control register
    /// Should be available at 0xFF07.
    pub fn write_control(&mut self, value: u8) {
        self.control = value;
    }

    /// Step the timer by the given number of cycles
    /// This will update the timer's internal state
    /// based on the number of M-cycles that have passed
    /// since the last step.
    pub fn step(&mut self, m_cycles: u8) {
        let t_cycles = m_cycles * 4;

        for _ in 0..t_cycles {
            self.step_internal();
        }
    }
    fn step_internal(&mut self) {
        self.divider = self.divider.wrapping_add(1);

        if self.has_overflowed {
            self.has_overflowed = false;
            self.interrupt_request = true;
            self.counter = self.modulo;
        }

        if self.should_increment_counter() {
            self.increment_counter();
        }
    }

    fn increment_counter(&mut self) {
        if self.counter == 0xFF {
            self.counter = 0;
            self.has_overflowed = true;
        } else {
            self.counter += 1;
        }
    }

    fn should_increment_counter(&mut self) -> bool {
        let bit_position = self.clock_frequency_to_bit_position();
        let and_result = (self.divider >> bit_position) & self.enabled() == 1;

        let should_increment = self.previous_and_result && !and_result;

        self.previous_and_result = and_result;

        should_increment
    }

    /// Isolate the second bit of the TAC register
    /// which determines whether the timer is enabled or not
    fn enabled(&self) -> u16 {
        ((self.control & 0b0000_0100) >> 2) as u16
    }

    #[cfg(test)]
    fn disable(&mut self) {
        self.control &= 0b1111_1011;
    }

    /// A bit of magic which is eventually used to determine the number of cycles
    /// that need to pass before the timer counter is incremented. This depends
    /// on the clock frequency selected in the control register.
    /// 00: 4096 Hz (256 M-cycles)
    /// 01: 262144 Hz (4 M-cycles)
    /// 10: 65536 Hz (16 M-cycles)
    /// 11: 16384 Hz (64 M-cycles)
    fn clock_frequency_to_bit_position(&self) -> u8 {
        match self.control & 0b0000_0011 {
            0b00 => 9,
            0b01 => 3,
            0b10 => 5,
            0b11 => 7,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter_incremented_at_correct_rate() {
        for rate in &[
            TimerRate::M4,
            TimerRate::M16,
            TimerRate::M64,
            TimerRate::M256,
        ] {
            let mut timer = rate.timer_at();

            run_timer_for(&mut timer, rate.cycles());

            assert_eq!(timer.read_counter(), 1);
        }
    }

    #[test]
    fn counter_not_incremented_when_disabled() {
        let mut timer = Timer::new();

        timer.disable();

        run_timer_for(&mut timer, 1024);

        assert_eq!(timer.read_counter(), 0);
    }

    // In some cases, where the timer is disabled. The and_result flips from
    // a one to a zero (falling edge) causing a TIMA increment even though
    // the corresponding bit of the counter didn't change.
    #[test]
    fn tima_increment_edge_case() {
        let rate = TimerRate::M4;
        let mut timer = rate.timer_at();

        run_timer_for(&mut timer, rate.cycles() - 2);

        timer.disable();

        run_timer_for(&mut timer, 1);

        assert_eq!(timer.read_counter(), 1);
    }

    /// Convience function to run in smaller increments
    /// to avoid overflow arithmetic during tests
    fn run_timer_for(timer: &mut Timer, cycles: u16) {
        for _ in 0..cycles {
            timer.step(1);
        }
    }

    enum TimerRate {
        M4,
        M16,
        M64,
        M256,
    }

    impl TimerRate {
        fn timer_at(&self) -> Timer {
            let mut timer = Timer::new();

            match self {
                TimerRate::M4 => timer.write_control(0b0000_0101),
                TimerRate::M16 => timer.write_control(0b0000_0110),
                TimerRate::M64 => timer.write_control(0b0000_0111),
                TimerRate::M256 => timer.write_control(0b0000_0100),
            }

            timer
        }

        fn cycles(&self) -> u16 {
            match self {
                TimerRate::M4 => 4,
                TimerRate::M16 => 16,
                TimerRate::M64 => 64,
                TimerRate::M256 => 256,
            }
        }
    }
}
