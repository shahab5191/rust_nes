mod bus;
mod cartridge;
mod cpu;
pub mod enums;
mod memory;
mod ppu;
use Result;
use cpu::opcode;
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
        for _ in 0..29_780 {
            // Handle delayed interrupt once per tick
            if matches!(self.bus.cpu.delayed_interrupt, Some(true)) {
                // TODO: Properly handle the delayed interrupt
                self.bus.cpu.delayed_interrupt = None;
            }

            // Fetch and decode instruction
            let opcode = self.bus.read_instruct();
            // get time spent on this part
            let instruction = match opcode::get_instruction(opcode) {
                Some(instr) => instr,
                None => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Invalid opcode: 0x{:02X}", opcode),
                    ));
                }
            };

            // Execute instruction and determine cycles
            let cycles = (instruction.execute)(&mut self.bus, instruction.address_mode);
            let cycle_count = cycles * 3;

            // Run PPU ticks
            for _ in 0..cycle_count {
                self.bus.ppu.tick();
                if self.bus.ppu.frame_complete {
                    self.bus.ppu.frame_complete = false;
                    break;
                }
            }
        }

        Ok(())
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

    pub fn get_assembly(&mut self, count: u16) -> (Vec<String>, u16) {
        let pc: u16 = self.bus.cpu.get_counter();
        let mut asm: Vec<String> = Vec::new();
        let mut line: u16 = pc;
        let current_line = asm.len() as u16;
        while asm.len() < count as usize {
            let (instruct, size) = self.bus.create_disassembled_line(line);
            asm.push(instruct);
            line += size as u16;
        }
        (asm, current_line)
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
        self.bus.cartridge.borrow_mut().load_ines_rom(file_path)?;
        self.bus.reset();
        Ok(())
    }

    pub fn get_cpu_reg(&self, register: enums::Registers) -> u8 {
        self.bus.cpu.get(register)
    }

    pub fn get_pc(&self) -> u16 {
        self.bus.cpu.get_counter()
    }
}
