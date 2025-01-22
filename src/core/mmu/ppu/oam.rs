/// OAM stands for Object Atribute Memory. This section of memory ($FE00-FE9F) contains
/// data used to display sprites (or objects). Each sprite is encoded in 4 bytes. For
/// more information see the [Sprite] struct. There is room for 40 sprites to be
/// displayed at any given time.
pub struct OAM([u8; 160]);

/// A sprite is an Object displayed on the screen. It can be moved in increments of 1 pixel (unlike
/// background and window tiles). There can only be 40 Sprites on the screen at a time (these
/// sprites are stored in OAM). A Sprite is encoded into 4 bytes.
///
/// Byte 0 - Y position: Vertical position opf the sprite. This
/// value is offset by 16 pixels (to allow smooth transition onto the screen),
/// so a value of Y = 16 places the sprite top corner of the sprite at the
/// top of the screen.
/// Byte 1 - X position: the Horizontal position of the sprite on the screen
/// As with Y, to allow smooth animation onto the screen, this value is offset
/// by 8. So X = 8 places the sprite at far left of the screen.
/// Byte 2 - TileNumber: Tile Number used for fetching the tile from TileDate
/// sprites always use unsigned (8000) addressing mode, so sprite tiles can
/// only be stored in the first 255 tile data slots.
/// Byte 3 - Sprite Flags. The last Byte represents bit-flags that alter the
/// rendering of a sprite. See [SpriteFlags] struct for more details
pub struct Sprite {
    pub x: u8,
    pub y: u8,
    pub tile_number: u8,
    pub flags: SpriteFlags,
}

/// Which Palette number is the sprite using
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PaletteNumber {
    /// Sprite palette 0
    OBP0,
    /// Sprite palette 1
    OBP1,
}

/// A single byte whos bits apply certain affects to sprites during rendering.
///
/// Bit-7: Obj-to-BG priority: 0 = sprite always rendered above BG. 1 = BG colors
/// 1-3 overlay sprite, sprite is still rendered above Color0.
///
/// Bit 6: Y-flip -- If set, sprite is flipped vertically.
/// Bit 5: X-Flip -- If set, sprite is flipped horizontally
/// Bit 4: Palette Number, If 0, use First sprite palette (0bp0), otherwise
/// Bit 0-3: CGB-only
/// use second sprite palette (0bp1).
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SpriteFlags(u8);

impl SpriteFlags {
    pub fn new(value: u8) -> Self {
        Self(value)
    }
    /// If true, sprite is always rendered above the BG, otherwise the sprite
    /// is only rendered above Color0.
    pub fn bg_priority(&self) -> bool {
        self.0 & 0b10000000 == 0
    }
    /// Is the sprite flipped in Y
    pub fn y_flip(&self) -> bool {
        self.0 & 0b01000000 == 0
    }
    // Is the sprite flipped in X
    pub fn x_flip(&self) -> bool {
        self.0 & 0b00100000 == 0
    }
    pub fn palette_number(&self) -> PaletteNumber {
        if self.0 & 0b00010000 == 0 {
            PaletteNumber::OBP0
        } else {
            PaletteNumber::OBP1
        }
    }
}

impl OAM {
    pub fn new() -> Self {
        Self([0; 160])
    }

    pub fn read(&self, addr: u16) -> u8 {
        Self::check_addr_range(addr);

        self.0[addr as usize]
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        Self::check_addr_range(addr);

        self.0[addr as usize] = value;
    }

    fn check_addr_range(addr: u16) {
        if addr > 160 {
            panic!("Address out of range for OAM")
        }
    }

    fn sprite_at(&self, sprite_number: u8) -> Sprite {
        if sprite_number > 40 {
            panic!("There are only 40 sprites")
        }

        let base = (sprite_number * 4) as usize;

        Sprite {
            x: self.0[base],
            y: self.0[base + 1],
            tile_number: self.0[base + 2],
            flags: SpriteFlags::new(self.0[base + 3]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod oam {
        use super::*;

        #[test]
        #[should_panic]
        fn out_of_range_read_panics() {
            let map = OAM::new();

            map.read(161);
        }

        #[test]
        #[should_panic]
        fn out_of_range_write_panics() {
            let mut map = OAM::new();

            map.write(161, 0);
        }

        #[test]
        fn read_write() {
            let mut map = OAM::new();
            for i in 0u16..160 {
                map.write(i, 1);

                assert_eq!(map.read(i), 1);
            }
        }

        #[test]
        fn sprite_at_returns_correct_sprite() {
            let mut map = OAM::new();
            map.write(20 * 4, 0x10);
            map.write(20 * 4 + 1, 0x20);
            map.write(20 * 4 + 2, 0x30);
            map.write(20 * 4 + 3, 0x40);

            let sprite = map.sprite_at(20);

            assert_eq!(sprite.x, 0x10);
            assert_eq!(sprite.y, 0x20);
            assert_eq!(sprite.tile_number, 0x30);
            assert_eq!(sprite.flags, SpriteFlags::new(0x40));
        }
    }

    mod sprite_flags {
        #[test]
        fn bg_priority() {
            let flags = super::super::SpriteFlags::new(0b00000000);
            assert_eq!(flags.bg_priority(), true);

            let flags = super::super::SpriteFlags::new(0b10000000);
            assert_eq!(flags.bg_priority(), false);
        }

        #[test]
        fn y_flip() {
            let flags = super::super::SpriteFlags::new(0b00000000);
            assert_eq!(flags.y_flip(), true);

            let flags = super::super::SpriteFlags::new(0b01000000);
            assert_eq!(flags.y_flip(), false);
        }

        #[test]
        fn x_flip() {
            let flags = super::super::SpriteFlags::new(0b00000000);
            assert_eq!(flags.x_flip(), true);

            let flags = super::super::SpriteFlags::new(0b00100000);
            assert_eq!(flags.x_flip(), false);
        }

        #[test]
        fn palette_number() {
            let flags = super::super::SpriteFlags::new(0b00000000);
            assert_eq!(flags.palette_number(), super::super::PaletteNumber::OBP0);

            let flags = super::super::SpriteFlags::new(0b00010000);
            assert_eq!(flags.palette_number(), super::super::PaletteNumber::OBP1);
        }
    }
}
