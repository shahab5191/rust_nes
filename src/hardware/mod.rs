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
        instructions::jmp(&mut self.bus, AddressMode::Indirect, 0);
        self.bus.cpu.dump_registers();
        // self.bus.memory.dump_zero_page();
    }
}
