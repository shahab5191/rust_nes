mod bus;
mod cpu;
mod memory;
use cpu::instructions::{self, AddressMode};

pub struct Hardware {
    bus: bus::Bus,
}

impl Hardware {
    pub fn new() -> Self {
        Self {
            bus: bus::Bus::new(),
        }
    }

    pub fn test_hardware(mut self) {
        instructions::adc(&mut self.bus, AddressMode::Immidiate, 0x01);
        instructions::asl(&mut self.bus, AddressMode::ZeroPageX, 0x02);
        instructions::and(&mut self.bus, AddressMode::Absolute, 0x02);
    }
}
