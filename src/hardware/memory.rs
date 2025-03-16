#[derive(Debug)]
pub struct Memory {
    mem: [u8; 65536],
}

impl Memory {
    pub fn new() -> Self {
        let mut temp = Memory { mem: [0; 65536] };
        temp.mem[0x0000] = 0b00000001;
        temp
    }

    pub fn read(&self, address: u16) -> u8 {
        let value = self.mem[address as usize];
        println!("Read: {0:010b}: {1:010b}", address, value);
        value
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.mem[address as usize] = value;
        println!("Write: {0:010b}: {1:010b}", address, value);
    }

    pub fn dump_zero_page(&self) {
        for i in 0..255 {
            println!("{0:010b}: {1:010b}", i, self.mem[i as usize])
        }
    }
}
