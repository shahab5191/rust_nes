mod memory;
use super::{bus::Bus, cartridge::Cartridge};
use memory::Memory;

#[derive(Debug, Clone, Copy)]
pub struct Ppu {
    pub cycle: u16,
    pub scanline: u16,
    pub frame_complete: bool,

    pub control: u8,         // 0x2000
    pub mask: u8,            // 0x2001
    pub status: u8,          // 0x2002
    pub oam_addr: u8,        // 0x2003
    pub oam_data: [u8; 256], // sprite memory
    pub scroll_latch: bool,  // latch for 0x2005/0x2006
    pub scroll_x: u8,
    pub scroll_y: u8,
    pub addr_latch: bool,
    pub vram_addr: u16, // current VRAM address
    pub temp_addr: u16, // temp VRAM address
    pub fine_x: u8,
    pub buffered_data: u8, // used for delayed PPU reads

    memory: Memory,
}

impl Ppu {
    pub fn new() -> Self {
        Ppu {
            cycle: 0,
            scanline: 0,
            frame_complete: false,
            control: 0,
            mask: 0,
            status: 0,
            oam_addr: 0,
            oam_data: [0; 256],
            scroll_latch: false,
            scroll_x: 0,
            scroll_y: 0,
            addr_latch: false,
            vram_addr: 0,
            temp_addr: 0,
            fine_x: 0,
            buffered_data: 0,
            memory: Memory::new(),
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

    pub fn read_register(&mut self, addr: u16, cartridge: &Cartridge) -> Option<u8> {
        match addr & 0x2007 {
            0x2002 => {
                // PPUSTATUS
                let data = self.status;
                self.status &= 0x7F; // clear VBlank
                self.addr_latch = false;
                Some(data)
            }
            0x2004 => {
                // OAMDATA
                Some(self.oam_data[self.oam_addr as usize])
            }
            0x2007 => {
                // PPUDATA
                let addr = self.vram_addr & 0x3FFF;
                let result = if addr < 0x3F00 {
                    let buffered = self.buffered_data;
                    self.buffered_data = cartridge.mapper.ppu_read(addr).unwrap_or(0);
                    buffered
                } else {
                    // Palette data is immediate
                    cartridge.mapper.ppu_read(addr).unwrap_or(0)
                };
                // Increment address
                self.vram_addr += if self.control & 0x04 != 0 { 32 } else { 1 };
                Some(result)
            }
            _ => Some(0),
        }
    }

    pub fn write_register(&mut self, addr: u16, value: u8, cartridge: &mut Cartridge) {
        match addr & 0x2007 {
            0x2000 => {
                // PPUCTRL
                self.control = value;
                self.temp_addr = (self.temp_addr & 0xF3FF) | (((value as u16) & 0x03) << 10);
            }
            0x2001 => {
                // PPUMASK
                self.mask = value;
            }
            0x2003 => {
                // OAMADDR
                self.oam_addr = value;
            }
            0x2004 => {
                // OAMDATA
                self.oam_data[self.oam_addr as usize] = value;
                self.oam_addr = self.oam_addr.wrapping_add(1);
            }
            0x2005 => {
                // PPUSCROLL
                if !self.scroll_latch {
                    self.scroll_x = value;
                    self.temp_addr = (self.temp_addr & 0xFFE0) | ((value >> 3) as u16);
                    self.fine_x = value & 0x07;
                } else {
                    self.scroll_y = value;
                    self.temp_addr = (self.temp_addr & 0x8FFF) | (((value as u16) & 0x07) << 12);
                    self.temp_addr = (self.temp_addr & 0xFC1F) | (((value as u16) & 0xF8) << 2);
                }
                self.scroll_latch = !self.scroll_latch;
            }
            0x2006 => {
                // PPUADDR
                if !self.addr_latch {
                    self.temp_addr = (self.temp_addr & 0x00FF) | (((value as u16) & 0x3F) << 8);
                } else {
                    self.temp_addr = (self.temp_addr & 0xFF00) | (value as u16);
                    self.vram_addr = self.temp_addr;
                }
                self.addr_latch = !self.addr_latch;
            }
            0x2007 => {
                // PPUDATA
                let addr = self.vram_addr & 0x3FFF;
                cartridge.mapper.ppu_write(addr, value);
                self.vram_addr += if self.control & 0x04 != 0 { 32 } else { 1 };
            }
            _ => {}
        }
    }
}
