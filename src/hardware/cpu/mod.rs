use std::{cell::RefCell, rc::Rc};

use super::{bus::Bus, enums::Registers, memory::Memory};

pub mod instructions;
pub mod opcode;

#[derive(Default, Debug, Clone)]
pub struct CPU {
    a: u8,
    x: u8,
    y: u8,
    s: u8,
    p: u8,
    pc: u16,
    pub delayed_interrupt: Option<bool>,
    memory: Rc<RefCell<Memory>>,
}

impl CPU {
    pub fn new(memory: Rc<RefCell<Memory>>) -> Self {
        CPU {
            a: 1,
            x: 0,
            y: 0,
            s: 0,
            p: 0b1000,
            pc: 0xFFFC,
            delayed_interrupt: None,
            memory: memory,
        }
    }

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
    pub fn get_decimal_mode(&self) -> u8 {
        let flag = self.get_status(3);
        flag
    }
    pub fn get_interrupt_disable(&self) -> u8 {
        let flag = self.get_status(2);
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

    pub fn stack_push(&mut self, value: u8) {
        let new_sp = self.get(Registers::S).wrapping_sub(1);
        self.set(Registers::S, new_sp);
        self.memory.borrow_mut().write(0x100 + new_sp as u16, value);
    }

    pub fn stack_pull(&mut self) -> u8 {
        let sp = self.get(Registers::S);
        self.set(Registers::S, sp + 1);
        self.memory.borrow().read(0x100 + sp as u16)
    }

    pub fn stack_push_word(&mut self, value: u16) {
        let low_byte = (value & 0x00FF) as u8;
        let high_byte = ((value & 0xFF00) >> 8) as u8;
        self.stack_push(low_byte);
        self.stack_push(high_byte);
    }

    pub fn stack_pull_word(&mut self) -> u16 {
        let high_byte = self.stack_pull();
        let low_byte = self.stack_pull();
        ((high_byte as u16) << 8) + (low_byte as u16)
    }

    pub fn nmi(&mut self, nmi_vector: u16) -> u8 {
        // TODO: Fix NMI handling
        // this is not working correctly
        println!("NMI triggered at PC: {:#04X}", self.pc);
        self.stack_push_word(self.pc);

        let mut p = self.p;
        p &= !0x10;
        p |= 0x20;
        self.stack_push(p);

        self.set_interrupt_disable(true);
        println!("Reading NMI vector at 0xFFFA");
        println!("NMI vector: {:#04X}", nmi_vector);
        self.pc = nmi_vector;
        return 7;
    }
}
