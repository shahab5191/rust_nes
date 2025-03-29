#[derive(Debug)]
pub struct Memory {
    mem: [u8; 65536],
}

impl Memory {
    pub fn new() -> Self {
        let mut temp = Memory { mem: [0; 65536] };
        temp.mem[0] = 0b00000100;
        temp.mem[1] = 0b00000000;
        temp.mem[2] = 0b00000000;
        temp.mem[3] = 0b00000111;
        temp
    }

    pub fn read(&self, address: u16) -> u8 {
        let value = self.mem[address as usize];
        println!("Read: {0:08b}: {1:08b}", address, value);
        value
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.mem[address as usize] = value;
        println!("Write: {0:08b}: {1:08b}", address, value);
    }

    pub fn dump_zero_page(&self) {
        for i in 0..255 {
            println!("{0:08b}: {1:08b}", i, self.mem[i as usize])
        }
    }

    pub fn dump_stack(&self) {
        for i in 0..255 {
            println!(
                "{0:08b}: {1:08b}",
                i + 0x100,
                self.mem[(i + 0x100) as usize]
            )
        }
    }
}
