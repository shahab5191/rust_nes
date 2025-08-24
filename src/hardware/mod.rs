mod bus;
mod cartridge;
mod cpu;
pub mod enums;
mod memory;
mod ppu;
use Result;
use cpu::opcode::{self};
use std::io;

pub struct Hardware {
    bus: bus::Bus,
    cpu_cycles: u32,
}

impl Hardware {
    pub fn new() -> Self {
        Self {
            bus: bus::Bus::new(),
            cpu_cycles: 0,
        }
    }

    pub fn step(&mut self, log: bool) -> Result<u32, io::Error> {
        // Execute a single CPU instruction
        let delayed_interrupt = match self.bus.cpu.delayed_interrupt {
            Some(true) => {
                self.bus.cpu.delayed_interrupt = None;
                true
            }
            _ => false,
        };
        let cycles = if self.bus.ppu.get_nmi_pending() && !delayed_interrupt {
            self.bus.ppu.set_nmi_pending(false);
            let nmi_vector = self.bus.read_word(0xFFFA) as u16;
            self.bus.cpu.nmi(nmi_vector)
        } else {
            // Fetch and decode instruction
            let opcode = self.bus.read_instruct();
            // get time spent on this part
            let instruction = match opcode::get_instruction(opcode) {
                Some(instr) => instr,
                None => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!(
                            "Invalid opcode [0x{:04X}]: 0x{:02X}",
                            self.bus.cpu.get_counter(),
                            opcode
                        ),
                    ));
                }
            };

            if log {
                let (instruct, _) = self
                    .bus
                    .create_disassembled_line(self.bus.cpu.get_counter());
                println!("[0x{:04X}] {}", self.bus.cpu.get_counter(), instruct);
            }

            (instruction.execute)(&mut self.bus, instruction.address_mode)
        };
        self.bus.cpu.delayed_interrupt = None;
        self.cpu_cycles += cycles as u32;

        let cycle_count = cycles * 3;

        // Run PPU ticks
        for _ in 0..cycle_count {
            self.bus.ppu.tick();
        }
        if self.cpu_cycles >= 29780 {
            // Reset CPU cycles after a frame
            let cycle = self.cpu_cycles;
            self.cpu_cycles = 0;
            self.bus.ppu.frame_complete = false;
            return Ok(cycle);
        }
        Ok(self.cpu_cycles)
    }

    pub fn tick(&mut self) -> Result<(), io::Error> {
        // Execute a single CPU instruction and update PPU
        loop {
            // Update the PPU state
            if self.step(false)? >= 29780 {
                return Ok(());
            }
        }
    }

    pub fn get_memory_dump(&self, start: usize, size: usize) -> String {
        let mut dumped_mem_str = String::new();
        for i in start..size {
            let byte = self
                .bus
                .cartridge
                .borrow()
                .mapper
                .cpu_read(i as u16)
                .unwrap_or(0);
            if i % 32 == 0 {
                if i != 0 {
                    dumped_mem_str.push('\n');
                }
                dumped_mem_str.push_str(&format!("{:04X}: ", i));
            }
            dumped_mem_str.push_str(&format!("{:02X} ", byte));
        }
        dumped_mem_str
    }

    pub fn get_assembly(&self, count: u16) -> (Vec<String>, u16) {
        let pc: u16 = self.bus.cpu.get_counter();
        let mut asm: Vec<String> = Vec::new();
        let mut line: u16 = pc;
        let current_line = asm.len() as u16;
        while asm.len() < count as usize {
            let (instruct, size) = self.bus.create_disassembled_line(line);
            asm.push(instruct);
            line = line.wrapping_add(size as u16);
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

    pub fn get_flag(&self, flag: enums::Flags) -> u8 {
        match flag {
            enums::Flags::Carry => self.bus.cpu.get_carry(),
            enums::Flags::Zero => self.bus.cpu.get_zero(),
            enums::Flags::InterruptDisable => self.bus.cpu.get_interrupt_disable(),
            enums::Flags::DecimalMode => self.bus.cpu.get_decimal_mode(),
            enums::Flags::BreakCommand => self.bus.cpu.get_break(),
            enums::Flags::Overflow => self.bus.cpu.get_overflow(),
            enums::Flags::Negative => self.bus.cpu.get_negative(),
        }
    }

    pub fn get_cycle(&self) -> u32 {
        self.cpu_cycles
    }

    pub fn get_palette(&self) -> [[u8; 4]; 32] {
        self.bus.ppu.get_palette_rgba()
    }
}
