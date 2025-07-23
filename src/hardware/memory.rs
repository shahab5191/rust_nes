use std::collections::HashMap;

use super::cpu::{instructions::AddressMode, opcode};

#[derive(Debug, Clone)]
pub struct Memory {
    mem: [u8; 0x800],
    assembly: HashMap<u16, String>,
}

impl Memory {
    pub fn new() -> Self {
        let mut temp = Memory {
            mem: [0; 0x800],
            assembly: HashMap::new(),
        };
        temp.mem[0] = 0b00000100;
        temp.mem[1] = 0b00000000;
        temp.mem[2] = 0b00000000;
        temp.mem[3] = 0b00000111;
        temp
    }

    pub fn silent_read(&self, address: u16) -> u8 {
        let real_address = address % 0x7ff;
        self.mem[real_address as usize]
    }

    pub fn read(&self, address: u16) -> u8 {
        // Handling address mirroring in NES
        let real_address = address % 0x7ff;
        let value = self.mem[real_address as usize];
        println!("Read: {0:08b}: {1:08b}", address, value);
        value
    }

    pub fn read_word(&self, address: u16) -> u16 {
        let low = self.read(address);
        let high = self.read(address + 1);
        ((high as u16) << 8) | (low as u16)
    }

    fn get_parameters(&self, addr_mode: &AddressMode, addr: u16) -> (String, u16) {
        match addr_mode {
            AddressMode::Immidiate => (format!("#${:02X}", self.read(addr + 1)), 2),
            AddressMode::ZeroPage => (format!("${:02X}", self.read(addr + 1)), 2),
            AddressMode::ZeroPageX => (format!("${:02X}, X", self.read(addr + 1)), 2),
            AddressMode::ZeroPageY => (format!("${:02X}, Y", self.read(addr + 1)), 2),
            AddressMode::Absolute => (format!("${:04X}", self.read_word(addr + 1)), 3),
            AddressMode::AbsoluteX => (format!("${:04X}, X", self.read_word(addr + 1)), 3),
            AddressMode::AbsoluteY => (format!("${:04X}, Y", self.read_word(addr + 1)), 3),
            AddressMode::Indirect => (format!("(${:04X})", self.read_word(addr + 1)), 3),
            AddressMode::IndirectX => (format!("(${:02X}, X)", self.read(addr + 1)), 2),
            AddressMode::IndirectY => (format!("(${:02X}), Y", self.read(addr + 1)), 2),
            AddressMode::Relative => (format!("${:02X}", self.read(addr + 1)), 2),
            AddressMode::Implicit => (String::new(), 1),
            AddressMode::Accumulator => (String::from("A"), 1),
        }
    }

    fn create_disassembled_line(&self, address: u16) -> String {
        /// Create a disassembled line for the given address
        /// # Arguments
        /// * `address` - The address to disassemble
        /// # Returns
        /// * A string representing the disassembled instruction at the given address
        let opcode = self.mem[address as usize];
        let instruction = opcode::get_instruction(opcode);
        let disassembled;
        if let Some(instruction) = instruction {
            let (param, _) = self.get_parameters(&instruction.address_mode, address);
            disassembled = format!("{:04X}: {} {}", address, instruction.name, param);
        } else {
            disassembled = format!("{:04X}: {:02X} <unknown>", address, opcode);
        }
        disassembled
    }

    fn update_assembly(&mut self, address: u16) {
        if self.assembly.contains_key(&address) {
            self.assembly.remove(&address);
            let instruction = opcode::get_instruction(self.mem[address as usize]);
            if let Some(instruction) = instruction {
                let (_, size) = self.get_parameters(&instruction.address_mode, address);
                for i in 1..size {
                    let next_address = address.wrapping_add(i);
                    if self.assembly.contains_key(&next_address) {
                        self.assembly.remove(&next_address);
                    }
                }
            }
            self.assembly
                .insert(address, self.create_disassembled_line(address));
        }
    }

    pub fn set_assembly(&mut self, assembly: HashMap<u16, String>) {
        self.assembly = assembly;
    }

    pub fn get_assembly(&self, address: u16) -> Option<String> {
        self.assembly.get(&address).cloned()
    }

    pub fn insert_assembly(&mut self, address: u16) {
        if self.assembly.contains_key(&address) {
            self.assembly.remove(&address);
        }
        self.assembly
            .insert(address, self.create_disassembled_line(address));
    }

    pub fn assembly_contains_key(&self, address: u16) -> bool {
        self.assembly.contains_key(&address)
    }

    pub fn write(&mut self, address: u16, value: u8) {
        if address < 0x2000 {
            let real_address = address % 0x800;
            self.mem[real_address as usize] = value;
            println!("Write: {0:08b}: {1:08b}", address, value);
            self.update_assembly(real_address);
        } else {
            println!("Warning: Writing to non-ram address {:#04X}", address);
            return;
        };
    }

    pub fn get_memory_slice(&self) -> &[u8] {
        &self.mem
    }
}
