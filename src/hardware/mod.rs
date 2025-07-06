mod bus;
mod cpu;
mod memory;
use Result;
use cpu::instructions::{self, AddressMode};
use cpu::opcode;
use std::io;

#[derive(Debug, Clone)]
pub struct Hardware {
    bus: bus::Bus,
    assembly: Vec<String>,
}

impl Hardware {
    pub fn new() -> Self {
        Self {
            bus: bus::Bus::new(),
            assembly: Vec::new(),
        }
    }

    pub fn test_hardware(mut self) {
        self.bus.memory.write(0x00, 0x04);
        self.bus.memory.write(0x01, 0xff);
        self.bus.memory.write(0x02, 0x00);
        self.bus.cpu.set_counter(0x00);
        instructions::jmp(&mut self.bus, AddressMode::Indirect);
        self.bus.cpu.dump_registers();
        // self.bus.memory.dump_zero_page();
    }

    pub fn load_program(&mut self, program: Vec<u8>) {
        println!("Loading program into memory");
        for (i, byte) in program.iter().enumerate() {
            self.bus.memory.write(i as u16, *byte);
        }
        self.bus.cpu.set_counter(0x00);
        println!(
            "Program loaded, starting at address: {:#04x}",
            self.bus.cpu.get_counter()
        );
        self.disassemble()
    }

    pub fn tick(&mut self) -> Result<(), io::Error> {
        println!("Starting CPU execution");
        if self.bus.cpu.delayed_interrupt.is_some() {
            if let Some(true) = self.bus.cpu.delayed_interrupt {
                //TODO: Handle the delayed interrupts
                println!("Handling delayed interrupt");
                self.bus.cpu.delayed_interrupt = None;
            }
        }
        let opcode = self.bus.read_instruct();
        let instruction = opcode::get_instruction(opcode);
        if instruction.is_none() {
            println!("Unknown opcode: {:#04x}", opcode);
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Unknown opcode: {:#04x}", opcode),
            ));
        }
        let instruction = instruction.unwrap();
        let address_mode = instruction.address_mode;

        // Execute the instruction
        let cycles = (instruction.execute)(&mut self.bus, address_mode);

        // Handle cycles
        for _ in 0..cycles {
            // sleep
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
        return Ok(());
    }

    pub fn get_memory_dump(&self, start: usize, size: usize) -> String {
        let mem = &self.bus.memory.get_memory_slice();
        let end = (start + size).min(mem.len());
        let slice = &mem[start..end];

        let mut dumped_mem_str = String::new();
        for (i, byte) in slice.iter().enumerate() {
            if i % 32 == 0 {
                if i != 0 {
                    dumped_mem_str.push('\n');
                }
                dumped_mem_str.push_str(&format!("{:04X}: ", start + i));
            }
            dumped_mem_str.push_str(&format!("{:02X} ", byte));
        }
        dumped_mem_str
    }

    pub fn get_assembly(&self, count: u16) -> (Vec<String>, u16) {
        let line: u16 = self.bus.cpu.get_counter();
        let start: u16 = i32::max(0, line as i32 - (count / 2) as i32) as u16;
        let end = u16::min(start + count, 0x1FFF);
        let asm = self.assembly[start as usize..end as usize].to_vec();
        (asm, line)
    }

    fn disassemble(&mut self) {
        let mut disassembled: Vec<String> = Vec::new();
        let mut pc: u16 = 0;
        while pc < 0x1FFF {
            let opcode = self.bus.memory.silent_read(pc);
            let instruction = opcode::get_instruction(opcode);
            if let Some(instr) = instruction {
                let (param, size) = self.get_parameters(&instr.address_mode, pc);
                disassembled.push(format!("{:04X}: {:02X} {}\n", pc, opcode, param));
                pc += size;
            } else {
                disassembled.push(format!("{:04X}: {:02X} UNKNOWN\n", pc, opcode));
                pc += 1; // Increment by 1 for unknown opcodes
            }
        }
        self.assembly = disassembled;
    }

    fn get_parameters(&self, addr_mode: &AddressMode, addr: u16) -> (String, u16) {
        match addr_mode {
            AddressMode::Immidiate => (format!("#${:02X}", self.bus.read(addr + 1)), 2),
            AddressMode::ZeroPage => (format!("${:02X}", self.bus.read(addr + 1)), 2),
            AddressMode::ZeroPageX => (format!("${:02X}, X", self.bus.read(addr + 1)), 2),
            AddressMode::ZeroPageY => (format!("${:02X}, Y", self.bus.read(addr + 1)), 2),
            AddressMode::Absolute => (format!("${:04X}", self.bus.read_word(addr + 1)), 3),
            AddressMode::AbsoluteX => (format!("${:04X}, X", self.bus.read_word(addr + 1)), 3),
            AddressMode::AbsoluteY => (format!("${:04X}, Y", self.bus.read_word(addr + 1)), 3),
            AddressMode::Indirect => (format!("(${:04X})", self.bus.read_word(addr + 1)), 3),
            AddressMode::IndirectX => (format!("(${:02X}, X)", self.bus.read(addr + 1)), 2),
            AddressMode::IndirectY => (format!("(${:02X}), Y", self.bus.read(addr + 1)), 2),
            AddressMode::Relative => (format!("${:02X}", self.bus.read(addr + 1)), 2),
            AddressMode::Implicit => (String::new(), 1),
            AddressMode::Accumulator => (String::from("A"), 1),
        }
    }
}
