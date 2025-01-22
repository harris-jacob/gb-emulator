mod background_map;
mod lcd_control;
mod lcdc_status;
mod oam;
mod tile;
mod tiledata;

use background_map::BackgroundMap;
use lcd_control::LCDControl;
use lcdc_status::LCDStatus;
use oam::OAM;
use tile::Pixel;
use tile::Tile;
use tiledata::TileData;

pub use background_map::BackgroundMapSelection;
pub use oam::SpriteSize;
pub use tiledata::TileAddressingMethod;

pub struct PPU {
    pub bg_map0: BackgroundMap,
    pub bg_map1: BackgroundMap,
    pub oam: OAM,
    pub tiledata: TileData,
    pub lyc: u8,
    pub interrupts: u8,
    ly: u8,
    lcdc: LCDControl,
    lcd_stat: LCDStatus,
}

impl PPU {
    pub fn new() -> Self {
        Self {
            bg_map0: BackgroundMap::new(),
            bg_map1: BackgroundMap::new(),
            lcdc: LCDControl::new(),
            lcd_stat: LCDStatus::new(),
            oam: OAM::new(),
            tiledata: TileData::new(),
            ly: 0,
            lyc: 0,
            interrupts: 0,
        }
    }

    fn update_ly(&mut self, value: u8) {
        self.ly = value;
        self.lcd_stat.set_lyc_eq_ly(self.ly == self.lyc);
    }
}
