mod cpu;
mod memory;
use cpu::CPU;
use memory::Memory;

pub struct Hardware {
    cpu: CPU,
}

impl Hardware {
    pub fn new() -> Self {
        Memory::new();
        Hardware { cpu: CPU::new() }
    }

    pub fn test_hardware(mut self) {
        self.cpu.interpret(vec![0, 0, 0]);
    }
}
