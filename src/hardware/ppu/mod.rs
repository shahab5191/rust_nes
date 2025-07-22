pub mod memory;

use memory::Memory;

#[derive(Debug, Clone, Copy)]
pub struct Ppu {
    pub memory: Memory,
    pub cycle: u16,
    pub scanline: u16,
    pub frame_complete: bool,
}

impl Ppu {
    pub fn new() -> Self {
        Ppu {
            memory: Memory::new(),
            cycle: 0,
            scanline: 0,
            frame_complete: false,
        }
    }

    pub fn reset(&mut self) {
        self.cycle = 0;
        self.scanline = 0;
        self.frame_complete = false;
    }

    pub fn tick(&mut self) {
        if self.cycle < 340 {
            self.cycle += 1;
        } else {
            self.cycle = 0;
            if self.scanline < 261 {
                self.scanline += 1;
            } else {
                self.scanline = 0;
                self.frame_complete = true;
            }
        }
    }

    pub fn read_tile(&self, tile_index: u16) -> [u8; 16] {
        let mut tile_data = [0; 16];
        for i in 0..16 {
            tile_data[i] = self.memory.read_chr(tile_index * 16 + i as u16);
        }
        tile_data
    }

    pub fn tile_to_rgb(&self, tile_data: [u8; 16]) -> [u8; 256] {
        let mut rgb_data: [u8; 256] = [0; 256];
        for row in 0..8 {
            for col in 0..8 {
                let row2 = row + 8;
                let bit0 = (tile_data[row] >> (7 - col)) & 0x1;
                let bit1 = (tile_data[row2] >> (7 - col)) & 0x1;
                let color_index: u8 = (bit1 << 1) | bit0;
                let color: [u8; 4] = match color_index {
                    0 => [0, 0, 0, 255],
                    1 => [75, 75, 75, 255],
                    2 => [200, 200, 200, 255],
                    3 => [255, 255, 255, 255],
                    _ => [0, 0, 0, 255],
                };
                let pixel_index = (row * 8 + col) * 4;
                rgb_data[pixel_index..pixel_index + 4].copy_from_slice(&color);
            }
        }
        rgb_data
    }

    pub fn read_registers(&self, address: u16) -> u8 {
        let reg = address & 0x2007; // Mask to get the relevant register address
        match reg {
            0x2000 => self.memory.read_nametable(0), // PPUCTRL
            0x2001 => self.memory.read_nametable(1), // PPUMASK
            0x2002 => self.memory.read_nametable(2), // PPUSTATUS
            0x2003 => self.memory.read_nametable(3), // OAMADDR
            0x2004 => self.memory.read_nametable(4), // OAMDATA
            0x2005 => self.memory.read_nametable(5), // PPUSCROLL
            0x2006 => self.memory.read_nametable(6), // PPUADDR
            0x2007 => self.memory.read_nametable(7), // PPUDATA
            _ => panic!("Invalid PPU register read at address: {:#X}", address),
        }
    }
}
