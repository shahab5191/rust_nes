use crate::hardware::bus;

use super::instructions::{self, AddressMode};

pub struct Instruction {
    pub name: &'static str,
    pub address_mode: AddressMode,
    pub execute: fn(&mut bus::Bus, address_mode: AddressMode) -> u8,
}

pub fn get_instruction(opcode: u8) -> Option<Instruction> {
    match opcode {
        0x00 => Some(Instruction {
            name: "BRK",
            address_mode: AddressMode::Implicit,
            execute: instructions::brk,
        }),
        0x01 => Some(Instruction {
            name: "ORA",
            address_mode: AddressMode::IndirectX,
            execute: instructions::ora,
        }),
        0x05 => Some(Instruction {
            name: "ORA",
            address_mode: AddressMode::ZeroPage,
            execute: instructions::ora,
        }),
        0x06 => Some(Instruction {
            name: "ASL",
            address_mode: AddressMode::ZeroPage,
            execute: instructions::asl,
        }),
        0x08 => Some(Instruction {
            name: "PHP",
            address_mode: AddressMode::Implicit,
            execute: instructions::php,
        }),
        0x09 => Some(Instruction {
            name: "ORA",
            address_mode: AddressMode::Immidiate,
            execute: instructions::ora,
        }),
        0x0A => Some(Instruction {
            name: "ASL",
            address_mode: AddressMode::Accumulator,
            execute: instructions::asl,
        }),
        0x0D => Some(Instruction {
            name: "ORA",
            address_mode: AddressMode::Absolute,
            execute: instructions::ora,
        }),
        0x0E => Some(Instruction {
            name: "ASL",
            address_mode: AddressMode::Absolute,
            execute: instructions::asl,
        }),
        0x10 => Some(Instruction {
            name: "BPL",
            address_mode: AddressMode::Relative,
            execute: instructions::bpl,
        }),
        0x11 => Some(Instruction {
            name: "ORA",
            address_mode: AddressMode::IndirectY,
            execute: instructions::ora,
        }),
        0x15 => Some(Instruction {
            name: "ORA",
            address_mode: AddressMode::ZeroPageX,
            execute: instructions::ora,
        }),
        0x16 => Some(Instruction {
            name: "ASL",
            address_mode: AddressMode::ZeroPageX,
            execute: instructions::asl,
        }),
        0x18 => Some(Instruction {
            name: "CLC",
            address_mode: AddressMode::Implicit,
            execute: instructions::clc,
        }),
        0x19 => Some(Instruction {
            name: "ORA",
            address_mode: AddressMode::AbsoluteY,
            execute: instructions::ora,
        }),
        0x1D => Some(Instruction {
            name: "ORA",
            address_mode: AddressMode::AbsoluteX,
            execute: instructions::ora,
        }),
        0x1E => Some(Instruction {
            name: "ASL",
            address_mode: AddressMode::AbsoluteX,
            execute: instructions::asl,
        }),
        0x20 => Some(Instruction {
            name: "JSR",
            address_mode: AddressMode::Absolute,
            execute: instructions::jsr,
        }),
        0x21 => Some(Instruction {
            name: "AND",
            address_mode: AddressMode::IndirectX,
            execute: instructions::and,
        }),
        0x24 => Some(Instruction {
            name: "BIT",
            address_mode: AddressMode::ZeroPage,
            execute: instructions::bit,
        }),
        0x25 => Some(Instruction {
            name: "AND",
            address_mode: AddressMode::ZeroPage,
            execute: instructions::and,
        }),
        0x26 => Some(Instruction {
            name: "ROL",
            address_mode: AddressMode::ZeroPage,
            execute: instructions::rol,
        }),
        0x28 => Some(Instruction {
            name: "PLP",
            address_mode: AddressMode::Implicit,
            execute: instructions::plp,
        }),
        0x29 => Some(Instruction {
            name: "AND",
            address_mode: AddressMode::Immidiate,
            execute: instructions::and,
        }),
        0x2A => Some(Instruction {
            name: "ROL",
            address_mode: AddressMode::Accumulator,
            execute: instructions::rol,
        }),
        0x2C => Some(Instruction {
            name: "BIT",
            address_mode: AddressMode::Absolute,
            execute: instructions::bit,
        }),
        0x2D => Some(Instruction {
            name: "AND",
            address_mode: AddressMode::Absolute,
            execute: instructions::and,
        }),
        0x2E => Some(Instruction {
            name: "ROL",
            address_mode: AddressMode::Absolute,
            execute: instructions::rol,
        }),
        0x30 => Some(Instruction {
            name: "BMI",
            address_mode: AddressMode::Relative,
            execute: instructions::bmi,
        }),
        0x31 => Some(Instruction {
            name: "AND",
            address_mode: AddressMode::IndirectY,
            execute: instructions::and,
        }),
        0x35 => Some(Instruction {
            name: "AND",
            address_mode: AddressMode::ZeroPageX,
            execute: instructions::and,
        }),
        0x36 => Some(Instruction {
            name: "ROL",
            address_mode: AddressMode::ZeroPageX,
            execute: instructions::rol,
        }),
        0x38 => Some(Instruction {
            name: "SEC",
            address_mode: AddressMode::Implicit,
            execute: instructions::sec,
        }),
        0x39 => Some(Instruction {
            name: "AND",
            address_mode: AddressMode::AbsoluteY,
            execute: instructions::and,
        }),
        0x3D => Some(Instruction {
            name: "AND",
            address_mode: AddressMode::AbsoluteX,
            execute: instructions::and,
        }),
        0x3E => Some(Instruction {
            name: "ROL",
            address_mode: AddressMode::AbsoluteX,
            execute: instructions::rol,
        }),
        0x40 => Some(Instruction {
            name: "RTI",
            address_mode: AddressMode::Implicit,
            execute: instructions::rti,
        }),
        0x41 => Some(Instruction {
            name: "EOR",
            address_mode: AddressMode::IndirectX,
            execute: instructions::eor,
        }),
        0x45 => Some(Instruction {
            name: "EOR",
            address_mode: AddressMode::ZeroPage,
            execute: instructions::eor,
        }),
        0x46 => Some(Instruction {
            name: "LSR",
            address_mode: AddressMode::ZeroPage,
            execute: instructions::lsr,
        }),
        0x48 => Some(Instruction {
            name: "PHA",
            address_mode: AddressMode::Implicit,
            execute: instructions::pha,
        }),
        0x49 => Some(Instruction {
            name: "EOR",
            address_mode: AddressMode::Immidiate,
            execute: instructions::eor,
        }),
        0x4A => Some(Instruction {
            name: "LSR",
            address_mode: AddressMode::Accumulator,
            execute: instructions::lsr,
        }),
        0x4C => Some(Instruction {
            name: "JMP",
            address_mode: AddressMode::Absolute,
            execute: instructions::jmp,
        }),
        0x4D => Some(Instruction {
            name: "EOR",
            address_mode: AddressMode::Absolute,
            execute: instructions::eor,
        }),
        0x4E => Some(Instruction {
            name: "LSR",
            address_mode: AddressMode::Absolute,
            execute: instructions::lsr,
        }),
        0x50 => Some(Instruction {
            name: "BVC",
            address_mode: AddressMode::Relative,
            execute: instructions::bvc,
        }),
        0x51 => Some(Instruction {
            name: "EOR",
            address_mode: AddressMode::IndirectY,
            execute: instructions::eor,
        }),
        0x55 => Some(Instruction {
            name: "EOR",
            address_mode: AddressMode::ZeroPageX,
            execute: instructions::eor,
        }),
        0x56 => Some(Instruction {
            name: "LSR",
            address_mode: AddressMode::ZeroPageX,
            execute: instructions::lsr,
        }),
        0x58 => Some(Instruction {
            name: "CLI",
            address_mode: AddressMode::Implicit,
            execute: instructions::cli,
        }),
        0x59 => Some(Instruction {
            name: "EOR",
            address_mode: AddressMode::AbsoluteY,
            execute: instructions::eor,
        }),
        0x5D => Some(Instruction {
            name: "EOR",
            address_mode: AddressMode::AbsoluteX,
            execute: instructions::eor,
        }),
        0x5E => Some(Instruction {
            name: "LSR",
            address_mode: AddressMode::AbsoluteX,
            execute: instructions::lsr,
        }),
        0x60 => Some(Instruction {
            name: "RTS",
            address_mode: AddressMode::Implicit,
            execute: instructions::rts,
        }),
        0x61 => Some(Instruction {
            name: "ADC",
            address_mode: AddressMode::IndirectX,
            execute: instructions::adc,
        }),
        0x65 => Some(Instruction {
            name: "ADC",
            address_mode: AddressMode::ZeroPage,
            execute: instructions::adc,
        }),
        0x66 => Some(Instruction {
            name: "ROR",
            address_mode: AddressMode::ZeroPage,
            execute: instructions::ror,
        }),
        0x68 => Some(Instruction {
            name: "PLA",
            address_mode: AddressMode::Implicit,
            execute: instructions::pla,
        }),
        0x69 => Some(Instruction {
            name: "ADC",
            address_mode: AddressMode::Immidiate,
            execute: instructions::adc,
        }),
        0x6A => Some(Instruction {
            name: "ROR",
            address_mode: AddressMode::Accumulator,
            execute: instructions::ror,
        }),
        0x6C => Some(Instruction {
            name: "JMP",
            address_mode: AddressMode::Indirect,
            execute: instructions::jmp,
        }),
        0x6D => Some(Instruction {
            name: "ADC",
            address_mode: AddressMode::Absolute,
            execute: instructions::adc,
        }),
        0x6E => Some(Instruction {
            name: "ROR",
            address_mode: AddressMode::Absolute,
            execute: instructions::ror,
        }),
        0x70 => Some(Instruction {
            name: "BVS",
            address_mode: AddressMode::Relative,
            execute: instructions::bvs,
        }),
        0x71 => Some(Instruction {
            name: "ADC",
            address_mode: AddressMode::IndirectY,
            execute: instructions::adc,
        }),
        0x75 => Some(Instruction {
            name: "ADC",
            address_mode: AddressMode::ZeroPageX,
            execute: instructions::adc,
        }),
        0x76 => Some(Instruction {
            name: "ROR",
            address_mode: AddressMode::ZeroPageX,
            execute: instructions::ror,
        }),
        0x78 => Some(Instruction {
            name: "SEI",
            address_mode: AddressMode::Implicit,
            execute: instructions::sei,
        }),
        0x79 => Some(Instruction {
            name: "ADC",
            address_mode: AddressMode::AbsoluteY,
            execute: instructions::adc,
        }),
        0x7D => Some(Instruction {
            name: "ADC",
            address_mode: AddressMode::AbsoluteX,
            execute: instructions::adc,
        }),
        0x7E => Some(Instruction {
            name: "ROR",
            address_mode: AddressMode::AbsoluteX,
            execute: instructions::ror,
        }),
        0x81 => Some(Instruction {
            name: "STA",
            address_mode: AddressMode::IndirectX,
            execute: instructions::sta,
        }),
        0x84 => Some(Instruction {
            name: "STY",
            address_mode: AddressMode::ZeroPage,
            execute: instructions::sty,
        }),
        0x85 => Some(Instruction {
            name: "STA",
            address_mode: AddressMode::ZeroPage,
            execute: instructions::sta,
        }),
        0x86 => Some(Instruction {
            name: "STX",
            address_mode: AddressMode::ZeroPage,
            execute: instructions::stx,
        }),
        0x88 => Some(Instruction {
            name: "DEY",
            address_mode: AddressMode::Implicit,
            execute: instructions::dey,
        }),
        0x8A => Some(Instruction {
            name: "TXA",
            address_mode: AddressMode::Implicit,
            execute: instructions::txa,
        }),
        0x8C => Some(Instruction {
            name: "STY",
            address_mode: AddressMode::Absolute,
            execute: instructions::sty,
        }),
        0x8D => Some(Instruction {
            name: "STA",
            address_mode: AddressMode::Absolute,
            execute: instructions::sta,
        }),
        0x8E => Some(Instruction {
            name: "STX",
            address_mode: AddressMode::Absolute,
            execute: instructions::stx,
        }),
        0x90 => Some(Instruction {
            name: "BCC",
            address_mode: AddressMode::Relative,
            execute: instructions::bcc,
        }),
        0x91 => Some(Instruction {
            name: "STA",
            address_mode: AddressMode::IndirectY,
            execute: instructions::sta,
        }),
        0x94 => Some(Instruction {
            name: "STY",
            address_mode: AddressMode::ZeroPageX,
            execute: instructions::sty,
        }),
        0x95 => Some(Instruction {
            name: "STA",
            address_mode: AddressMode::ZeroPageX,
            execute: instructions::sta,
        }),
        0x96 => Some(Instruction {
            name: "STX",
            address_mode: AddressMode::ZeroPageY,
            execute: instructions::stx,
        }),
        0x98 => Some(Instruction {
            name: "TYA",
            address_mode: AddressMode::Implicit,
            execute: instructions::tya,
        }),
        0x99 => Some(Instruction {
            name: "STA",
            address_mode: AddressMode::AbsoluteY,
            execute: instructions::sta,
        }),
        0x9A => Some(Instruction {
            name: "TXS",
            address_mode: AddressMode::Implicit,
            execute: instructions::txs,
        }),
        0x9D => Some(Instruction {
            name: "STA",
            address_mode: AddressMode::AbsoluteX,
            execute: instructions::sta,
        }),
        0xA0 => Some(Instruction {
            name: "LDY",
            address_mode: AddressMode::Immidiate,
            execute: instructions::ldy,
        }),
        0xA1 => Some(Instruction {
            name: "LDY",
            address_mode: AddressMode::IndirectX,
            execute: instructions::ldy,
        }),
        0xA2 => Some(Instruction {
            name: "LDX",
            address_mode: AddressMode::Immidiate,
            execute: instructions::ldx,
        }),
        0xA4 => Some(Instruction {
            name: "LDY",
            address_mode: AddressMode::ZeroPage,
            execute: instructions::ldy,
        }),
        0xA5 => Some(Instruction {
            name: "LDA",
            address_mode: AddressMode::ZeroPage,
            execute: instructions::lda,
        }),
        0xA6 => Some(Instruction {
            name: "LDX",
            address_mode: AddressMode::ZeroPage,
            execute: instructions::ldx,
        }),
        0xA8 => Some(Instruction {
            name: "TAY",
            address_mode: AddressMode::Implicit,
            execute: instructions::tay,
        }),
        0xA9 => Some(Instruction {
            name: "LDA",
            address_mode: AddressMode::Immidiate,
            execute: instructions::lda,
        }),
        0xAA => Some(Instruction {
            name: "TAX",
            address_mode: AddressMode::Implicit,
            execute: instructions::tax,
        }),
        0xAC => Some(Instruction {
            name: "LDY",
            address_mode: AddressMode::Absolute,
            execute: instructions::ldy,
        }),
        0xAD => Some(Instruction {
            name: "LDA",
            address_mode: AddressMode::Absolute,
            execute: instructions::lda,
        }),
        0xAE => Some(Instruction {
            name: "LDX",
            address_mode: AddressMode::Absolute,
            execute: instructions::ldx,
        }),
        0xB0 => Some(Instruction {
            name: "BCS",
            address_mode: AddressMode::Relative,
            execute: instructions::bcs,
        }),
        0xB1 => Some(Instruction {
            name: "LDA",
            address_mode: AddressMode::IndirectY,
            execute: instructions::lda,
        }),
        0xB4 => Some(Instruction {
            name: "LDY",
            address_mode: AddressMode::ZeroPageX,
            execute: instructions::ldy,
        }),
        0xB5 => Some(Instruction {
            name: "LDA",
            address_mode: AddressMode::ZeroPageX,
            execute: instructions::lda,
        }),
        0xB6 => Some(Instruction {
            name: "LDX",
            address_mode: AddressMode::ZeroPageY,
            execute: instructions::ldx,
        }),
        0xB8 => Some(Instruction {
            name: "CLV",
            address_mode: AddressMode::Implicit,
            execute: instructions::clv,
        }),
        0xB9 => Some(Instruction {
            name: "LDA",
            address_mode: AddressMode::AbsoluteY,
            execute: instructions::lda,
        }),
        0xBA => Some(Instruction {
            name: "TSX",
            address_mode: AddressMode::Implicit,
            execute: instructions::tsx,
        }),
        0xBC => Some(Instruction {
            name: "LDY",
            address_mode: AddressMode::AbsoluteX,
            execute: instructions::ldy,
        }),
        0xBD => Some(Instruction {
            name: "LDA",
            address_mode: AddressMode::AbsoluteX,
            execute: instructions::lda,
        }),
        0xBE => Some(Instruction {
            name: "LDX",
            address_mode: AddressMode::AbsoluteY,
            execute: instructions::ldx,
        }),
        0xC0 => Some(Instruction {
            name: "CPY",
            address_mode: AddressMode::Immidiate,
            execute: instructions::cpy,
        }),
        0xC1 => Some(Instruction {
            name: "CMP",
            address_mode: AddressMode::IndirectX,
            execute: instructions::cmp,
        }),
        0xC4 => Some(Instruction {
            name: "CPY",
            address_mode: AddressMode::ZeroPage,
            execute: instructions::cpy,
        }),
        0xC5 => Some(Instruction {
            name: "CMP",
            address_mode: AddressMode::ZeroPage,
            execute: instructions::cmp,
        }),
        0xC6 => Some(Instruction {
            name: "DEC",
            address_mode: AddressMode::ZeroPage,
            execute: instructions::dec,
        }),
        0xC8 => Some(Instruction {
            name: "INY",
            address_mode: AddressMode::Implicit,
            execute: instructions::iny,
        }),
        0xC9 => Some(Instruction {
            name: "CMP",
            address_mode: AddressMode::Immidiate,
            execute: instructions::cmp,
        }),
        0xCA => Some(Instruction {
            name: "DEX",
            address_mode: AddressMode::Implicit,
            execute: instructions::dex,
        }),
        0xCC => Some(Instruction {
            name: "CPY",
            address_mode: AddressMode::Absolute,
            execute: instructions::cpy,
        }),
        0xCD => Some(Instruction {
            name: "CMP",
            address_mode: AddressMode::Absolute,
            execute: instructions::cmp,
        }),
        0xCE => Some(Instruction {
            name: "DEC",
            address_mode: AddressMode::Absolute,
            execute: instructions::dec,
        }),
        0xD0 => Some(Instruction {
            name: "BNE",
            address_mode: AddressMode::Relative,
            execute: instructions::bne,
        }),
        0xD1 => Some(Instruction {
            name: "CMP",
            address_mode: AddressMode::IndirectY,
            execute: instructions::cmp,
        }),
        0xD5 => Some(Instruction {
            name: "CMP",
            address_mode: AddressMode::ZeroPageX,
            execute: instructions::cmp,
        }),
        0xD6 => Some(Instruction {
            name: "DEC",
            address_mode: AddressMode::ZeroPageX,
            execute: instructions::dec,
        }),
        0xD8 => Some(Instruction {
            name: "CLD",
            address_mode: AddressMode::Implicit,
            execute: instructions::cld,
        }),
        0xD9 => Some(Instruction {
            name: "CMP",
            address_mode: AddressMode::AbsoluteY,
            execute: instructions::cmp,
        }),
        0xDD => Some(Instruction {
            name: "CMP",
            address_mode: AddressMode::AbsoluteX,
            execute: instructions::cmp,
        }),
        0xDE => Some(Instruction {
            name: "DEC",
            address_mode: AddressMode::AbsoluteX,
            execute: instructions::dec,
        }),
        0xE0 => Some(Instruction {
            name: "CPX",
            address_mode: AddressMode::Immidiate,
            execute: instructions::cpx,
        }),
        0xE1 => Some(Instruction {
            name: "SBC",
            address_mode: AddressMode::IndirectX,
            execute: instructions::sbc,
        }),
        0xE4 => Some(Instruction {
            name: "CPX",
            address_mode: AddressMode::ZeroPage,
            execute: instructions::cpx,
        }),
        0xE5 => Some(Instruction {
            name: "SBC",
            address_mode: AddressMode::ZeroPage,
            execute: instructions::sbc,
        }),
        0xE6 => Some(Instruction {
            name: "INC",
            address_mode: AddressMode::ZeroPage,
            execute: instructions::inc,
        }),
        0xE8 => Some(Instruction {
            name: "INX",
            address_mode: AddressMode::Implicit,
            execute: instructions::inx,
        }),
        0xE9 => Some(Instruction {
            name: "SBC",
            address_mode: AddressMode::Immidiate,
            execute: instructions::sbc,
        }),
        0xEA => Some(Instruction {
            name: "NOP",
            address_mode: AddressMode::Implicit,
            execute: instructions::nop,
        }),
        0xEC => Some(Instruction {
            name: "CPX",
            address_mode: AddressMode::Absolute,
            execute: instructions::cpx,
        }),
        0xED => Some(Instruction {
            name: "SBC",
            address_mode: AddressMode::Absolute,
            execute: instructions::sbc,
        }),
        0xEE => Some(Instruction {
            name: "INC",
            address_mode: AddressMode::Absolute,
            execute: instructions::inc,
        }),
        0xF0 => Some(Instruction {
            name: "BEQ",
            address_mode: AddressMode::Relative,
            execute: instructions::beq,
        }),
        0xF1 => Some(Instruction {
            name: "SBC",
            address_mode: AddressMode::IndirectY,
            execute: instructions::sbc,
        }),
        0xF5 => Some(Instruction {
            name: "SBC",
            address_mode: AddressMode::ZeroPageX,
            execute: instructions::sbc,
        }),
        0xF6 => Some(Instruction {
            name: "INC",
            address_mode: AddressMode::ZeroPageX,
            execute: instructions::inc,
        }),
        0xF8 => Some(Instruction {
            name: "SED",
            address_mode: AddressMode::Implicit,
            execute: instructions::sed,
        }),
        0xF9 => Some(Instruction {
            name: "SBC",
            address_mode: AddressMode::AbsoluteY,
            execute: instructions::sbc,
        }),
        0xFD => Some(Instruction {
            name: "SBC",
            address_mode: AddressMode::AbsoluteX,
            execute: instructions::sbc,
        }),
        0xFE => Some(Instruction {
            name: "INC",
            address_mode: AddressMode::AbsoluteX,
            execute: instructions::inc,
        }),
        _ => None, // Return None for unknown opcodes
    }
}
