mod memory;
use std::{cell::RefCell, rc::Rc};

use super::cartridge::Cartridge;
use memory::Memory;

#[derive(Debug, Clone)]
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
    cartridge: Rc<RefCell<Cartridge>>,
}

impl Ppu {
    pub fn new(cartridge: Rc<RefCell<Cartridge>>) -> Self {
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
            cartridge,
        }
    }

    pub fn reset(&mut self) {
        self.cycle = 0;
        self.scanline = 0;
        self.frame_complete = false;
    }

    pub fn tick(&mut self) {
        self.cycle += 1;
        if self.scanline == 261 {
            if self.cycle == 1 {
                // End of VBlank, clear VBlank flag
                self.status &= !0xC0; // Clear VBlank and sprite 0 hit flags
            } else if self.cycle >= 280 && self.cycle < 304 {
                // Pre-render scanline, reset PPU state
                self.vram_addr = (self.vram_addr & 0x7BE0) | (self.temp_addr & 0x041F);
            } else if self.cycle >= 1 && self.cycle < 256 {
                // Visible scanline, reset PPU state
                self.vram_addr = (self.vram_addr & 0x7FE0) | (self.temp_addr & 0x001F);
            }
        } else if self.scanline < 240 {
            // Visible scanlines
            if self.cycle >= 1 && self.cycle <= 256 {
                // Render pixel logic
                let nt_addr = (self.vram_addr >> 10) & 0x03;
                let tile_id = self.read_vram(self.vram_addr).unwrap_or(0);
                let attr_addr = 0x23C0
                    | (self.vram_addr & 0x0C00)
                    | ((self.vram_addr >> 4) & 0x38)
                    | ((self.vram_addr >> 2) & 0x07);
                let attr = self.read_vram(attr_addr).unwrap_or(0);
                let mut pallette_index = attr;
                if (self.vram_addr & 0x0040) != 0 {
                    // bottem half
                    pallette_index >>= 4;
                }
                if (self.vram_addr & 0x0002) != 0 {
                    // Right half
                    pallette_index >>= 2;
                }

                let pallette_selector = (pallette_index & 0x03) << 2;

                // TODO: Fetch tile data and render pixel
            } else if self.cycle == 257 {
                // Reset horizontal bits of vram_addr from temp_addr (for next scanline's start)
                self.vram_addr = (self.vram_addr & 0x7BE0) | (self.temp_addr & 0x041F);
            } else if self.cycle == 328 {
            }
        } else if self.scanline == 240 {
        } else if self.scanline >= 241 && self.scanline < 261 {
            // Post-render scanlines
            if self.scanline == 241 && self.cycle == 1 {
                // Start of VBlank, set VBlank flag
                self.status |= 0x80; // Set VBlank flag
                self.frame_complete = true; // Indicate frame completion
                if (self.control >> 7) & 0x01 != 0 {
                    // If NMI is enabled, trigger NMI
                    // This would typically be handled by the CPU
                    eprintln!("NMI Triggered");
                }
            }
        }
        if self.cycle >= 341 {
            self.cycle = 0;
            self.scanline += 1;
            if self.scanline >= 262 {
                self.scanline = 0; // Reset scanline after reaching the end
            }
        }
    }

    fn read_vram(&mut self, addr: u16) -> Option<u8> {
        let mapped_addr = addr & 0x3FFF; // Mask to 14 bits
        if mapped_addr < 0x3F00 {
            // CHR ROM
            self.cartridge.borrow().mapper.ppu_read(mapped_addr)
        } else if mapped_addr >= 0x3F00 && mapped_addr < 0x4000 {
            // Palette memory
            let address = (mapped_addr - 0x3F00) % 0x20;
            self.memory.read_palette(address)
        } else {
            // Invalid address
            eprintln!("PPU Read: Invalid address 0x{:04X}", addr);
            None
        }
    }

    fn write_vram(&mut self, addr: u16, value: u8) {
        let mapped_addr = addr & 0x3FFF; // Mask to 14 bits
        if mapped_addr < 0x3F00 {
            // CHR ROM
            self.cartridge
                .borrow_mut()
                .mapper
                .ppu_write(mapped_addr, value);
        } else if mapped_addr >= 0x3F00 && mapped_addr < 0x4000 {
            // Palette memory
            let address = (mapped_addr - 0x3F00) % 0x20;
            self.memory.write_palette(address, value);
        } else {
            // Invalid address
            eprintln!("PPU Write: Invalid address 0x{:04X}", addr);
        }
    }

    pub fn read_register(&mut self, addr: u16) -> Option<u8> {
        match addr & 0x2007 {
            0x2002 => {
                // PPUSTATUS
                let value = self.status.clone();
                self.status &= !0x80;
                self.addr_latch = false;
                self.scroll_latch = false;
                Some(value)
            }
            0x2004 => {
                // OAMDATA
                let val = Some(self.oam_data[self.oam_addr as usize]);
                self.oam_addr = self.oam_addr.wrapping_add(1);
                val
            }
            0x2007 => {
                // PPUDATA
                let value = self.read_vram(addr);
                let result: Option<u8>;
                if addr & 0x3FFF >= 0x3F00 {
                    // Palette memory is imidiately read
                    result = value;
                } else {
                    // Regular VRAM read is delayed
                    result = Some(self.buffered_data);
                }
                self.buffered_data = value.unwrap_or(0);
                self.vram_addr =
                    self.vram_addr
                        .wrapping_add(if self.control & 0x04 != 0 { 32 } else { 1 });
                result
            }
            _ => Some(0),
        }
    }

    pub fn write_register(&mut self, addr: u16, value: u8) {
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
                self.write_vram(addr, value);
                self.vram_addr =
                    self.vram_addr
                        .wrapping_add(if self.control & 0x04 != 0 { 32 } else { 1 });
            }
            _ => {}
        }
    }
}
