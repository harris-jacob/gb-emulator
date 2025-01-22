mod background_map;
mod oam;
mod tile;
mod tiledata;

use background_map::BackgroundMap;
use tile::Pixel;
use tile::Tile;
use tiledata::TileData;

pub struct PPU {
    ly: u8,
    pub tiledata: TileData,
    pub bg_map0: BackgroundMap,
    pub bg_map1: BackgroundMap,
}

impl PPU {
    pub fn new() -> Self {
        Self {
            ly: 0,
            tiledata: TileData::new(),
            bg_map0: BackgroundMap::new(),
            bg_map1: BackgroundMap::new(),
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {}
}
