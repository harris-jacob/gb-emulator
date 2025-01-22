#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Interrupt {
    VBlank = 0,
    LCDStat = 1,
    Timer = 2,
    Serial = 3,
    Joypad = 4,
}
