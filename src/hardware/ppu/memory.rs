use crate::hardware::bus::Bus;

#[derive(Debug, Clone, Copy)]
pub struct Memory {
    nametable: [u8; 4096],
    palette: [u8; 32],
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            nametable: [0; 4096],
            palette: [0; 32],
        }
    }

    pub fn write_chr(&self, address: u16, value: u8, mut bus: Bus) {
        bus.cartridge_ppu_write(address, value);
    }

    pub fn read_nametable(&self, address: u16, mut bus: &Bus) -> Option<u8> {
        bus.cartridge_ppu_read(address)
    }

    pub fn write_nametable(&mut self, address: u16, value: u8) {
        self.nametable[address as usize] = value;
    }

    pub fn read_palette(&self, address: u16) -> u8 {
        self.palette[address as usize]
    }

    pub fn write_palette(&mut self, address: u16, value: u8) {
        self.palette[address as usize] = value;
    }

    pub fn read(&self, address: u16) -> u8 {
        if address < 0x2000 {
            self.read_chr(address)
        } else if address < 0x3000 {
            self.read_nametable(address - 0x2000)
        } else if address < 0x3F00 {
            // Unused area, return 0
            0
        } else if address < 0x4000 {
            // Handle Mirroring
            // For NES, the palette is mirrored every 32 bytes
            let address = (address - 0x3F00) % 0x20;
            self.read_palette(address - 0x3F00)
        } else {
            panic!("Invalid memory read at address: {:#X}", address);
        }
    }
}
