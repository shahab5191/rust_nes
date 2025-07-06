pub mod instructions;
pub mod opcode;

#[derive(Debug)]
pub enum Registers {
    A,
    X,
    Y,
    S,
    P,
}

#[derive(Debug, Clone, Copy)]
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
        println!("Status: {0:08b}", self.p);
    }

    fn get_status(&self, bit: u8) -> u8 {
        ((1 << bit) & (self.p)) >> bit
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
    pub fn set_break(&mut self, state: bool) {
        println!("Setting break to: {0}", state);
        self.set_status(4, state);
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
    pub fn get_break(&self) -> u8 {
        let flag = self.get_status(4);
        println!("Getting break: {0}", flag);
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
        println!("Register({0:?}): {1:08b}", register, value);
        value
    }

    pub fn set(&mut self, register: Registers, value: u8) {
        println!("Setting register: {:?}", register);
        match register {
            Registers::X => {
                self.x = value;
                println!("Register({0:?}): {1:08b}", register, self.x);
            }
            Registers::Y => {
                self.y = value;
                println!("Register({0:?}): {1:08b}", register, self.y);
            }
            Registers::A => {
                self.a = value;
                println!("Register({0:?}): {1:08b}", register, self.a);
            }
            Registers::S => {
                self.s = value;
                println!("Register({0:?}): {1:08b}", register, self.s);
            }
            Registers::P => {
                self.p = value;
                println!("Register({0:?}): {1:08b}", register, self.p);
            }
        };
    }

    pub fn get_counter(&self) -> u16 {
        println!("Counter: {0:08b}", self.pc);
        self.pc
    }
    pub fn set_counter(&mut self, value: u16) {
        println!("Setting counter: {0:08b}", value);
        self.pc = value;
        println!("Counter: {0:08b}", self.pc);
    }
    pub fn dump_registers(&self) {
        println!("x: {0:08b}", self.x);
        println!("y: {0:08b}", self.y);
        println!("accumulator: {0:08b}", self.a);
        println!("stack: {0:08b}", self.s);
        println!("status: {0:08b}", self.p);
        println!("counter: {0:b}", self.pc);
    }
}
