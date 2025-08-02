mod bus;
mod cartridge;
mod cpu;
mod memory;
mod ppu;
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
        for _ in 0..(cycles * 3) {
            self.bus.ppu.tick();
            if self.bus.ppu.frame_complete {
                self.bus.ppu.frame_complete = false;
                println!("Frame complete");
            }
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
        let mut asm: Vec<String> = Vec::new();
        let mut line: u16 = pc;
        while asm.len() < count as usize / 2 {
            if line == 0 {
                break;
            }
            line -= 1;

            if self.bus.assembly_contains_key(line) {
                asm.insert(0, self.bus.get_assembly(line).unwrap());
            }
        }
        line = pc;
        let current_line = asm.len() as u16;
        while asm.len() < count as usize {
            if line > 0x1FFF {
                break;
            }
            if self.bus.assembly_contains_key(line) {
                asm.push(self.bus.get_assembly(line).unwrap());
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
                let (param, size) =
                    Hardware::get_parameters(&mut self.bus, &instr.address_mode, pc);
                disassembled.insert(pc, format!("{:04X}: {} {}", pc, instr.name, param));
                pc += size;
            } else {
                disassembled.insert(pc, format!("{:04X}: {:02X} UNKNOWN", pc, opcode));
                pc += 1; // Increment by 1 for unknown opcodes
            }
        }
        self.bus.set_assembly(disassembled);
    }

    fn get_parameters(bus: &mut Bus, addr_mode: &AddressMode, addr: u16) -> (String, u16) {
        match addr_mode {
            AddressMode::Immidiate => (format!("#${:02X}", bus.cpu_read(addr + 1)), 2),
            AddressMode::ZeroPage => (format!("${:02X}", bus.cpu_read(addr + 1)), 2),
            AddressMode::ZeroPageX => (format!("${:02X}, X", bus.cpu_read(addr + 1)), 2),
            AddressMode::ZeroPageY => (format!("${:02X}, Y", bus.cpu_read(addr + 1)), 2),
            AddressMode::Absolute => (format!("${:04X}", bus.read_word(addr + 1)), 3),
            AddressMode::AbsoluteX => (format!("${:04X}, X", bus.read_word(addr + 1)), 3),
            AddressMode::AbsoluteY => (format!("${:04X}, Y", bus.read_word(addr + 1)), 3),
            AddressMode::Indirect => (format!("(${:04X})", bus.read_word(addr + 1)), 3),
            AddressMode::IndirectX => (format!("(${:02X}, X)", bus.cpu_read(addr + 1)), 2),
            AddressMode::IndirectY => (format!("(${:02X}), Y", bus.cpu_read(addr + 1)), 2),
            AddressMode::Relative => (format!("${:02X}", bus.cpu_read(addr + 1)), 2),
            AddressMode::Implicit => (String::new(), 1),
            AddressMode::Accumulator => (String::from("A"), 1),
        }
    }

    pub fn get_chr_image(&mut self, table_number: u8) -> [u8; 128 * 128 * 4] {
        let mut image = [0; 128 * 128 * 4];
        let start_tile: u32 = if table_number == 0 { 0 } else { 256 };
        for tile_index in start_tile..start_tile + 256 {
            let tile_data = self.bus.read_tile(tile_index as u16);
            let color = self.bus.tile_to_rgb(tile_data);
            let tile_y = (tile_index - start_tile) / 16;
            let tile_x = (tile_index - start_tile) % 16;
            for row in 0..8 {
                for col in 0..8 {
                    let pixel_y = (tile_y * 8) + row;
                    let pixel_x = (tile_x * 8) + col;
                    let pixel_index = ((pixel_y * 128 + pixel_x) * 4) as usize;
                    let color_index = ((row * 8 + col) * 4) as usize;
                    image[pixel_index..pixel_index + 4]
                        .copy_from_slice(&color[color_index..color_index + 4]);
                }
            }
        }
        image
    }

    pub fn load_rom(&mut self, file_path: &str) -> Result<(), io::Error> {
        self.bus.cartridge.load_ines_rom(file_path)?;
        self.bus.reset();
        self.disassemble();
        Ok(())
    }
}
