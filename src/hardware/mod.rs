mod bus;
mod cpu;
mod memory;
use Result;
use bus::Bus;
use cpu::instructions::AddressMode;
use cpu::opcode;
use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone)]
pub struct Hardware {
    bus: bus::Bus,
}

impl Hardware {
    pub fn new() -> Self {
        Self {
            bus: bus::Bus::new(),
        }
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
        let pc: u16 = self.bus.cpu.get_counter();
        let start: u16 = i32::max(0, pc as i32 - (count / 2) as i32) as u16;
        let end = u16::min(start + count, 0x1FFF);
        let mut asm: Vec<String> = Vec::new();
        let mut line: u16 = pc;
        while asm.len() < count as usize / 2 {
            if line == 0 {
                break;
            }
            line -= 1;

            if self.bus.assembly.contains_key(&line) {
                println!("Found assembly for line: {:#04x}", line);
                asm.insert(0, self.bus.assembly[&line].clone());
            }
        }
        line = pc;
        let current_line = asm.len() as u16;
        while asm.len() < count as usize {
            if line > 0x1FFF {
                break;
            }
            if self.bus.assembly.contains_key(&line) {
                println!("Found assembly for line: {:#04x}", line);
                asm.push(self.bus.assembly[&line].clone());
            }
            line += 1;
        }
        (asm, current_line)
    }

    fn disassemble(&mut self) {
        let mut disassembled: HashMap<u16, String> = HashMap::new();
        let mut pc: u16 = 0;
        while pc < 0x1FFF {
            let opcode = self.bus.memory.silent_read(pc);
            let instruction = opcode::get_instruction(opcode);
            if let Some(instr) = instruction {
                let (param, size) = Hardware::get_parameters(&self.bus, &instr.address_mode, pc);
                disassembled.insert(pc, format!("{:04X}: {:02X} {}\n", pc, opcode, param));
                pc += size;
            } else {
                disassembled.insert(pc, format!("{:04X}: {:02X} UNKNOWN\n", pc, opcode));
                pc += 1; // Increment by 1 for unknown opcodes
            }
        }
        self.bus.assembly = disassembled;
    }

    fn get_parameters(bus: &Bus, addr_mode: &AddressMode, addr: u16) -> (String, u16) {
        match addr_mode {
            AddressMode::Immidiate => (format!("#${:02X}", bus.read(addr + 1)), 2),
            AddressMode::ZeroPage => (format!("${:02X}", bus.read(addr + 1)), 2),
            AddressMode::ZeroPageX => (format!("${:02X}, X", bus.read(addr + 1)), 2),
            AddressMode::ZeroPageY => (format!("${:02X}, Y", bus.read(addr + 1)), 2),
            AddressMode::Absolute => (format!("${:04X}", bus.read_word(addr + 1)), 3),
            AddressMode::AbsoluteX => (format!("${:04X}, X", bus.read_word(addr + 1)), 3),
            AddressMode::AbsoluteY => (format!("${:04X}, Y", bus.read_word(addr + 1)), 3),
            AddressMode::Indirect => (format!("(${:04X})", bus.read_word(addr + 1)), 3),
            AddressMode::IndirectX => (format!("(${:02X}, X)", bus.read(addr + 1)), 2),
            AddressMode::IndirectY => (format!("(${:02X}), Y", bus.read(addr + 1)), 2),
            AddressMode::Relative => (format!("${:02X}", bus.read(addr + 1)), 2),
            AddressMode::Implicit => (String::new(), 1),
            AddressMode::Accumulator => (String::from("A"), 1),
        }
    }
}
