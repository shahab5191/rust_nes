use crate::hardware::cpu::instructions::AddressMode;
use std::mem::offset_of;

#[derive(Debug)]
pub struct Memory {}

impl Memory {
    pub fn new() {
        println!("Hello from memory!")
    }

    pub fn get(&self, address: u16) -> u8 {
        0b01101001
    }
}
