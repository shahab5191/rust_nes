impl Default for Memory {
    fn default() -> Self {
        Memory::new()
    }
}

#[derive(Debug, Clone)]
pub struct Memory {
    mem: [u8; 0x800],
}

impl Memory {
    pub fn new() -> Self {
        let mut temp = Memory { mem: [0; 0x800] };
        temp.mem[0] = 0b00000100;
        temp.mem[1] = 0b00000000;
        temp.mem[2] = 0b00000000;
        temp.mem[3] = 0b00000111;
        temp
    }

    pub fn silent_read(&self, address: u16) -> u8 {
        let real_address = address % 0x7ff;
        self.mem[real_address as usize]
    }

    pub fn read(&self, address: u16) -> u8 {
        // Handling address mirroring in NES
        let real_address = address % 0x7ff;
        let value = self.mem[real_address as usize];
        value
    }

    pub fn read_word(&self, address: u16) -> u16 {
        let low = self.read(address);
        let high = self.read(address + 1);
        ((high as u16) << 8) | (low as u16)
    }

    pub fn write(&mut self, address: u16, value: u8) {
        if address < 0x2000 {
            let real_address = address % 0x800;
            self.mem[real_address as usize] = value;
        } else {
            println!("Warning: Writing to non-ram address {:#04X}", address);
            return;
        };
    }

    pub fn get_memory_slice(&self) -> &[u8] {
        &self.mem
    }

    pub fn reset(&mut self) {
        self.mem = [0; 0x800];
    }
}
