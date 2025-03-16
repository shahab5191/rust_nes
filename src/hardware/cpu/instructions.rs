use std::fmt::{self, Display, Formatter};

use crate::hardware::bus::Bus;

use super::Registers;

#[derive(PartialEq, Eq)]
pub enum AddressMode {
    Implicit,
    Immidiate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndexedIndirect,
    IndirectIndexed,
}

impl Display for AddressMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AddressMode::Implicit => write!(f, ""),
            AddressMode::Immidiate => write!(f, "#"),
            AddressMode::ZeroPage => write!(f, "$"),
            AddressMode::ZeroPageX => write!(f, "$"),
            AddressMode::ZeroPageY => write!(f, "$"),
            AddressMode::Absolute => write!(f, "$"),
            AddressMode::AbsoluteX => write!(f, "$"),
            AddressMode::AbsoluteY => write!(f, "$"),
            AddressMode::IndexedIndirect => write!(f, ""),
            AddressMode::IndirectIndexed => write!(f, ""),
        }
    }
}

fn format_with_address_mode(address_mode: &AddressMode, operand: u8) -> String {
    match *address_mode {
        AddressMode::Implicit => "".to_string(),
        AddressMode::Immidiate => format!("#{:X}", operand),
        AddressMode::ZeroPage => format!("${:X}", operand),
        AddressMode::ZeroPageX => format!("${:X},X", operand),
        AddressMode::ZeroPageY => format!("${:X},Y", operand),
        AddressMode::Absolute => format!("${:X}", operand),
        AddressMode::AbsoluteX => format!("${:X},X", operand),
        AddressMode::AbsoluteY => format!("${:X},Y", operand),
        AddressMode::IndexedIndirect => format!("(${:X},X)", operand),
        AddressMode::IndirectIndexed => format!("(${:X}),Y", operand),
    }
}

fn log_instruct(instruct: &str, address_mode: &AddressMode, operand: u8) {
    let address_format = format_with_address_mode(address_mode, operand);
    println!("{0} {1}", instruct, address_format);
}

pub fn adc(bus: &mut Bus, address_mode: AddressMode, operand: u8) {
    // Add with carry
    log_instruct("ADC", &address_mode, operand);
    let value = bus.read_address_with_mode(address_mode, operand as u16);
    let result: u16 =
        (value as u16) + (bus.cpu.get(Registers::A) as u16) + (bus.cpu.get_carry() as u16);
    bus.cpu.set_carry(result > 0xFF);
    bus.cpu.set_zero(result == 0);
    bus.cpu.set_overflow(
        ((bus.cpu.get(Registers::A) ^ result as u8) & (value ^ result as u8) & 0x80) != 0,
    );
    bus.cpu.set_negative(result & 0x80 != 0);
    bus.cpu.set(Registers::A, result as u8);
}

pub fn and(bus: &mut Bus, address_mode: AddressMode, operand: u8) {
    // Logical AND
    log_instruct("AND", &address_mode, operand);
    let value = bus.read_address_with_mode(address_mode, operand as u16);
    let result = bus.cpu.get(Registers::A) & value;
    bus.cpu.set_zero(result == 0);
    bus.cpu.set_negative(result & 0x80 != 0);
    bus.cpu.set(Registers::A, result);
}

pub fn asl(bus: &mut Bus, address_mode: AddressMode, operand: u8) {
    log_instruct("ASL", &address_mode, operand);
    let value = bus.read_address_with_mode(address_mode, operand as u16);
    let result = value << 1;
    bus.cpu.set_carry(value & 0x80 != 0);
    bus.cpu.set_zero(result == 0);
    bus.cpu.set_negative(result & 0x80 != 0);
    bus.cpu.set(Registers::A, result);
}
