use super::{
    cpu::{CPU, Registers, instructions::AddressMode},
    memory::Memory,
};

pub struct Bus {
    pub cpu: CPU,
    pub memory: Memory,
}

impl Bus {
    pub fn new() -> Self {
        let bus = Bus {
            cpu: CPU::new(),
            memory: Memory::new(),
        };
        return bus;
    }

    pub fn read(&self, address: u16) -> u8 {
        self.memory.read(address)
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.memory.write(address, value)
    }

    pub fn read_address_with_mode(&self, address_mode: AddressMode, operand: u16) -> u8 {
        match address_mode {
            AddressMode::Implicit => 0,
            AddressMode::Immidiate => operand as u8,
            AddressMode::Relative => 0,
            AddressMode::Accumulator => self.cpu.get(Registers::A),
            AddressMode::IndexedIndirect => {
                let zero_page_pointer = (operand + (self.cpu.get(Registers::X) as u16)) % 256;
                let low_byte = self.memory.read(zero_page_pointer);
                let high_byte = self.memory.read(zero_page_pointer + 1);
                let address: u16 = (high_byte as u16) << 8 + (low_byte as u16);
                self.read(address)
            }
            AddressMode::IndirectIndexed => {
                let zero_page_pointer = operand % 256;
                let low_byte = self.memory.read(zero_page_pointer);
                let high_byte = self.memory.read(zero_page_pointer + 1);
                let address = (high_byte as u16)
                    << 8 + (low_byte as u16) + (self.cpu.get(Registers::Y) as u16);
                self.memory.read(address)
            }
            AddressMode::Absolute => self.memory.read(operand),
            AddressMode::AbsoluteX => {
                let address = operand + (self.cpu.get(Registers::X) as u16);
                self.memory.read(address)
            }
            AddressMode::AbsoluteY => {
                let address = operand + (self.cpu.get(Registers::Y) as u16);
                self.memory.read(address)
            }
            AddressMode::ZeroPage => self.memory.read(operand),
            AddressMode::ZeroPageX => {
                let address = (operand + (self.cpu.get(Registers::X) as u16)) % 256;
                self.read(address)
            }
            AddressMode::ZeroPageY => {
                let address = (operand + (self.cpu.get(Registers::Y) as u16)) % 256;
                self.read(address)
            }
        }
    }
}
