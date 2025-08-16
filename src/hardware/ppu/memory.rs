#[derive(Debug, Clone)]
pub struct Memory {
    palette: [u8; 32],
    frame_buffer: Vec<u8>,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            palette: [0; 32],
            frame_buffer: vec![0; 256 * 240 * 4],
        }
    }

    pub fn read_palette(&self, address: u16) -> Option<u8> {
        self.palette.get(address as usize).copied()
    }

    pub fn write_palette(&mut self, address: u16, value: u8) {
        self.palette[address as usize] = value;
    }
}
