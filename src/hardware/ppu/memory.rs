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

    pub fn read_nametable(&self, address: u16, mut bus: &Bus) -> Option<u8> {
        self.nametable(address)
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
}
