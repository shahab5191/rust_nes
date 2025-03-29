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

    pub fn increment_pc(&mut self, address_mode: &AddressMode) {
        let pc = self.cpu.get_counter();
        let value = match address_mode {
            AddressMode::Immidiate => 2,
            AddressMode::IndexedIndirect => 2,
            AddressMode::IndirectIndexed => 2,
            AddressMode::Indirect => 3,
            AddressMode::ZeroPage => 2,
            AddressMode::ZeroPageX => 2,
            AddressMode::ZeroPageY => 2,
            AddressMode::Absolute => 3,
            AddressMode::AbsoluteX => 3,
            AddressMode::AbsoluteY => 3,
            AddressMode::Implicit => 1,
            AddressMode::Accumulator => 1,
            AddressMode::Relative => 1,
        };
        println!("Increament pc by {}", value);
        let new_pc = pc.wrapping_add(value);
        self.cpu.set_counter(new_pc);
    }

    pub fn increment_pc_by(&mut self, value: u16) {
        let pc = self.cpu.get_counter();
        self.cpu.set_counter(pc + value);
    }

    pub fn read(&self, address: u16) -> u8 {
        self.memory.read(address)
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.memory.write(address, value)
    }

    pub fn stack_push(&mut self, value: u8) {
        let sp = self.cpu.get(Registers::S);
        self.cpu.set(Registers::S, sp - 1);
        self.write(0x100 + (sp - 1) as u16, value);
    }

    pub fn stack_pull(&mut self) -> u8 {
        let sp = self.cpu.get(Registers::S);
        self.cpu.set(Registers::S, sp + 1);
        self.read(0x100 + sp as u16)
    }

    pub fn stack_push_word(&mut self, value: u16) {
        let low_byte = (value & 0x00FF) as u8;
        let high_byte = ((value & 0xFF00) >> 8) as u8;
        self.stack_push(high_byte);
        self.stack_push(low_byte);
    }

    pub fn stack_pull_word(&mut self) -> u16 {
        let low_byte = self.stack_pull();
        let high_byte = self.stack_pull();
        (high_byte as u16) << 8 + (low_byte as u16)
    }

    pub fn read_address_with_mode(&self, address_mode: AddressMode, operand: u16) -> (u8, u16) {
        match address_mode {
            AddressMode::Implicit => (0, 0),
            AddressMode::Immidiate => (operand as u8, 0),
            AddressMode::Relative => (0, 0),
            AddressMode::Accumulator => (self.cpu.get(Registers::A), 0),
            AddressMode::Indirect => {
                let low_byte = self.memory.read(operand);
                let high_byte = self.memory.read(operand + 1);
                let address: u16 = ((high_byte as u16) << 8) + (low_byte as u16);
                (self.read(address), address)
            }
            AddressMode::IndexedIndirect => {
                let zero_page_pointer = (operand + (self.cpu.get(Registers::X) as u16)) % 256;
                let low_byte = self.memory.read(zero_page_pointer);
                let high_byte = self.memory.read(zero_page_pointer + 1);
                let address: u16 = (high_byte as u16) << 8 + (low_byte as u16);
                (self.read(address), address)
            }
            AddressMode::IndirectIndexed => {
                let zero_page_pointer = operand % 256;
                let low_byte = self.memory.read(zero_page_pointer);
                let high_byte = self.memory.read(zero_page_pointer + 1);
                let address = (high_byte as u16)
                    << 8 + (low_byte as u16) + (self.cpu.get(Registers::Y) as u16);
                (self.read(address), address)
            }
            AddressMode::Absolute => (self.memory.read(operand), operand),
            AddressMode::AbsoluteX => {
                let address = operand + (self.cpu.get(Registers::X) as u16);
                (self.memory.read(address), address)
            }
            AddressMode::AbsoluteY => {
                let address = operand + (self.cpu.get(Registers::Y) as u16);
                (self.memory.read(address), address)
            }
            AddressMode::ZeroPage => (self.memory.read(operand), operand),
            AddressMode::ZeroPageX => {
                let address = (operand + (self.cpu.get(Registers::X) as u16)) % 256;
                (self.read(address), address)
            }
            AddressMode::ZeroPageY => {
                let address = (operand + (self.cpu.get(Registers::Y) as u16)) % 256;
                (self.read(address), address)
            }
        }
    }
}
