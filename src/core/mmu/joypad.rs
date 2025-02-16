use std::sync::Mutex;

/// Gameboy Joypad register, Directly accessible to the "user"
/// Call its methods to signal to the gameboy that buttons have been pressed
/// and released.
pub struct Joypad {
    // Held in a mutex because both the MMU and the "user" (implementer of the keyboard
    // interaction logic) need write access to the joypad.
    state: Mutex<State>,
}

struct State {
    // Everytime a button is pressed, a joypad interrupt is requested.
    interrupt_request: bool,
    // Internal state of 'buttons'
    button_signal: u8,
    // Internal state of 'buttons'
    dpad_signal: u8,
    // Mask set by MMU which controls which signal set to access
    mask: u8,
}

impl State {
    pub fn new() -> Self {
        Self {
            interrupt_request: false,
            button_signal: 0xF,
            dpad_signal: 0xF,
            mask: 0,
        }
    }
}

/// Represents the set of buttons available on the gameboy.
/// Used to signal the Joypad register that a button is being "pressed"
/// or "released".
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Button {
    // Directional Up
    Up,
    // Directional Down
    Down,
    // Direction Left
    Left,
    // Direction Right
    Right,
    // A button
    A,
    // B Button
    B,
    // Select Button
    Select,
    // Start Button
    Start,
}

/// The eight button interactions are aranged as a 2x4 matrix.
/// All mapped to a single 8bit register. The register is represented
/// as follows:
/// The exact mappings are as follows:
/// - Bit 7-6: Unused
/// - Bit 5: Select Button (row)
/// - Bit 4: Select d-pad (row)
/// - Bit 3: Start / Down
/// - Bit 2: Select / Up
/// - Bit 1: B / Left
/// - Bit 0: A / Right
///
/// Select buttons -- If this is 0: Buttons can be read from the lower nibble
/// Select d-pad -- If this is 0: Directional keys can be read from the lower
///                 nibble.
///
/// The lower nibble is read-only from the perspective of the MMU. The bit being
/// 0 (unset) signifies the button being pressed.
///
/// If both button and d-pad select is set (0x30) the lower nibble reads 0xF.
impl Joypad {
    pub fn new() -> Self {
        Self {
            state: Mutex::new(State::new()),
        }
    }

    // Should be available at 0xFF00. Reads the state of the joypad register
    pub(super) fn read(&self) -> u8 {
        let guard = self.state.lock().expect("Should aqcuire mutext");

        if guard.mask & 0x10 == 0 {
            return guard.dpad_signal | guard.mask;
        }

        if guard.mask & 0x20 == 0 {
            return guard.button_signal | guard.mask;
        }

        return guard.mask | 0xF;
    }

    // Should be available at 0xFF00. Writes the state of the joypad register.
    pub(super) fn write(&self, value: u8) {
        let mut guard = self.state.lock().expect("Should aqcuire mutex");

        guard.mask = value & 0xF0
    }

    pub(super) fn interrupt_requested(&self) -> bool {
        let guard = self.state.lock().expect("Should aqcuire mutex");

        guard.interrupt_request
    }

    pub(super) fn reset_interrupt(&self) {
        let mut guard = self.state.lock().expect("Should aqcuire mutex");

        guard.interrupt_request = false;
    }

    // Signal that a button has been pressed
    pub fn button_down(&self, button: Button) {
        let mut guard = self.state.lock().expect("Should aqcuire mutex");

        match button {
            Button::Up => guard.dpad_signal &= 0x4 ^ 0xF,
            Button::Down => guard.dpad_signal &= 0x8 ^ 0xF,
            Button::Left => guard.dpad_signal &= 0x2 ^ 0xF,
            Button::Right => guard.dpad_signal &= 0x1 ^ 0xF,
            Button::A => guard.button_signal &= 0x1 ^ 0xF,
            Button::B => guard.button_signal &= 0x2 ^ 0xF,
            Button::Select => guard.button_signal &= 0x4 ^ 0xF,
            Button::Start => guard.button_signal &= 0x8 ^ 0xF,
        };

        guard.interrupt_request = true;
    }

    // Signal that a button has been released
    pub fn button_release(&self, button: Button) {
        let mut guard = self.state.lock().expect("Should acquire mutex");

        match button {
            Button::Up => guard.dpad_signal |= 0x4,
            Button::Down => guard.dpad_signal |= 0x8,
            Button::Left => guard.dpad_signal |= 0x2,
            Button::Right => guard.dpad_signal |= 0x1,
            Button::A => guard.button_signal |= 0x1,
            Button::B => guard.button_signal |= 0x2,
            Button::Select => guard.button_signal |= 0x4,
            Button::Start => guard.button_signal |= 0x8,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_write() {
        let joypad = Joypad::new();

        joypad.write(0xF0);

        assert_eq!(joypad.read(), 0xFF);
    }

    #[test]
    fn ignores_writes_to_lower_nibble() {
        let joypad = Joypad::new();

        joypad.write(0x20);

        assert_eq!(joypad.read(), 0x20 | 0xF);
    }

    #[test]
    fn button_down_release_buttons() {
        let joypad = Joypad::new();
        joypad.write(0x10);

        joypad.button_down(Button::A);
        assert_eq!(joypad.read(), 0x10 | 0b1110);
        joypad.button_release(Button::A);
        assert_eq!(joypad.read(), 0x10 | 0xF);

        joypad.button_down(Button::B);
        assert_eq!(joypad.read(), 0x10 | 0b1101);
        joypad.button_release(Button::B);
        assert_eq!(joypad.read(), 0x10 | 0xF);

        joypad.button_down(Button::Select);
        assert_eq!(joypad.read(), 0x10 | 0b1011);
        joypad.button_release(Button::Select);
        assert_eq!(joypad.read(), 0x10 | 0xF);

        joypad.button_down(Button::Start);
        assert_eq!(joypad.read(), 0x10 | 0b0111);
        joypad.button_release(Button::Start);
        assert_eq!(joypad.read(), 0x10 | 0xF);
    }

    #[test]
    fn button_down_release_dpad() {
        let joypad = Joypad::new();
        joypad.write(0x20);

        joypad.button_down(Button::Right);
        assert_eq!(joypad.read(), 0x20 | 0b1110);
        joypad.button_release(Button::Right);
        assert_eq!(joypad.read(), 0x20 | 0xF);

        joypad.button_down(Button::Left);
        assert_eq!(joypad.read(), 0x20 | 0b1101);
        joypad.button_release(Button::Left);
        assert_eq!(joypad.read(), 0x20 | 0xF);

        joypad.button_down(Button::Up);
        assert_eq!(joypad.read(), 0x20 | 0b1011);
        joypad.button_release(Button::Up);
        assert_eq!(joypad.read(), 0x20 | 0xF);

        joypad.button_down(Button::Down);
        assert_eq!(joypad.read(), 0x20 | 0b0111);
        joypad.button_release(Button::Down);
        assert_eq!(joypad.read(), 0x20 | 0xF);
    }

    #[test]
    fn mask_switch_changes_read_dpad_output() {
        let joypad = Joypad::new();

        joypad.write(0x10);
        joypad.button_down(Button::Up);
        assert_eq!(joypad.read(), 0x10 | 0xF);

        // Switch mask
        joypad.write(0x20);
        assert_eq!(joypad.read(), 0x20 | 0b1011);
    }

    #[test]
    fn mask_switch_changes_read_button_output() {
        let joypad = Joypad::new();

        joypad.write(0x20);
        joypad.button_down(Button::A);
        assert_eq!(joypad.read(), 0x20 | 0xF);

        // Switch mask
        joypad.write(0x10);
        assert_eq!(joypad.read(), 0x10 | 0b1110);
    }

    #[test]
    fn interrupt_requested_when_button_pressed() {
        let joypad = Joypad::new();
        joypad.write(0x10);
        joypad.button_down(Button::A);

        assert_eq!(joypad.interrupt_requested(), true);
    }
}
