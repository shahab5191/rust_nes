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
        instructions::bit(&mut self.bus, AddressMode::ZeroPage, 0x00);
        self.bus.cpu.dump_registers();
    }
}
