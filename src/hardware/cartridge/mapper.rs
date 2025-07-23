use std::fmt::Debug;

pub trait Mapper: Debug {
    fn cpu_read(&self, addr: u16) -> Option<u8>;
    fn cpu_write(&mut self, addr: u16, value: u8) -> bool;

    fn ppu_read(&self, addr: u16) -> Option<u8>;
    fn ppu_write(&mut self, addr: u16, value: u8) -> bool;

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
    chr: Vec<u8>,
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
    // === CPU ===
    fn cpu_read(&self, addr: u16) -> Option<u8> {
        match addr {
            0x6000..=0x7FFF => Some(self.prg_ram[(addr - 0x6000) as usize]),
            0x8000..=0xFFFF => {
                let index = if self.prg_rom.len() == 0x4000 {
                    // 16KB mirrored
                    (addr - 0x8000) as usize % 0x4000
                } else {
                    (addr - 0x8000) as usize
                };
                Some(self.prg_rom[index])
            }
            _ => None,
        }
    }

    fn cpu_write(&mut self, addr: u16, value: u8) -> bool {
        match addr {
            0x6000..=0x7FFF => {
                self.prg_ram[(addr - 0x6000) as usize] = value;
                true
            }
            0x8000..=0xFFFF => {
                // ROM: ignore writes
                false
            }
            _ => false,
        }
    }

    // === PPU ===

    fn ppu_read(&self, addr: u16) -> Option<u8> {
        match addr {
            0x0000..=0x1FFF => Some(self.chr[addr as usize]),
            _ => None,
        }
    }

    fn ppu_write(&mut self, addr: u16, value: u8) -> bool {
        match addr {
            0x0000..=0x1FFF => {
                if self.chr_is_ram {
                    self.chr[addr as usize] = value;
                    true
                } else {
                    false // CHR ROM is read-only
                }
            }
            _ => false,
        }
    }

    fn box_clone(&self) -> Box<dyn Mapper> {
        Box::new(self.clone())
    }
}
