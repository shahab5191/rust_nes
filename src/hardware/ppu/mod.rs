mod memory;
mod palette_map;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::cartridge::Cartridge;

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

    palette: [u8; 32],
    frame_buffer: Vec<u8>,
    cartridge: Rc<RefCell<Cartridge>>,
    nmi_pending: bool,

    palette_rgba: [[u8; 4]; 32],     // Map for NES colors
    color_map: HashMap<u8, [u8; 4]>, // Map for NES colors
}

impl Ppu {
    pub fn new(cartridge: Rc<RefCell<Cartridge>>) -> Self {
        Ppu {
            cycle: 0,
            scanline: 0,
            frame_complete: false,
            control: 0x00,       // PPUCTRL: All bits off
            mask: 0x00,          // PPUMASK: Rendering off
            status: 0x00,        // PPUSTATUS: No flags set (or 0x80 if starting in VBlank)
            oam_addr: 0x00,      // OAMADDR: Start at 0
            oam_data: [0; 256],  // OAM: Zero-filled (or random for accuracy)
            scroll_latch: false, // PPUSCROLL first write
            scroll_x: 0x00,
            scroll_y: 0x00,
            addr_latch: false,   // PPUADDR first write
            vram_addr: 0x0000,   // Current VRAM address
            temp_addr: 0x0000,   // Temp VRAM address
            fine_x: 0x00,        // Fine X scroll
            buffered_data: 0x00, // PPUDATA read buffer
            cartridge,
            palette: [0; 32], // Palette RAM: Zero-filled (or random)
            frame_buffer: vec![0; 256 * 240 * 4], // Black screen
            nmi_pending: false, // No NMI pending
            palette_rgba: [[0; 4]; 32], // RGBA palette
            color_map: palette_map::get_color_map(),
        }
    }

    pub fn reset(&mut self) {
        self.cycle = 0;
        self.scanline = 0;
        self.frame_complete = false;
        self.control = 0x00; // Clear PPUCTRL
        self.mask = 0x00; // Clear PPUMASK
        self.status = 0x00; // Clear PPUSTATUS
        self.oam_addr = 0x00; // Reset OAM address
        //self.oam_data unchanged    // OAM typically not cleared on reset
        self.scroll_latch = false; // Reset scroll latch
        self.scroll_x = 0x00;
        self.scroll_y = 0x00;
        self.addr_latch = false; // Reset address latch
        self.vram_addr = 0x0000; // Clear VRAM address
        self.temp_addr = 0x0000; // Clear temp address
        self.fine_x = 0x00; // Clear fine X
        self.buffered_data = 0x00; // Clear read buffer
        //self.pallette unchanged    // Palette typically not cleared on reset
        //self.frame_buffer unchanged // Frame buffer typically not cleared
        self.nmi_pending = false; // Clear pending NMI
    }

    pub fn tick(&mut self) {
        self.cycle += 1;
        if self.scanline < 240 {
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
                if self.cycle % 8 == 0 && self.cycle <= 256 {
                    // Increment coarse X (bits 0-4)
                    if self.vram_addr & 0x001F == 0x001F {
                        // Coarse X == 31
                        self.vram_addr &= !0x001F; // Reset coarse X
                        self.vram_addr ^= 0x0400; // Flip nametable X (bit 10)
                    } else {
                        self.vram_addr += 1; // Increment coarse X
                    }
                }
                if (self.vram_addr & 0x7000) == 0x7000 {
                    // Fine Y == 7
                    self.vram_addr &= !0x7000; // Reset fine Y
                    let mut coarse_y = (self.vram_addr & 0x03E0) >> 5;
                    if coarse_y == 29 {
                        coarse_y = 0;
                        self.vram_addr ^= 0x0800; // Flip nametable Y (bit 11)
                    } else if coarse_y == 31 {
                        coarse_y = 0; // No flip if overflow beyond 29 (safety)
                    } else {
                        coarse_y += 1;
                    }
                    self.vram_addr = (self.vram_addr & !0x03E0) | (coarse_y << 5);
                } else {
                    self.vram_addr += 0x1000; // Increment fine Y (bit 12-14)
                }
            } else if self.cycle == 257 {
                // Reset horizontal bits of vram_addr from temp_addr (for next scanline's start)
                self.vram_addr = (self.vram_addr & 0x7BE0) | (self.temp_addr & 0x041F);
            } else if self.cycle == 328 {
            }
        } else if self.scanline == 240 {
        } else if self.scanline == 241 {
            if self.cycle == 1 {
                // Start of VBlank, set VBlank flag
                self.status |= 0x80; // Set VBlank flag
                self.frame_complete = true; // Indicate frame completion
                if (self.control & 0x80) != 0 {
                    // If NMI is enabled, trigger NMI
                    self.nmi_pending = true;
                    println!("NMI triggered at scanline 241, cycle 1");
                }
            }
        } else if self.scanline > 241 && self.scanline < 261 {
            // Post-render scanlines
        } else if self.scanline == 261 {
            if self.cycle == 1 {
                // End of VBlank, clear VBlank flag
                self.status &= 0x1f; // Clear VBlank and sprite 0 hit flags
            } else if self.cycle >= 280 && self.cycle < 304 {
                self.vram_addr = (self.vram_addr & 0x041F) | (self.temp_addr & 0x7BE0);
            } else if self.cycle == 257 {
                self.vram_addr = (self.vram_addr & 0x7BE0) | (self.temp_addr & 0x041F);
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

    pub fn get_nmi_pending(&self) -> bool {
        self.nmi_pending
    }

    pub fn set_nmi_pending(&mut self, status: bool) {
        self.nmi_pending = status;
    }

    fn read_vram(&mut self, addr: u16) -> Option<u8> {
        let mapped_addr = addr & 0x3FFF; // Mask to 14 bits
        if mapped_addr < 0x3F00 {
            // CHR ROM
            self.cartridge.borrow().mapper.ppu_read(mapped_addr)
        } else if mapped_addr >= 0x3F00 && mapped_addr < 0x4000 {
            // Palette memory
            let address = (mapped_addr - 0x3F00) % 0x20;
            self.palette.get(address as usize).copied()
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
            self.set_palette(address as usize, value);
        } else {
            // Invalid address
            eprintln!("PPU Write: Invalid address 0x{:04X}", addr);
        }
    }

    pub fn read_register(&mut self, addr: u16) -> Option<u8> {
        match addr & 0x2007 {
            0x2002 => {
                // PPUSTATUS
                let value = self.status;
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
                let value = self.read_vram(self.vram_addr);
                let result = if self.vram_addr & 0x3FFF >= 0x3F00 {
                    // Palette memory is imidiately read
                    value
                } else {
                    // Regular VRAM read is delayed
                    let buffered_value = self.buffered_data;
                    self.buffered_data = value.unwrap_or(0);
                    Some(buffered_value)
                };
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
                println!("PPUCTRL write: {:#04X}", value);
                let nmi_was_enabled = (self.control & 0x80) != 0;
                self.control = value;
                self.temp_addr = (self.temp_addr & 0xF3FF) | (((value as u16) & 0x03) << 10);

                let nmi_is_enabled = (self.control & 0x80) != 0;
                if !nmi_was_enabled && nmi_is_enabled && (self.status & 0x80) != 0 {
                    self.nmi_pending = true;
                    println!("NMI triggered immediately on $2000 write");
                }
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
                println!("PPUDATA write: {:#04X} to {:#04X}", value, addr);
                self.write_vram(addr, value);
                self.vram_addr =
                    self.vram_addr
                        .wrapping_add(if self.control & 0x04 != 0 { 32 } else { 1 });
            }
            _ => {}
        }
    }

    pub fn get_palette(&self) -> &[u8; 32] {
        &self.palette
    }

    pub fn get_nes_color(&self, index: u8) -> [u8; 4] {
        self.color_map
            .get(&index)
            .cloned()
            .unwrap_or([0, 0, 0, 255])
    }

    pub fn set_palette(&mut self, index: usize, value: u8) {
        if index < self.palette.len() {
            self.palette[index] = value;
            self.palette_rgba[index] = self.get_nes_color(value);
        } else {
            eprintln!("PPU set_palette: Index {} out of bounds", index);
        }
    }

    pub fn get_palette_rgba(&self) -> [[u8; 4]; 32] {
        self.palette_rgba
    }
}
