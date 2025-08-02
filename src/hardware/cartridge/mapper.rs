use std::fmt::Debug;

pub trait Mapper: Debug {
    fn cpu_read(&self, addr: u16) -> Option<u8>;
    fn cpu_write(&mut self, addr: u16, value: u8) -> bool;

    fn ppu_read(&self, addr: u16) -> Option<u8>;
    fn ppu_write(&mut self, addr: u16, value: u8) -> bool;

    fn reset(&mut self);

    fn box_clone(&self) -> Box<dyn Mapper>;
}

impl Clone for Box<dyn Mapper> {
    fn clone(&self) -> Box<dyn Mapper> {
        self.box_clone()
    }
}

#[derive(Debug, Clone)]
pub struct Mapper0 {
    prg_rom: Vec<u8>,
    prg_ram: Vec<u8>,
    pub chr: Vec<u8>,
    chr_is_ram: bool,
}

impl Mapper0 {
    pub fn new(prg_rom: Vec<u8>, chr: Vec<u8>, chr_is_ram: bool) -> Self {
        let prg_ram = vec![0u8; 8 * 1024]; // 8KB default

        Self {
            prg_rom,
            prg_ram,
            chr,
            chr_is_ram,
        }
    }
}

impl Mapper for Mapper0 {
    fn cpu_read(&self, addr: u16) -> Option<u8> {
        match addr {
            // PRG RAM (Optional, typically 0x6000-0x7FFF)
            0x6000..=0x7FFF => {
                // Mask to get the offset within the 8KB block.
                let index = (addr - 0x6000) as usize;
                if index < self.prg_ram.len() {
                    Some(self.prg_ram[index])
                } else {
                    eprintln!("CPU Read: PRG RAM address 0x{:04X} out of bounds.", addr);
                    None
                }
            }
            // PRG ROM (0x8000-0xFFFF)
            0x8000..=0xFFFF => {
                // Mapper0 (NROM) can have 16KB or 32KB PRG ROM.
                // If 16KB, 0xC000-0xFFFF mirrors 0x8000-0xBFFF.
                // If 32KB, it's straight 0x8000-0xFFFF.

                let prg_rom_len = self.prg_rom.len();
                let mut mapped_addr = (addr - 0x8000) as usize;

                if prg_rom_len == 0x4000 {
                    // 16KB PRG ROM
                    // Mirror 0xC000-0xFFFF back to 0x8000-0xBFFF
                    mapped_addr %= 0x4000;
                }

                if mapped_addr < prg_rom_len {
                    Some(self.prg_rom[mapped_addr])
                } else {
                    eprintln!(
                        "CPU Read: PRG ROM address 0x{:04X} out of bounds (mapped to {}).",
                        addr, mapped_addr
                    );
                    None
                }
            }
            _ => {
                // Addresses outside cartridge memory space (handled by Bus)
                None
            }
        }
    }

    fn cpu_write(&mut self, addr: u16, value: u8) -> bool {
        match addr {
            // PRG RAM
            0x6000..=0x7FFF => {
                let index = (addr - 0x6000) as usize;
                if index < self.prg_ram.len() {
                    self.prg_ram[index] = value;
                    true // Write successful
                } else {
                    eprintln!("CPU Write: PRG RAM address 0x{:04X} out of bounds.", addr);
                    false
                }
            }
            // PRG ROM (Writes to ROM are ignored)
            0x8000..=0xFFFF => {
                eprintln!("CPU Write: Attempted to write to PRG ROM at 0x{:04X}", addr);
                false
            }
            _ => false, // Address outside cartridge memory space
        }
    }

    fn ppu_read(&self, addr: u16) -> Option<u8> {
        let index = (addr & 0x3FFF) as usize;
        if index < self.chr.len() {
            let data = self.chr[index];
            Some(data)
        } else {
            eprintln!("PPU Read: CHR address 0x{:04X} out of bounds.", addr);
            None
        }
    }

    fn ppu_write(&mut self, addr: u16, data: u8) -> bool {
        // PPU addresses 0x0000-0x1FFF map directly to CHR data on Mapper0.
        // Only allow writes if chr_is_ram is true.
        if self.chr_is_ram {
            let index = addr as usize;
            if index < self.chr.len() {
                self.chr[index] = data;
                true // Write successful
            } else {
                eprintln!("PPU Write: CHR RAM address 0x{:04X} out of bounds.", addr);
                false // Write failed
            }
        } else {
            eprintln!("PPU Write: Attempted to write to CHR-ROM at 0x{:04X}", addr);
            false // Write ignored
        }
    }

    fn box_clone(&self) -> Box<dyn Mapper> {
        Box::new(self.clone())
    }

    fn reset(&mut self) {
        self.prg_ram.fill(0);
        if self.chr_is_ram {
            self.chr.fill(0);
        }
    }
}
