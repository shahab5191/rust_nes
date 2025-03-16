use instructions::AddressMode;

pub mod instructions;

pub struct CPU {
    a: u8,
    x: u8,
    y: u8,
    s: u8,
    status: u8,
    pc: u16,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            a: 0,
            x: 0,
            y: 0,
            s: 0,
            status: 0b1000,
            pc: 0xFFFC,
        }
    }

    pub fn interpret(&mut self, program: Vec<u8>) {}

    fn set_status(&mut self, bit: u8, state: bool) {
        let mask = (1 << bit) & ((state as u8) << bit);
        self.status |= mask;
    }
    fn get_status(&self, bit: u8) -> u8 {
        ((1 << bit) ^ (self.status)) >> bit
    }

    pub fn set_carry(&mut self, state: bool) {
        self.set_status(0, state);
    }
    pub fn set_zero(&mut self, state: bool) {
        self.set_status(1, state);
    }
    pub fn set_interrupt_disable(&mut self, state: bool) {
        self.set_status(2, state);
    }
    pub fn set_decimal(&mut self, state: bool) {
        self.set_status(3, state);
    }
    pub fn set_overflow(&mut self, state: bool) {
        self.set_status(6, state);
    }
    pub fn set_negative(&mut self, state: bool) {
        self.set_status(7, state);
    }

    pub fn get_carry(&self) -> u8 {
        self.get_status(0)
    }
    pub fn get_zero(&self) -> u8 {
        self.get_status(1)
    }
    pub fn get_interrupt_disable(&self) -> u8 {
        self.get_status(2)
    }
    pub fn get_decimal(&self) -> u8 {
        self.get_status(3)
    }
    pub fn get_overflow(&self) -> u8 {
        self.get_status(6)
    }
    pub fn get_negative(&self) -> u8 {
        self.get_status(7)
    }

    pub fn get_address_with_mode(&self, address_mode: AddressMode, operand: u8) -> u16 {
        match address_mode {
            AddressMode::immidiate => 0,
            AddressMode::indexed_indirect => 1,
            AddressMode::indirect_indexed => 2,
            AddressMode::absolute_x => (operand as u16) + (self.x as u16),
            AddressMode::absolute_y => (operand as u16) + (self.y as u16),
            AddressMode::zero_page => operand as u16,
            AddressMode::zero_page_x => ((operand as u16) + (self.x as u16)) % 256,
            AddressMode::zero_page_y => ((operand as u16) + (self.x as u16)) % 256,
        }
    }
    pub fn print(&self) {
        println!("flags: {0:b}", self.status)
    }
}
