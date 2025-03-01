pub struct Header {
    pub cartridge_type: CartridgeType,
    // Enum for rom size
    pub rom_size: ROMSize,
    // External ram for the cartridge
    pub ram_size: RAMSize,
}

impl Header {
    pub fn new(data: &[u8]) -> Self {
        Self {
            cartridge_type: (&data[0x147]).try_into().expect("Unknown cartridge type"),
            rom_size: (&data[0x148]).try_into().expect("Unknown rom size"),
            ram_size: (&data[0x149]).try_into().expect("Unknown ram size"),
        }
    }

    pub fn rom_bank_count(&self) -> usize {
        match self.rom_size {
            ROMSize::KB32 => 2,
            ROMSize::KB64 => 4,
            ROMSize::KB128 => 8,
            ROMSize::KB256 => 16,
            ROMSize::KB512 => 32,
            ROMSize::KB1024 => 64,
            ROMSize::KB2048 => 128,
            ROMSize::KB4096 => 256,
            ROMSize::KB8192 => 512,
        }
    }

    pub fn ram_bank_count(&self) -> usize {
        match self.ram_size {
            RAMSize::None => 0,
            RAMSize::KB2 => 1,
            RAMSize::KB8 => 1,
            RAMSize::KB32 => 4,
            RAMSize::KB128 => 16,
            RAMSize::KB64 => 8,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CartridgeType {
    ROMOnly,
    MBC1,
    MBC1Battery,
    MBC3,
    MBC3Battery,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ROMSize {
    KB32,
    KB64,
    KB128,
    KB256,
    KB512,
    KB1024,
    KB2048,
    KB4096,
    KB8192,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RAMSize {
    None,
    KB2,
    KB8,
    KB32,
    KB128,
    KB64,
}

impl TryFrom<&u8> for CartridgeType {
    type Error = String;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            // Rom Only
            0x00 => Ok(CartridgeType::ROMOnly),
            // MBC1
            0x01 => Ok(CartridgeType::MBC1),
            // MBC1+RAM
            0x02 => Ok(CartridgeType::MBC1),
            // MBC1+RAM+BATTERY
            0x03 => Ok(CartridgeType::MBC1Battery),
            // ROM+RAM
            0x8 => Ok(CartridgeType::ROMOnly),
            // ROM+RAM+BATTERY, TODO: save
            0x9 => Ok(CartridgeType::ROMOnly),
            // MBC3+Timer+BATTERY
            0x0F => Ok(CartridgeType::MBC3Battery),
            // MBC3+Timer+RAM+BATTERY
            0x10 => Ok(CartridgeType::MBC3Battery),
            // MBC3
            0x11 => Ok(CartridgeType::MBC3),
            // MBC3+RAM
            0x12 => Ok(CartridgeType::MBC3),
            // MBC3+RAM+BATTERY
            0x13 => Ok(CartridgeType::MBC3Battery),
            n => {
                let error = format!("Unsupported Cartridge type {:X}", n);
                Err(error)
            }
        }
    }
}

impl TryFrom<&u8> for ROMSize {
    type Error = &'static str;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(ROMSize::KB32),
            0x01 => Ok(ROMSize::KB64),
            0x02 => Ok(ROMSize::KB128),
            0x03 => Ok(ROMSize::KB256),
            0x04 => Ok(ROMSize::KB512),
            0x05 => Ok(ROMSize::KB1024),
            0x06 => Ok(ROMSize::KB2048),
            0x07 => Ok(ROMSize::KB4096),
            0x08 => Ok(ROMSize::KB8192),
            _ => Err("Unknown rom size"),
        }
    }
}

impl TryFrom<&u8> for RAMSize {
    type Error = &'static str;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(RAMSize::None),
            0x01 => Ok(RAMSize::KB2),
            0x02 => Ok(RAMSize::KB8),
            0x03 => Ok(RAMSize::KB32),
            0x04 => Ok(RAMSize::KB128),
            0x05 => Ok(RAMSize::KB64),
            _ => Err("Unknown ram size"),
        }
    }
}
