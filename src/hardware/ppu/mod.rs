pub mod memory;

use crate::utils::Color;
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

    fn read_tile(&self, tile_index: u16) -> [u8; 16] {
        let mut tile_data = [0; 16];
        for i in 0..16 {
            tile_data[i] = self.memory.read_chr(tile_index * 16 + i as u16);
        }
        tile_data
    }

    fn tile_to_rgb(&self, tile_data: [u8; 16]) -> [u8; 256] {
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

    pub fn get_chr_image(&self, table_number: u8) -> [u8; 128 * 128 * 4] {
        let mut image = [0; 128 * 128 * 4];
        let start_tile: u32 = if table_number == 0 { 0 } else { 256 };
        for tile_index in start_tile..start_tile + 256 {
            let mut tile_data = self.read_tile(tile_index as u16);
            let color = self.tile_to_rgb(tile_data);
            let tile_y = (tile_index - start_tile) / 16;
            let tile_x = (tile_index - start_tile) % 16;
            for row in 0..8 {
                for col in 0..8 {
                    let pixel_y = (tile_y * 8) + row;
                    let pixel_x = (tile_x * 8) + col;
                    let pixel_index = ((pixel_y * 128 + pixel_x) * 4) as usize;
                    let color_index = ((row * 8 + col) * 4) as usize;
                    image[pixel_index..pixel_index + 4]
                        .copy_from_slice(&color[color_index..color_index + 4]);
                }
            }
        }
        image
    }
}
