pub mod instructions;

#[derive(Debug)]
pub enum Registers {
    A,
    X,
    Y,
    S,
    P,
}

pub struct CPU {
    a: u8,
    x: u8,
    y: u8,
    s: u8,
    p: u8,
    pc: u16,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            a: 0,
            x: 0,
            y: 0,
            s: 0,
            p: 0b1000,
            pc: 0xFFFC,
        }
    }

    pub fn interpret(&mut self, program: Vec<u8>) {}

    fn set_status(&mut self, bit: u8, state: bool) {
        let mask = (1 << bit) & ((state as u8) << bit);
        self.p |= mask;
        println!("Status: {0:010b}", self.p);
    }
    fn get_status(&self, bit: u8) -> u8 {
        ((1 << bit) ^ (self.p)) >> bit
    }

    pub fn set_carry(&mut self, state: bool) {
        println!("Setting carry to: {0}", state);
        self.set_status(0, state);
    }
    pub fn set_zero(&mut self, state: bool) {
        println!("Setting zero to: {0}", state);
        self.set_status(1, state);
    }
    pub fn set_interrupt_disable(&mut self, state: bool) {
        println!("Setting interrupt disable to: {0}", state);
        self.set_status(2, state);
    }
    pub fn set_decimal(&mut self, state: bool) {
        println!("Setting decimal to: {0}", state);
        self.set_status(3, state);
    }
    pub fn set_overflow(&mut self, state: bool) {
        println!("Setting overflow to: {0}", state);
        self.set_status(6, state);
    }
    pub fn set_negative(&mut self, state: bool) {
        println!("Setting negative to: {0}", state);
        self.set_status(7, state);
    }

    pub fn get_carry(&self) -> u8 {
        let flag = self.get_status(0);
        println!("Getting carry: {0}", flag);
        flag
    }
    pub fn get_zero(&self) -> u8 {
        let flag = self.get_status(1);
        println!("Getting zero: {0}", flag);
        flag
    }
    pub fn get_interrupt_disable(&self) -> u8 {
        let flag = self.get_status(2);
        println!("Getting interrupt disable: {0}", flag);
        flag
    }
    pub fn get_decimal(&self) -> u8 {
        let flag = self.get_status(3);
        println!("Getting decimal: {0}", flag);
        flag
    }
    pub fn get_overflow(&self) -> u8 {
        let flag = self.get_status(6);
        println!("Getting overflow: {0}", flag);
        flag
    }
    pub fn get_negative(&self) -> u8 {
        let flag = self.get_status(7);
        println!("Getting negative: {0}", flag);
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
        println!("Register({0:?}): {1:010b}", register, value);
        value
    }

    pub fn set(&mut self, register: Registers, value: u8) {
        println!("Setting register: {:?}", register);
        match register {
            Registers::X => self.x = value,
            Registers::Y => self.y = value,
            Registers::A => self.a = value,
            Registers::S => self.s = value,
            Registers::P => self.p = value,
        };
        println!("{0:?}: {1:010b}", register, value);
    }

    pub fn get_counter(&self) -> u16 {
        println!("Counter: {0:010b}", self.pc);
        self.pc
    }
    pub fn setg_counter(&mut self, value: u16) {
        println!("Setting counter: {0:010b}", value);
        self.pc = value;
        println!("Counter: {0:010b}", self.pc);
    }
    pub fn print(&self) {
        println!("x: {0:010b}", self.x);
        println!("y: {0:010b}", self.y);
        println!("accumulator: {0:010b}", self.a);
        println!("stack: {0:010b}", self.s);
        println!("status: {0:010b}", self.p);
        println!("counter: {0:010b}", self.pc);
    }
}
