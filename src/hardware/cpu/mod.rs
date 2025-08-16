use super::enums::Registers;

pub mod instructions;
pub mod opcode;

#[derive(Default, Debug, Clone, Copy)]
pub struct CPU {
    a: u8,
    x: u8,
    y: u8,
    s: u8,
    p: u8,
    pc: u16,
    pub delayed_interrupt: Option<bool>,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            a: 1,
            x: 0,
            y: 0,
            s: 0,
            p: 0b1000,
            pc: 0xFFFC,
            delayed_interrupt: None,
        }
    }

    pub fn interpret(&mut self, program: Vec<u8>) {}

    fn set_status(&mut self, bit: u8, state: bool) {
        self.p &= !(1 << bit); // Clear the bit
        self.p |= (state as u8) << bit;
    }

    fn get_status(&self, bit: u8) -> u8 {
        ((1 << bit) & (self.p)) >> bit
    }

    pub fn set_carry(&mut self, state: bool) {
        self.set_status(0, state);
    }
    pub fn set_zero(&mut self, state: bool) {
        self.set_status(1, state);
    }
    pub fn set_interrupt_disable(&mut self, state: bool) {
        self.set_status(2, state);
    }
    pub fn set_decimal(&mut self, state: bool) {
        self.set_status(3, state);
    }
    pub fn set_overflow(&mut self, state: bool) {
        self.set_status(6, state);
    }
    pub fn set_negative(&mut self, state: bool) {
        self.set_status(7, state);
    }
    pub fn set_break(&mut self, state: bool) {
        self.set_status(4, state);
    }

    pub fn get_carry(&self) -> u8 {
        let flag = self.get_status(0);
        flag
    }
    pub fn get_zero(&self) -> u8 {
        let flag = self.get_status(1);
        flag
    }
    pub fn get_interrupt_disable(&self) -> u8 {
        let flag = self.get_status(2);
        flag
    }
    pub fn get_decimal(&self) -> u8 {
        let flag = self.get_status(3);
        flag
    }
    pub fn get_overflow(&self) -> u8 {
        let flag = self.get_status(6);
        flag
    }
    pub fn get_negative(&self) -> u8 {
        let flag = self.get_status(7);
        flag
    }
    pub fn get_break(&self) -> u8 {
        let flag = self.get_status(4);
        flag
    }

    pub fn get(&self, register: Registers) -> u8 {
        let value = match register {
            Registers::X => self.x,
            Registers::Y => self.y,
            Registers::P => self.p,
            Registers::A => self.a,
            Registers::S => self.s,
        };
        value
    }

    pub fn set(&mut self, register: Registers, value: u8) {
        match register {
            Registers::X => {
                self.x = value;
            }
            Registers::Y => {
                self.y = value;
            }
            Registers::A => {
                self.a = value;
            }
            Registers::S => {
                self.s = value;
            }
            Registers::P => {
                self.p = value;
            }
        };
    }

    pub fn get_counter(&self) -> u16 {
        self.pc
    }
    pub fn set_counter(&mut self, value: u16) {
        self.pc = value;
    }
    pub fn dump_registers(&self) {
        println!("x: {0:08b}", self.x);
        println!("y: {0:08b}", self.y);
        println!("accumulator: {0:08b}", self.a);
        println!("stack: {0:08b}", self.s);
        println!("status: {0:08b}", self.p);
        println!("counter: {0:b}", self.pc);
    }

    pub fn reset(&mut self) {
        self.a = 0xFF;
        self.x = 0;
        self.y = 0;
        self.s = 0xFD; // Stack starts at 0xFD
        self.pc = 0x0;
        self.p = 0b00000100; // Set unused bit, clear others
        self.delayed_interrupt = None;
        println!("CPU reset.");
    }
}
