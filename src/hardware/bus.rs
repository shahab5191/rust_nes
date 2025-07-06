use std::collections::HashMap;

use super::{
    cpu::{CPU, Registers, instructions::AddressMode},
    memory::Memory,
};
pub struct ReadAddressWithModeResult {
    pub value: u8,
    pub address: u16,
    pub cycles: u8,
}

#[derive(Debug, Clone)]
pub struct Bus {
    pub cpu: CPU,
    pub memory: Memory,
    pub assembly: HashMap<u16, String>,
}

impl Bus {
    pub fn new() -> Self {
        let bus = Bus {
            cpu: CPU::new(),
            memory: Memory::new(),
            assembly: HashMap::new(),
        };
        return bus;
    }

    pub fn increment_pc(&mut self, address_mode: &AddressMode) {
        let pc = self.cpu.get_counter();
        let value = match address_mode {
            AddressMode::Immidiate => 2,
            AddressMode::IndirectX => 2,
            AddressMode::IndirectY => 2,
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

    pub fn read_instruct(&self) -> u8 {
        let pc = self.cpu.get_counter();
        self.memory.read(pc)
    }

    pub fn read_next(&self) -> u8 {
        let pc = self.cpu.get_counter();
        self.memory.read(pc.wrapping_add(1))
    }

    pub fn read_word(&self, address: u16) -> u16 {
        let low_byte = self.read(address);
        let high_byte = self.read(address.wrapping_add(1));
        ((high_byte as u16) << 8) + (low_byte as u16)
    }

    pub fn read_word_buggy(&self, address: u16) -> u16 {
        let low = self.read(address);
        let high = if address & 0x00FF == 0x00FF {
            println!("Buggy read at address {:#04x}", &address);
            println!("Reading high byte from address {:#04x}", address & 0xFF00);
            self.read(address & 0xFF00)
        } else {
            self.read(address.wrapping_add(1))
        };
        ((high as u16) << 8) | (low as u16)
    }

    pub fn read_next_word(&self) -> u16 {
        let pc = self.cpu.get_counter();
        self.read_word(pc.wrapping_add(1))
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.memory.write(address, value)
    }

    pub fn stack_push(&mut self, value: u8) {
        let new_sp = self.cpu.get(Registers::S).wrapping_sub(1);
        self.cpu.set(Registers::S, new_sp);
        self.write(0x100 + new_sp as u16, value);
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

    pub fn read_address_with_mode(&self, address_mode: &AddressMode) -> ReadAddressWithModeResult {
        "
        Reads the value from memory based on the address mode and operand.
        args:
            address_mode: The addressing mode to use for reading.
            operand: The operand to use in conjunction with the address mode.
        returns:
            A tuple containing the value read from memory and the address it was read from.
        ";
        match address_mode {
            AddressMode::Implicit => {
                return ReadAddressWithModeResult {
                    value: self.cpu.get(Registers::A),
                    address: 0,
                    cycles: 0,
                };
            }
            AddressMode::Immidiate => ReadAddressWithModeResult {
                value: self.read_next(),
                address: 0,
                cycles: 0,
            },
            AddressMode::Relative => {
                let relative = self.read_next() as i32;
                let pc = (self.cpu.get_counter() as i32).wrapping_add(relative);
                ReadAddressWithModeResult {
                    value: 0,
                    address: pc as u16,
                    cycles: 0,
                }
            }

            AddressMode::Accumulator => ReadAddressWithModeResult {
                value: self.cpu.get(Registers::A),
                address: 0,
                cycles: 0,
            },
            AddressMode::Indirect => {
                let pointer: u16 = self.read_next_word();
                ReadAddressWithModeResult {
                    value: 0,
                    address: self.read_word_buggy(pointer),
                    cycles: 0,
                }
            }
            AddressMode::IndirectX => {
                let operand = self.read_next();
                let zero_page_pointer = operand.wrapping_add(self.cpu.get(Registers::X));
                // We don't use read_word because we have to wrap into zero page
                // when reading high byte. read_word first converts the address to
                // u16 then adds 1 to it, which is not what we want here.
                let low_byte = self.read(zero_page_pointer as u16);
                let high_byte = self.read(zero_page_pointer.wrapping_add(1) as u16);
                let address: u16 = (high_byte as u16) << 8 + (low_byte as u16);
                ReadAddressWithModeResult {
                    value: self.read(address),
                    address,
                    cycles: 0,
                }
            }
            AddressMode::IndirectY => {
                let operand = self.read_next() as u16;
                let zero_page_pointer = self.read_word(operand);
                let address = zero_page_pointer.wrapping_add(self.cpu.get(Registers::Y) as u16);
                let crossed_page = (zero_page_pointer & 0xFF00) != (address & 0xFF00);
                let mut cycles = 0;
                if crossed_page {
                    println!("Crossed page boundary in indirect indexed mode");
                    cycles = 1;
                }
                ReadAddressWithModeResult {
                    value: self.read(address),
                    address,
                    cycles,
                }
            }
            AddressMode::Absolute => {
                let address = self.read_next_word();
                ReadAddressWithModeResult {
                    value: self.memory.read(address),
                    address,
                    cycles: 0,
                }
            }
            AddressMode::AbsoluteX => {
                let operand = self.read_next_word();
                let address = operand.wrapping_add(self.cpu.get(Registers::X) as u16);
                let crossed_page = (operand & 0xFF00) != (address & 0xFF00);
                let mut cycles = 0;
                if crossed_page {
                    println!("Crossed page boundary in absolute X mode");
                    cycles = 1;
                }
                ReadAddressWithModeResult {
                    value: self.read(address),
                    address,
                    cycles,
                }
            }
            AddressMode::AbsoluteY => {
                let operand = self.read_next_word();
                let address = operand.wrapping_add(self.cpu.get(Registers::Y) as u16);
                let crossed_page = (operand & 0xFF00) != (address & 0xFF00);
                let mut cycles = 0;
                if crossed_page {
                    println!("Crossed page boundary in absolute Y mode");
                    cycles = 1;
                }
                ReadAddressWithModeResult {
                    value: self.memory.read(address),
                    address,
                    cycles,
                }
            }
            AddressMode::ZeroPage => {
                let address = self.read_next() as u16;
                println!("Zero page address: {:#04x}", address);
                let value = self.read(address);
                println!("Zero page value: {:#04x}", value);
                ReadAddressWithModeResult {
                    value,
                    address,
                    cycles: 0,
                }
            }
            AddressMode::ZeroPageX => {
                let operand = self.read_next() as u16;
                let address = operand.wrapping_add(self.cpu.get(Registers::X) as u16);
                ReadAddressWithModeResult {
                    value: self.read(address),
                    address,
                    cycles: 0,
                }
            }
            AddressMode::ZeroPageY => {
                let operand = self.read_next() as u16;
                let address = operand.wrapping_add(self.cpu.get(Registers::Y) as u16);
                ReadAddressWithModeResult {
                    value: self.read(address),
                    address,
                    cycles: 0,
                }
            }
        }
    }
}
