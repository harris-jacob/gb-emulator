use std::time::{SystemTime, UNIX_EPOCH};

/// Real time clock that continues to tick when the gameboy is powered
/// off.
pub struct RTC {
    // Because the RTC should 'continue to tick' when the emulator is off. We
    // keep track of a theoretical clock zero so we can emulate the clock
    // being active when the emulator is not running. For this to work, the
    // zero time must be saved to disk.
    zero: u64,
    seconds: u8,
    minutes: u8,
    hours: u8,
    days: Days,
    // Latched data starts empty, accessing the clock registers before latching
    // will return the 'live' values. Once latched, the latched value is always
    // returned until it is updated.
    latched: Option<LatchedClockData>,
}

/// State struct which represents the 'persisted' RTC state that is used to
/// emulate 'battery' backed hardware.
pub struct RTCState {
    pub zero: u64,
}

/// When writing $00, and then $01 to this register, the current time becomes
/// latched into the RTC registers. The latched data will not change until it
/// becomes latched again, by repeating the write $00->$01 procedure. This
/// provides a way to read the RTC registers while the clock keeps ticking.
/// $08  RTC S   Seconds   0-59 ($00-$3B)
/// $09  RTC M   Minutes   0-59 ($00-$3B)
/// $0A  RTC H   Hours     0-23 ($00-$17)
/// $0B  RTC DL  Lower 8 bits of Day Counter ($00-$FF)
/// $0C  RTC DH  Upper 1 bit of Day Counter, Carry Bit, Halt Flag
///       Bit 0  Most significant bit of Day Counter (Bit 8)
///       Bit 6  Halt (0=Active, 1=Stop Timer)
///       Bit 7  Day Counter Carry Bit (1=Counter Overflow)
pub struct LatchedClockData {
    pub seconds: u8,
    pub minutes: u8,
    pub hours: u8,
    pub days_lower: u8,
    pub days_upper: u8,
}

/// First 9 bits represent the number of days (0-512). Rest
/// of the register is reserved for flags:
/// Bit 13 - Halt (0=active, 1=StopTimer)
/// But 14 - Day Counter carry (1=Day Counter overflow)
pub struct Days(u16);

impl RTC {
    pub fn new(now: SystemTime) -> Self {
        Self {
            zero: Self::since_epoch(now),
            seconds: 0,
            minutes: 0,
            hours: 0,
            days: Days::new(),
            latched: None,
        }
    }

    pub fn write_seconds(&mut self, now: SystemTime, new: u8) {
        // In reality i'm not sure what happens on the hardware if a value larger
        // than this is written to this register (I think its undefined behaviour)
        // but clamping it seems like a good idea for emulation. It ensures nothing
        // strange happens on the next 'tick'.
        self.seconds = new.clamp(9, 59);
        self.calculate_zero(now);
    }

    // If the RTC has been latched, reads the seconds of the latch register.
    // If the RTC has never been latched, returns the 'live' seconds register
    pub fn read_seconds(&self) -> u8 {
        self.latched
            .as_ref()
            .map(|latched| latched.seconds)
            .unwrap_or(self.seconds)
    }

    pub fn write_minutes(&mut self, now: SystemTime, new: u8) {
        // As above
        self.minutes = new.clamp(0, 59);
        self.calculate_zero(now);
    }

    // If the RTC has been latched, reads the seconds of the latch register.
    // If the RTC has never been latched, returns the 'live' minutes register
    pub fn read_minutes(&self) -> u8 {
        self.latched
            .as_ref()
            .map(|latched| latched.minutes)
            .unwrap_or(self.minutes)
    }

    pub fn write_hours(&mut self, now: SystemTime, new: u8) {
        // As above
        self.hours = new.clamp(0, 23);
        self.calculate_zero(now);
    }

    // If the RTC has been latched, reads the seconds of the latch register.
    // If the RTC has never been latched, returns the 'live' hours register
    pub fn read_hours(&self) -> u8 {
        self.latched
            .as_ref()
            .map(|latched| latched.hours)
            .unwrap_or(self.hours)
    }

    pub fn write_days_lower(&mut self, now: SystemTime, new: u8) {
        self.days.set_lower(new);
        self.calculate_zero(now);
    }

    // If the RTC has been latched, reads the seconds of the latch register.
    // If the RTC has never been latched, returns the 'live' days_lower register
    pub fn read_days_lower(&self) -> u8 {
        self.latched
            .as_ref()
            .map(|latched| latched.days_lower)
            .unwrap_or(self.days.lower())
    }

    pub fn write_days_upper(&mut self, now: SystemTime, new: u8) {
        self.days.set_upper(new);
        self.calculate_zero(now);
    }

    // If the RTC has been latched, reads the seconds of the latch register.
    // If the RTC has never been latched, returns the 'live' days_upper register
    pub fn read_days_upper(&self) -> u8 {
        self.latched
            .as_ref()
            .map(|latched| latched.days_upper)
            .unwrap_or(self.days.upper())
    }

    fn since_epoch(time: SystemTime) -> u64 {
        time.duration_since(UNIX_EPOCH)
            .expect("time should be after epoch")
            .as_secs()
    }

    fn calculate_zero(&mut self, now: SystemTime) {
        let mut since_epoch = Self::since_epoch(now);
        since_epoch = since_epoch
            .checked_sub(self.seconds as u64)
            .expect("Now is too close to epoch");

        since_epoch = since_epoch
            .checked_sub(self.minutes as u64 * 60)
            .expect("Now is too close to epoch");

        since_epoch = since_epoch
            .checked_sub(self.hours as u64 * 3600)
            .expect("Now is too close to epoch");

        self.zero = since_epoch
            .checked_sub(self.days.days() as u64 * 3600 * 24)
            .expect("Now is too close to epoch");
    }

    /// Emulation cycle of the RTC. Updates the state of the interal registers
    /// relative to the time passed since the last call.
    pub fn update(&mut self, now: SystemTime) {
        if self.days.halted() {
            // When we halt we recalculate zero relative to the registers to
            // give the illusion that the registers are 'frozen'. If we didn't
            // do this, as soon as we unhalted, the registers would 'jump' the
            // next time they were read/updated.
            self.calculate_zero(now);
        }

        let duration = Self::since_epoch(now)
            .checked_sub(self.zero)
            .expect("Now should be after tzero");

        self.seconds = (duration % 60) as u8;
        self.minutes = ((duration / 60) % 60) as u8;
        self.hours = ((duration / 3600) % 24) as u8;

        let days = (duration / (3600 * 24)) as u16;

        self.days.update_days(days);
    }

    /// Copies the current state of the internal registers to the 'latched'
    /// registers. Latch register is a 'snapshot' of the clock and remains in
    /// fixed until this function is called again
    pub fn latch(&mut self) {
        self.latched = Some(LatchedClockData {
            seconds: self.seconds,
            minutes: self.minutes,
            hours: self.hours,
            days_lower: self.days.lower(),
            days_upper: self.days.upper(),
        })
    }
}

impl Days {
    fn new() -> Self {
        Self(0)
    }

    fn update_days(&mut self, new: u16) {
        if new >= 512 {
            self.set_overflow();
        }

        self.0 = (new & 0x1FF) | (self.0 & !0x1FF);
    }

    fn overflow(&self) -> bool {
        self.0 & (1 << 15) != 0
    }

    fn halted(&self) -> bool {
        self.0 & (1 << 14) != 0
    }

    fn set_halted(&mut self, value: bool) {
        if value {
            self.0 |= 1 << 14
        } else {
            self.0 &= !(1 << 14)
        };
    }

    fn days(&self) -> u16 {
        self.0 & 0x1FF
    }

    fn set_overflow(&mut self) {
        self.0 |= 1 << 15;
    }

    fn set_upper(&mut self, new: u8) {
        self.0 = self.0 & 0xFF | ((new as u16) << 8)
    }

    fn set_lower(&mut self, new: u8) {
        self.0 = self.0 & 0xFF00 | new as u16
    }

    fn lower(&self) -> u8 {
        self.0 as u8
    }

    fn upper(&self) -> u8 {
        (self.0 >> 8) as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod rtc {
        use std::time::Duration;

        use super::*;

        #[test]
        fn live_registers_returned_when_never_latch() {
            let now = SystemTime::now();

            let later = now
                .checked_add(Duration::from_secs(10))
                .expect("valid time")
                .checked_add(Duration::from_secs(60 * 10))
                .expect("valid time")
                .checked_add(Duration::from_secs(3600 * 10))
                .expect("valid time")
                .checked_add(Duration::from_secs(3600 * 24 * 10))
                .expect("valid time");

            let mut rtc = RTC::new(now);

            assert_eq!(rtc.read_seconds(), 0);
            assert_eq!(rtc.read_minutes(), 0);
            assert_eq!(rtc.read_hours(), 0);
            assert_eq!(rtc.read_days_lower(), 0);
            assert_eq!(rtc.read_days_upper(), 0);

            // Clock the RTC forward to later
            rtc.update(later);

            assert_eq!(rtc.read_seconds(), 10);
            assert_eq!(rtc.read_minutes(), 10);
            assert_eq!(rtc.read_hours(), 10);
            assert_eq!(rtc.read_days_lower(), 10);
            assert_eq!(rtc.read_days_upper(), 0);
        }

        #[test]
        fn latched_registers_returned_when_latched() {
            let now = SystemTime::now();

            let later = now
                .checked_add(Duration::from_secs(10))
                .expect("valid time")
                .checked_add(Duration::from_secs(60 * 10))
                .expect("valid time")
                .checked_add(Duration::from_secs(3600 * 10))
                .expect("valid time")
                .checked_add(Duration::from_secs(3600 * 24 * 10))
                .expect("valid time");

            let mut rtc = RTC::new(now);

            assert_eq!(rtc.read_seconds(), 0);
            assert_eq!(rtc.read_minutes(), 0);
            assert_eq!(rtc.read_hours(), 0);
            assert_eq!(rtc.read_days_lower(), 0);
            assert_eq!(rtc.read_days_upper(), 0);

            rtc.latch();

            // Clock the RTC forward to later
            rtc.update(later);

            assert_eq!(rtc.read_seconds(), 0);
            assert_eq!(rtc.read_minutes(), 0);
            assert_eq!(rtc.read_hours(), 0);
            assert_eq!(rtc.read_days_lower(), 0);
            assert_eq!(rtc.read_days_upper(), 0);
        }

        #[test]
        fn relatching_clock_updates_registers() {
            let now = SystemTime::now();

            let later = now
                .checked_add(Duration::from_secs(10))
                .expect("valid time")
                .checked_add(Duration::from_secs(60 * 10))
                .expect("valid time")
                .checked_add(Duration::from_secs(3600 * 10))
                .expect("valid time")
                .checked_add(Duration::from_secs(3600 * 24 * 10))
                .expect("valid time");

            let mut rtc = RTC::new(now);

            assert_eq!(rtc.read_seconds(), 0);
            assert_eq!(rtc.read_minutes(), 0);
            assert_eq!(rtc.read_hours(), 0);
            assert_eq!(rtc.read_days_lower(), 0);
            assert_eq!(rtc.read_days_upper(), 0);

            rtc.latch();

            // Clock the RTC forward to later
            rtc.update(later);

            // Relatch the clock after update
            rtc.latch();

            assert_eq!(rtc.read_seconds(), 10);
            assert_eq!(rtc.read_minutes(), 10);
            assert_eq!(rtc.read_hours(), 10);
            assert_eq!(rtc.read_days_lower(), 10);
            assert_eq!(rtc.read_days_upper(), 0);
        }

        #[test]
        fn test_write_seconds() {
            let now = SystemTime::now();
            let mut rtc = RTC::new(now);

            let now = now
                .checked_add(Duration::from_secs(50))
                .expect("valid time");

            rtc.write_seconds(now, 10);

            rtc.update(now);

            assert_eq!(rtc.read_seconds(), 10);
        }

        #[test]
        fn test_write_minutes() {
            let now = SystemTime::now();
            let mut rtc = RTC::new(now);

            let now = now
                .checked_add(Duration::from_secs(100 * 60))
                .expect("valid time");

            rtc.write_minutes(now, 45);

            rtc.update(now);

            assert_eq!(rtc.read_minutes(), 45);
        }

        #[test]
        fn test_write_hours() {
            let now = SystemTime::now();
            let mut rtc = RTC::new(now);

            let now = now
                .checked_add(Duration::from_secs(20 * 3600))
                .expect("valid time");

            rtc.write_hours(now, 22);

            rtc.update(now);

            assert_eq!(rtc.read_hours(), 22);
        }

        #[test]
        fn test_write_days() {
            let now = SystemTime::now();
            let mut rtc = RTC::new(now);

            let now = now
                .checked_add(Duration::from_secs(100 * 3600 * 24))
                .expect("valid time");

            rtc.write_days_lower(now, 100);
            rtc.write_days_upper(now, 100);

            rtc.update(now);

            assert_eq!(rtc.read_days_lower(), 100);
            assert_eq!(rtc.read_days_upper(), 100);
        }

        #[test]
        fn test_registers_halted() {
            let now = SystemTime::now();
            let mut rtc = RTC::new(now);

            // Halt the RTC
            rtc.write_days_upper(now, 0b01000000);

            let now = now
                .checked_add(Duration::from_secs(100 * 250))
                .expect("valid time");

            rtc.update(now);

            assert_eq!(rtc.read_seconds(), 0);
            assert_eq!(rtc.read_minutes(), 0);
        }

        // test to ensure the RTC registers don't 'jump' when unhalting
        // the RTC.
        #[test]
        fn test_halted_then_unhalt() {
            let now = SystemTime::now();
            let mut rtc = RTC::new(now);

            // Halt the RTC
            rtc.write_days_upper(now, 0b010000000);

            let now = now
                .checked_add(Duration::from_secs(100 * 250))
                .expect("valid time");

            // Unhalt the RTC some time later
            rtc.write_days_upper(now, 0);

            let later = now
                .checked_add(Duration::from_secs(10 * 60))
                .expect("valid time");

            rtc.update(later);

            assert_eq!(rtc.read_minutes(), 10);
            assert_eq!(rtc.read_seconds(), 0);
        }

        #[test]
        fn test_days_overflow() {
            let now = SystemTime::now();
            let mut rtc = RTC::new(now);

            rtc.write_days_lower(now, 0xFF);
            rtc.write_days_upper(now, 0b1);

            let now = now
                .checked_add(Duration::from_secs(24 * 3600))
                .expect("valid time");

            // Update RTC to a day later
            rtc.update(now);

            assert_eq!(rtc.read_days_lower(), 0);
            assert_eq!(rtc.read_days_upper(), 0b10000000);
        }
    }

    mod days {
        use super::*;

        #[test]
        fn set_and_get_lower() {
            let mut days = Days::new();
            days.set_lower(10);

            assert_eq!(days.lower(), 10)
        }

        #[test]
        fn set_and_get_upper() {
            let mut days = Days::new();
            days.set_upper(39);
            assert_eq!(days.upper(), 39);
        }

        #[test]
        fn update_and_get_days() {
            let mut days = Days::new();
            days.update_days(100);

            assert_eq!(days.days(), 100);

            days.update_days(200);

            assert_eq!(days.days(), 200);
        }

        #[test]
        fn update_days_overflow() {
            let mut days = Days::new();

            days.update_days(600);

            assert_eq!(days.days(), 88);
            assert!(days.overflow());
        }

        #[test]
        fn halt() {
            let mut days = Days::new();

            days.set_halted(true);

            assert_eq!(days.upper(), 0b01000000);
            assert!(days.halted());

            days.set_halted(false);
            assert_eq!(days.upper(), 0);
            assert!(!days.halted());
        }
    }
}
