mod bus;
mod cpu;
mod memory;
use cpu::instructions::{self, AddressMode};
use cpu::opcode;

pub struct Hardware {
    bus: bus::Bus,
}

impl Hardware {
    pub fn new() -> Self {
        Self {
            bus: bus::Bus::new(),
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
    }

    pub fn run(&mut self) {
        println!("Starting CPU execution");
        loop {
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
                break;
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
        }
    }
}
