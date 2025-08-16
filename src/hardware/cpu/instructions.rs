use std::fmt::{self, Display, Formatter};

use crate::hardware::bus::Bus;

use super::Registers;

#[derive(PartialEq, Eq)]
pub enum AddressMode {
    Implicit,
    Immidiate,
    Accumulator,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Relative,
    Indirect,
    IndirectX,
    IndirectY,
}

impl Display for AddressMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AddressMode::Relative => write!(f, "R"),
            AddressMode::Implicit => write!(f, "I"),
            AddressMode::Immidiate => write!(f, "Imm"),
            AddressMode::Accumulator => write!(f, "A"),
            AddressMode::ZeroPage => write!(f, "ZP"),
            AddressMode::ZeroPageX => write!(f, "ZPX"),
            AddressMode::ZeroPageY => write!(f, "ZPY"),
            AddressMode::Absolute => write!(f, "Ab"),
            AddressMode::AbsoluteX => write!(f, "AbX"),
            AddressMode::AbsoluteY => write!(f, "AbY"),
            AddressMode::Indirect => write!(f, "Ind"),
            AddressMode::IndirectX => write!(f, "IndX"),
            AddressMode::IndirectY => write!(f, "IndY"),
        }
    }
}

fn log_instruct(instruct_name: &str, address_mode: &AddressMode, bus: Option<&mut Bus>) {
    // match bus {
    //     Some(b) => {
    //         let (instruct, _) = b.get_instruction_text(address_mode, None);
    //         println!(
    //             "{0:x}: {1} - [ {2} {3} ]",
    //             b.cpu.get_counter(),
    //             instruct,
    //             instruct_name,
    //             address_mode
    //         );
    //     }
    //     None => {
    //         println!("[ {0} {1} ]", instruct_name, address_mode);
    //     }
    // }
}

pub fn adc(bus: &mut Bus, address_mode: AddressMode) -> u8 {
    // Add with carry
    log_instruct("ADC", &address_mode, Some(bus));
    let val = bus.read_address_with_mode(&address_mode);
    let result: u16 =
        (val.value as u16) + (bus.cpu.get(Registers::A) as u16) + (bus.cpu.get_carry() as u16);
    bus.cpu.set_carry(result > 0xFF);
    bus.cpu.set_zero(result & 0xFF == 0);
    bus.cpu.set_overflow(
        ((bus.cpu.get(Registers::A) ^ result as u8) & (val.value ^ result as u8) & 0x80) != 0,
    );
    bus.cpu.set_negative(result & 0x80 != 0);
    bus.cpu.set(Registers::A, result as u8);
    bus.increment_pc(&address_mode);
    let cycles: u8 = match address_mode {
        AddressMode::Immidiate => 2,
        AddressMode::ZeroPage => 3,
        AddressMode::ZeroPageX => 4,
        AddressMode::Absolute => 4,
        AddressMode::AbsoluteX => 4,
        AddressMode::AbsoluteY => 4,
        AddressMode::IndirectX => 6,
        AddressMode::IndirectY => 5,
        _ => 0,
    } + val.cycles;
    cycles
}

pub fn and(bus: &mut Bus, address_mode: AddressMode) -> u8 {
    // Logical AND
    log_instruct("AND", &address_mode, Some(bus));
    let val = bus.read_address_with_mode(&address_mode);
    let result = bus.cpu.get(Registers::A) & val.value;
    bus.cpu.set_zero(result == 0);
    bus.cpu.set_negative(result & 0x80 != 0);
    bus.cpu.set(Registers::A, result);
    bus.increment_pc(&address_mode);
    let cycles: u8 = match address_mode {
        AddressMode::Immidiate => 2,
        AddressMode::ZeroPage => 3,
        AddressMode::ZeroPageX => 4,
        AddressMode::Absolute => 4,
        AddressMode::AbsoluteX => 4,
        AddressMode::AbsoluteY => 4,
        AddressMode::IndirectX => 6,
        AddressMode::IndirectY => 5,
        _ => 0,
    } + val.cycles;
    cycles
}

pub fn asl(bus: &mut Bus, address_mode: AddressMode) -> u8 {
    // Arithmetic shift left
    log_instruct("ASL", &address_mode, Some(bus));
    let val = bus.read_address_with_mode(&address_mode);
    let result = val.value << 1;
    bus.cpu.set_carry(val.value & 0x80 != 0);
    bus.cpu.set_zero(result == 0);
    bus.cpu.set_negative(result & 0x80 != 0);
    bus.cpu.set(Registers::A, result);
    bus.increment_pc(&address_mode);
    let cycles: u8 = match address_mode {
        AddressMode::Accumulator => 2,
        AddressMode::ZeroPage => 5,
        AddressMode::ZeroPageX => 6,
        AddressMode::Absolute => 6,
        AddressMode::AbsoluteX => 7,
        _ => 0,
    } + val.cycles;
    cycles
}

pub fn bcc(bus: &mut Bus, address_mode: AddressMode) -> u8 {
    // Branch if carry clear
    log_instruct("BCC", &address_mode, Some(bus));
    let val = bus.read_address_with_mode(&address_mode);
    let mut cycles = val.cycles;
    if bus.cpu.get_carry() == 0 {
        cycles += 1;
        let pc = bus.cpu.get_counter();
        bus.cpu.set_counter(val.address);
        if pc & 0xFF00 != val.address & 0xFF00 {
            cycles += 1;
        }
    }
    bus.increment_pc(&address_mode);
    cycles + 2
}

pub fn bcs(bus: &mut Bus, address_mode: AddressMode) -> u8 {
    // Branch if carry set
    log_instruct("BCS", &address_mode, Some(bus));
    let val = bus.read_address_with_mode(&address_mode);
    let mut cycles = val.cycles;
    if bus.cpu.get_carry() != 0 {
        cycles += 1;
        let pc = bus.cpu.get_counter();
        bus.cpu.set_counter(val.address);
        if pc & 0xFF00 != val.address & 0xFF00 {
            cycles += 1;
        }
    }
    bus.increment_pc(&address_mode);
    cycles + 2
}

pub fn beq(bus: &mut Bus, address_mode: AddressMode) -> u8 {
    // Branch if equal
    log_instruct("BEQ", &address_mode, Some(bus));
    let val = bus.read_address_with_mode(&address_mode);
    let mut cycles = val.cycles;
    if bus.cpu.get_zero() != 0 {
        cycles += 1;
        let pc = bus.cpu.get_counter();
        bus.cpu.set_counter(val.address);
        if pc & 0xFF00 != val.address & 0xFF00 {
            cycles += 1;
        }
    } else {
        bus.increment_pc(&address_mode);
    }
    cycles + 2
}

pub fn bit(bus: &mut Bus, address_mode: AddressMode) -> u8 {
    // Bit test
    log_instruct("BIT", &address_mode, Some(bus));
    let val = bus.read_address_with_mode(&address_mode);
    let result = bus.cpu.get(Registers::A) & val.value;
    bus.cpu.set_zero(result == 0);
    bus.cpu.set_overflow(val.value & 0x40 != 0);
    bus.cpu.set_negative(val.value & 0x80 != 0);
    let cycles: u8 = match address_mode {
        AddressMode::ZeroPage => 3,
        AddressMode::Absolute => 4,
        _ => 0,
    } + val.cycles;
    bus.increment_pc(&address_mode);
    cycles
}

pub fn bmi(bus: &mut Bus, address_mode: AddressMode) -> u8 {
    // Branch if minus
    log_instruct("BMI", &address_mode, Some(bus));
    let val = bus.read_address_with_mode(&address_mode);
    let mut cycles = val.cycles;
    if bus.cpu.get_negative() != 0 {
        cycles += 1;
        let pc = bus.cpu.get_counter();
        bus.cpu.set_counter(val.address);
        if pc & 0xFF00 != val.address & 0xFF00 {
            cycles += 1;
        }
    } else {
        bus.increment_pc(&address_mode);
    }
    cycles + 2
}

pub fn bne(bus: &mut Bus, address_mode: AddressMode) -> u8 {
    // Branch if not Equal
    log_instruct("BNE", &address_mode, Some(bus));
    let val = bus.read_address_with_mode(&address_mode);
    let mut cycles = val.cycles;
    if bus.cpu.get_zero() == 0 {
        cycles += 1;
        let pc = bus.cpu.get_counter();
        bus.cpu.set_counter(val.address);
        if pc & 0xFF00 != val.address & 0xFF00 {
            cycles += 1;
        }
    } else {
        bus.increment_pc(&address_mode);
    }
    cycles + 2
}

pub fn bpl(bus: &mut Bus, address_mode: AddressMode) -> u8 {
    // Branch if positive
    log_instruct("BPL", &address_mode, Some(bus));
    let val = bus.read_address_with_mode(&address_mode);
    let mut cycles = val.cycles;
    if bus.cpu.get_negative() == 0 {
        cycles += 1;
        let pc = bus.cpu.get_counter();
        bus.cpu.set_counter(val.address);
        if pc & 0xFF00 != val.address & 0xFF00 {
            cycles += 1;
        }
        bus.increment_pc(&address_mode);
    } else {
        bus.increment_pc(&address_mode);
    }
    cycles + 2
}

pub fn brk(bus: &mut Bus, address_mode: AddressMode) -> u8 {
    // Force intrupt
    log_instruct("BRK", &address_mode, Some(bus));
    bus.stack_push_word(bus.cpu.get_counter() + 2);
    bus.stack_push(bus.cpu.get(Registers::P));
    bus.cpu.set_break(true);
    let low_byte = bus.read(0xFFFE) as u16;
    let high_byte = bus.read(0xFFFF) as u16;
    bus.cpu.set_counter((high_byte << 8) + low_byte);
    bus.cpu.set_interrupt_disable(true);
    bus.increment_pc_by(2);
    return 2;
}

pub fn bvc(bus: &mut Bus, address_mode: AddressMode) -> u8 {
    // Branch if Overflow clear
    log_instruct("BVC", &address_mode, Some(bus));
    let val = bus.read_address_with_mode(&address_mode);
    let mut cycles = val.cycles;
    if bus.cpu.get_overflow() == 0 {
        cycles += 1;
        let pc = bus.cpu.get_counter();
        bus.cpu.set_counter(val.address);
        if pc & 0xFF00 != val.address & 0xFF00 {
            cycles += 1;
        }
    } else {
        bus.increment_pc(&address_mode);
    }
    cycles + 2
}

pub fn bvs(bus: &mut Bus, address_mode: AddressMode) -> u8 {
    // Branch if Overflow set
    log_instruct("BVS", &address_mode, Some(bus));
    let val = bus.read_address_with_mode(&address_mode);
    let mut cycles = val.cycles;
    if bus.cpu.get_overflow() != 0 {
        cycles += 1;
        let pc = bus.cpu.get_counter();
        bus.cpu.set_counter(val.address);
        if pc & 0xFF00 != val.address & 0xFF00 {
            cycles += 1;
        }
    } else {
        bus.increment_pc(&address_mode);
    }
    cycles + 2
}

pub fn clc(bus: &mut Bus, address_mode: AddressMode) -> u8 {
    // Clear Carry Flag
    log_instruct("CLC", &address_mode, Some(bus));
    bus.increment_pc(&address_mode);
    bus.cpu.set_carry(false);
    2 // CLC takes 2 cycles
}

pub fn cld(bus: &mut Bus, address_mode: AddressMode) -> u8 {
    // Clear Decimal Mode
    log_instruct("CLD", &address_mode, Some(bus));
    bus.increment_pc(&address_mode);
    bus.cpu.set_decimal(false);
    2 // CLD takes 2 cycles
}

pub fn cli(bus: &mut Bus, address_mode: AddressMode) -> u8 {
    // Set Interrupt Disable
    log_instruct("CLI", &address_mode, Some(bus));
    bus.increment_pc(&address_mode);
    bus.cpu.set_overflow(false);
    2 // CLI takes 2 cycles
}

pub fn clv(bus: &mut Bus, address_mode: AddressMode) -> u8 {
    // Clear Overflow FLag
    log_instruct("CLV", &address_mode, Some(bus));
    bus.increment_pc(&address_mode);
    bus.cpu.set_overflow(true);
    2 // CLV takes 2 cycles
}

pub fn cmp(bus: &mut Bus, address_mode: AddressMode) -> u8 {
    // Compare
    log_instruct("CMP", &address_mode, Some(bus));
    let val = bus.read_address_with_mode(&address_mode);
    let acc = bus.cpu.a;
    let res = acc as i8 - val.value as i8;
    if res >= 0 {
        bus.cpu.set_carry(true);
        if res == 0 {
            bus.cpu.set_zero(true);
        }
    }
    bus.cpu.set_negative(res < 0);
    bus.increment_pc(&address_mode);
    let cycles: u8 = match address_mode {
        AddressMode::Immidiate => 2,
        AddressMode::ZeroPage => 3,
        AddressMode::ZeroPageX => 4,
        AddressMode::Absolute => 4,
        AddressMode::AbsoluteX => 4,
        AddressMode::AbsoluteY => 4,
        AddressMode::IndirectX => 6,
        AddressMode::IndirectY => 5,
        _ => 0,
    } + val.cycles;
    cycles
}

pub fn cpx(bus: &mut Bus, address_mode: AddressMode) -> u8 {
    // Compare X Register
    log_instruct("CPX", &address_mode, Some(bus));
    let val = bus.read_address_with_mode(&address_mode);
    let reg = bus.cpu.x;
    let res = reg as i8 - val.value as i8;
    if res >= 0 {
        bus.cpu.set_carry(true);
        if res == 0 {
            bus.cpu.set_zero(true);
        }
    }
    bus.cpu.set_negative(res < 0);
    bus.increment_pc(&address_mode);
    let cycles: u8 = match address_mode {
        AddressMode::Immidiate => 2,
        AddressMode::ZeroPage => 3,
        AddressMode::Absolute => 4,
        _ => 0,
    } + val.cycles;
    cycles
}

pub fn cpy(bus: &mut Bus, address_mode: AddressMode) -> u8 {
    // Compare Y Register
    log_instruct("CPY", &address_mode, Some(bus));
    let val = bus.read_address_with_mode(&address_mode);
    let reg = bus.cpu.y;
    let res = reg as i8 - val.value as i8;
    if res >= 0 {
        bus.cpu.set_carry(true);
        if res == 0 {
            bus.cpu.set_zero(true);
        }
    }
    bus.cpu.set_negative(res < 0);
    bus.increment_pc(&address_mode);
    let cycles: u8 = match address_mode {
        AddressMode::Immidiate => 2,
        AddressMode::ZeroPage => 3,
        AddressMode::Absolute => 4,
        _ => 0,
    } + val.cycles;
    cycles
}

pub fn dec(bus: &mut Bus, address_mode: AddressMode) -> u8 {
    // Decrement Memory
    log_instruct("DEC", &address_mode, Some(bus));
    let is_accumulator = address_mode == AddressMode::Accumulator;
    let val = bus.read_address_with_mode(&address_mode);
    let res = val.value.wrapping_sub(1);
    bus.cpu.set_negative(res & 0x80 != 0);
    bus.cpu.set_zero(res == 0);
    if is_accumulator {
        bus.cpu.a = res;
    } else {
        bus.write(val.address, res as u8);
    }
    bus.increment_pc(&address_mode);
    let cycles: u8 = match address_mode {
        AddressMode::ZeroPage => 5,
        AddressMode::ZeroPageX => 6,
        AddressMode::Absolute => 6,
        AddressMode::AbsoluteX => 7,
        _ => 0,
    } + val.cycles;
    cycles
}

pub fn dex(bus: &mut Bus, address_mode: AddressMode) -> u8 {
    // Decrement X Register
    if address_mode != AddressMode::Implicit {
        panic!("Only Implicit address mode is acceptable for DEX!");
    }
    log_instruct("DEX", &address_mode, Some(bus));
    let val = bus.cpu.x;
    let res = val.wrapping_sub(1);
    bus.cpu.set_zero(res == 0);
    bus.cpu.set_negative(res & 0x80 != 0);
    bus.cpu.x = res;
    bus.increment_pc(&address_mode);
    2 // DEX takes 2 cycles
}

pub fn dey(bus: &mut Bus, address_mode: AddressMode) -> u8 {
    // Decrement Y Register
    if address_mode != AddressMode::Implicit {
        panic!("Only Implicit address mode is acceptable for DEY!");
    }
    log_instruct("DEY", &address_mode, Some(bus));
    let val = bus.cpu.y;
    let res = val.wrapping_sub(1);
    bus.cpu.set_zero(res == 0);
    bus.cpu.set_negative(res & 0x80 != 0);
    bus.cpu.y = res;
    bus.increment_pc(&address_mode);
    2 // DEY takes 2 cycles
}

pub fn eor(bus: &mut Bus, address_mode: AddressMode) -> u8 {
    // Exclusive OR
    log_instruct("EOR", &address_mode, Some(bus));
    let acc = bus.cpu.a;
    let val = bus.read_address_with_mode(&address_mode);
    let res = acc ^ val.value;
    bus.cpu.set_negative(res & 0x80 != 0);
    bus.cpu.set_zero(res == 0);
    bus.cpu.a = res;
    bus.increment_pc(&address_mode);
    let cycles: u8 = match address_mode {
        AddressMode::Immidiate => 2,
        AddressMode::ZeroPage => 3,
        AddressMode::ZeroPageX => 4,
        AddressMode::Absolute => 4,
        AddressMode::AbsoluteX => 4,
        AddressMode::AbsoluteY => 4,
        AddressMode::IndirectX => 6,
        AddressMode::IndirectY => 5,
        _ => 0,
    } + val.cycles;
    cycles
}

pub fn inc(bus: &mut Bus, address_mode: AddressMode) -> u8 {
    // Increment Memory
    log_instruct("INC", &address_mode, Some(bus));
    let is_accumulator = address_mode == AddressMode::Accumulator;
    let val = bus.read_address_with_mode(&address_mode);
    let res = val.value.wrapping_add(1);
    bus.cpu.set_zero(res == 0);
    bus.cpu.set_negative(res & 0x80 != 0);
    if is_accumulator {
        bus.cpu.a = res;
    } else {
        bus.write(val.address, res);
    }
    bus.increment_pc(&address_mode);
    let cycles: u8 = match address_mode {
        AddressMode::ZeroPage => 5,
        AddressMode::ZeroPageX => 6,
        AddressMode::Absolute => 6,
        AddressMode::AbsoluteX => 7,
        _ => 0,
    } + val.cycles;
    cycles
}

pub fn inx(bus: &mut Bus, address_mode: AddressMode) -> u8 {
    // Increment X Register
    log_instruct("INX", &address_mode, Some(bus));
    let reg = bus.cpu.x;
    let res = reg.wrapping_add(1);
    bus.cpu.set_zero(res == 0);
    bus.cpu.set_negative(res & 0x80 != 0);
    bus.cpu.x = res;
    bus.increment_pc(&address_mode);
    2 // INX takes 2 cycles
}

pub fn iny(bus: &mut Bus, address_mode: AddressMode) -> u8 {
    // Increment Y Register
    log_instruct("INY", &address_mode, Some(bus));
    let reg = bus.cpu.y;
    let res = reg.wrapping_add(1);
    bus.cpu.set_zero(res == 0);
    bus.cpu.set_negative(res & 0x80 != 0);
    bus.cpu.y = res;
    bus.increment_pc(&address_mode);
    2 // INY takes 2 cycles
}

pub fn jmp(bus: &mut Bus, address_mode: AddressMode) -> u8 {
    // Jump
    log_instruct("JMP", &address_mode, Some(bus));
    let val = bus.read_address_with_mode(&address_mode);
    bus.cpu.set_counter(val.address);
    match address_mode {
        AddressMode::Absolute => 3,
        AddressMode::Indirect => 5,
        _ => 0,
    }
}

pub fn jsr(bus: &mut Bus, address_mode: AddressMode) -> u8 {
    // Jump to Subroutine
    log_instruct("JSR", &address_mode, Some(bus));
    let val = bus.read_address_with_mode(&address_mode);
    bus.stack_push_word(bus.cpu.get_counter().wrapping_sub(1));
    bus.cpu.set_counter(val.address);
    bus.increment_pc(&address_mode);
    6 // JSR takes 6 cycles
}

pub fn lda(bus: &mut Bus, address_mode: AddressMode) -> u8 {
    // Load Accumulator
    log_instruct("LDA", &address_mode, Some(bus));
    let val = bus.read_address_with_mode(&address_mode);
    bus.cpu.set_zero(val.value == 0);
    bus.cpu.set_negative(val.value & 0x80 != 0);
    bus.cpu.set(Registers::A, val.value);
    bus.increment_pc(&address_mode);
    let cycles: u8 = match address_mode {
        AddressMode::Immidiate => 2,
        AddressMode::ZeroPage => 3,
        AddressMode::ZeroPageX => 4,
        AddressMode::Absolute => 4,
        AddressMode::AbsoluteX => 4,
        AddressMode::AbsoluteY => 4,
        AddressMode::IndirectX => 6,
        AddressMode::IndirectY => 5,
        _ => 0,
    } + val.cycles;
    cycles
}

pub fn ldx(bus: &mut Bus, address_mode: AddressMode) -> u8 {
    // Load X Register
    log_instruct("LDX", &address_mode, Some(bus));
    let val = bus.read_address_with_mode(&address_mode);
    bus.cpu.set_zero(val.value == 0);
    bus.cpu.set_negative(val.value & 0x80 != 0);
    bus.cpu.set(Registers::X, val.value);
    bus.increment_pc(&address_mode);
    let cycles: u8 = match address_mode {
        AddressMode::Immidiate => 2,
        AddressMode::ZeroPage => 3,
        AddressMode::ZeroPageY => 4,
        AddressMode::Absolute => 4,
        AddressMode::AbsoluteY => 4,
        _ => 0,
    } + val.cycles;
    cycles
}

pub fn ldy(bus: &mut Bus, address_mode: AddressMode) -> u8 {
    // Load Y Register
    log_instruct("LDY", &address_mode, Some(bus));
    let val = bus.read_address_with_mode(&address_mode);
    bus.cpu.set_zero(val.value == 0);
    bus.cpu.set_negative(val.value & 0x80 != 0);
    bus.cpu.set(Registers::Y, val.value);
    bus.increment_pc(&address_mode);
    let cycles: u8 = match address_mode {
        AddressMode::Immidiate => 2,
        AddressMode::ZeroPage => 3,
        AddressMode::ZeroPageX => 4,
        AddressMode::Absolute => 4,
        AddressMode::AbsoluteX => 4,
        _ => 0,
    } + val.cycles;
    cycles
}

pub fn lsr(bus: &mut Bus, address_mode: AddressMode) -> u8 {
    // Logical Shift Right
    log_instruct("LSR", &address_mode, Some(bus));
    let is_accumulator = address_mode == AddressMode::Accumulator;
    let val = bus.read_address_with_mode(&address_mode);
    let res = val.value >> 1;
    bus.cpu.set_carry(val.value & 0x01 != 0);
    bus.cpu.set_zero(res == 0);
    bus.cpu.set_negative(false);
    if is_accumulator {
        bus.cpu.set(Registers::A, res);
    } else {
        bus.write(val.address, res);
    }
    bus.increment_pc(&address_mode);
    let cycles: u8 = match address_mode {
        AddressMode::Accumulator => 2,
        AddressMode::ZeroPage => 5,
        AddressMode::ZeroPageX => 6,
        AddressMode::Absolute => 6,
        AddressMode::AbsoluteX => 7,
        _ => 0,
    } + val.cycles;
    cycles
}

pub fn nop(bus: &mut Bus, address_mode: AddressMode) -> u8 {
    // No Operation
    log_instruct("NOP", &address_mode, Some(bus));
    bus.increment_pc(&address_mode);
    2 // NOP takes 2 cycles
}

pub fn ora(bus: &mut Bus, address_mode: AddressMode) -> u8 {
    // Logical Inclusive OR
    log_instruct("ORA", &address_mode, Some(bus));
    let val = bus.read_address_with_mode(&address_mode);
    let res = bus.cpu.get(Registers::A) | val.value;
    bus.cpu.set_zero(res == 0);
    bus.cpu.set_negative(res & 0x80 != 0);
    bus.cpu.set(Registers::A, res);
    bus.increment_pc(&address_mode);
    let cycles: u8 = match address_mode {
        AddressMode::Immidiate => 2,
        AddressMode::ZeroPage => 3,
        AddressMode::ZeroPageX => 4,
        AddressMode::Absolute => 4,
        AddressMode::AbsoluteX => 4,
        AddressMode::AbsoluteY => 4,
        AddressMode::IndirectX => 6,
        AddressMode::IndirectY => 5,
        _ => 0,
    } + val.cycles;
    cycles
}

pub fn pha(bus: &mut Bus, address_mode: AddressMode) -> u8 {
    // Push Accumulator
    log_instruct("PHA", &address_mode, Some(bus));
    bus.increment_pc(&address_mode);
    bus.stack_push(bus.cpu.get(Registers::A));
    3 // PHA takes 3 cycles
}

pub fn php(bus: &mut Bus, address_mode: AddressMode) -> u8 {
    // Push Processor Status
    log_instruct("PHP", &address_mode, Some(bus));
    bus.increment_pc(&address_mode);
    bus.stack_push(bus.cpu.get(Registers::P));
    3 // PHP takes 3 cycles
}

pub fn pla(bus: &mut Bus, address_mode: AddressMode) -> u8 {
    // Pull Accumulator
    log_instruct("PLA", &address_mode, Some(bus));
    let val = bus.stack_pull();
    bus.cpu.set_zero(val == 0);
    bus.cpu.set_negative(val & 0x80 != 0);
    bus.cpu.set(Registers::A, val);
    bus.increment_pc(&address_mode);
    4 // PLA takes 4 cycles
}

pub fn plp(bus: &mut Bus, address_mode: AddressMode) -> u8 {
    // Pull Processor Status
    log_instruct("PLP", &address_mode, Some(bus));
    let mut val = bus.stack_pull();
    let interrupt_disable = val & 0b0000_0100;
    bus.cpu.delayed_interrupt = Some(interrupt_disable != 0);
    val = (val & 0b1111_1011) | (bus.cpu.get(Registers::P) & 0b0000_0100);
    bus.cpu.set(Registers::P, val);
    bus.increment_pc(&address_mode);
    4 // PLP takes 4 cycles
}

pub fn rol(bus: &mut Bus, address_mode: AddressMode) -> u8 {
    // Rotate Left
    log_instruct("ROL", &address_mode, Some(bus));
    let is_accumulator = address_mode == AddressMode::Accumulator;
    let val = bus.read_address_with_mode(&address_mode);
    let res = (val.value << 1) | bus.cpu.get_carry();
    bus.cpu.set_carry(val.value & 0x80 != 0);
    bus.cpu.set_zero(res == 0);
    bus.cpu.set_negative(res & 0x80 != 0);
    if is_accumulator {
        bus.cpu.set(Registers::A, res);
    } else {
        bus.write(val.address, res);
    }
    bus.increment_pc(&address_mode);
    let cycles: u8 = match address_mode {
        AddressMode::Accumulator => 2,
        AddressMode::ZeroPage => 5,
        AddressMode::ZeroPageX => 6,
        AddressMode::Absolute => 6,
        AddressMode::AbsoluteX => 7,
        _ => 0,
    } + val.cycles;
    cycles
}

pub fn ror(bus: &mut Bus, address_mode: AddressMode) -> u8 {
    // Rotate Right
    log_instruct("ROR", &address_mode, Some(bus));
    let is_accumulator = address_mode == AddressMode::Accumulator;
    let val = bus.read_address_with_mode(&address_mode);
    let res = (val.value >> 1) | (bus.cpu.get_carry() << 7);
    bus.cpu.set_carry(val.value & 0x01 != 0);
    bus.cpu.set_zero(res == 0);
    bus.cpu.set_negative(res & 0x80 != 0);
    if is_accumulator {
        bus.cpu.set(Registers::A, res);
    } else {
        bus.write(val.address, res);
    }
    bus.increment_pc(&address_mode);
    let cycles: u8 = match address_mode {
        AddressMode::Accumulator => 2,
        AddressMode::ZeroPage => 5,
        AddressMode::ZeroPageX => 6,
        AddressMode::Absolute => 6,
        AddressMode::AbsoluteX => 7,
        _ => 0,
    } + val.cycles;
    cycles
}

pub fn rti(bus: &mut Bus, address_mode: AddressMode) -> u8 {
    // Return from Interrupt
    log_instruct("RTI", &address_mode, Some(bus));
    let flags = bus.stack_pull();
    bus.cpu.set(Registers::P, flags);
    let pc = bus.stack_pull_word();
    bus.cpu.set_counter(pc);
    6 // RTI takes 6 cycles
}

pub fn rts(bus: &mut Bus, address_mode: AddressMode) -> u8 {
    // Return from Subroutine
    log_instruct("RTS", &address_mode, Some(bus));
    let pc = bus.stack_pull_word();
    bus.cpu.set_counter(pc + 1);
    6 // RTS takes 6 cycles
}

pub fn sbc(bus: &mut Bus, address_mode: AddressMode) -> u8 {
    // Subtract with Carry
    log_instruct("SBC", &address_mode, Some(bus));
    let val = bus.read_address_with_mode(&address_mode);
    let acc = bus.cpu.get(Registers::A);
    let carry = bus.cpu.get_carry();
    let result = acc as i16 + !val.value as i16 + carry as i16;
    bus.cpu.set_carry(result > 0xFF);
    bus.cpu.set_zero(result & 0xFF == 0);
    bus.cpu
        .set_overflow(((acc ^ result as u8) & (!val.value as u8 ^ result as u8) & 0x80) != 0);
    bus.cpu.set_negative(result & 0x80 != 0);
    bus.cpu.set(Registers::A, result as u8);
    bus.increment_pc(&address_mode);
    let cycles: u8 = match address_mode {
        AddressMode::Immidiate => 2,
        AddressMode::ZeroPage => 3,
        AddressMode::ZeroPageX => 4,
        AddressMode::Absolute => 4,
        AddressMode::AbsoluteX => 4,
        AddressMode::AbsoluteY => 4,
        AddressMode::IndirectX => 6,
        AddressMode::IndirectY => 5,
        _ => 0,
    } + val.cycles;
    cycles
}

pub fn sec(bus: &mut Bus, address_mode: AddressMode) -> u8 {
    // Set Carry Flag
    log_instruct("SEC", &address_mode, Some(bus));
    bus.increment_pc(&address_mode);
    bus.cpu.set_carry(true);
    2 // SEC takes 2 cycles
}

pub fn sed(bus: &mut Bus, address_mode: AddressMode) -> u8 {
    // Set Decimal Flag
    log_instruct("SED", &address_mode, Some(bus));
    bus.increment_pc(&address_mode);
    bus.cpu.set_decimal(true);
    2 // SED takes 2 cycles
}

pub fn sei(bus: &mut Bus, address_mode: AddressMode) -> u8 {
    // Set Interrupt Disable
    log_instruct("SEI", &address_mode, Some(bus));
    bus.increment_pc(&address_mode);
    bus.cpu.set_interrupt_disable(true);
    2 // SEI takes 2 cycles
}

pub fn sta(bus: &mut Bus, address_mode: AddressMode) -> u8 {
    // Store Accumulator
    log_instruct("STA", &address_mode, Some(bus));
    let val = bus.read_address_with_mode(&address_mode);
    bus.write(val.address, bus.cpu.get(Registers::A));
    bus.increment_pc(&address_mode);
    let cycles: u8 = match address_mode {
        AddressMode::ZeroPage => 3,
        AddressMode::ZeroPageX => 4,
        AddressMode::Absolute => 4,
        AddressMode::AbsoluteX => 5,
        AddressMode::AbsoluteY => 5,
        AddressMode::IndirectX => 6,
        AddressMode::IndirectY => 6,
        _ => 0,
    } + val.cycles;
    cycles
}

pub fn stx(bus: &mut Bus, address_mode: AddressMode) -> u8 {
    // Store X Register
    log_instruct("STX", &address_mode, Some(bus));
    let val = bus.read_address_with_mode(&address_mode);
    bus.write(val.address, bus.cpu.get(Registers::X));
    bus.increment_pc(&address_mode);
    let cycles: u8 = match address_mode {
        AddressMode::ZeroPage => 3,
        AddressMode::ZeroPageY => 4,
        AddressMode::Absolute => 4,
        _ => 0,
    } + val.cycles;
    cycles
}

pub fn sty(bus: &mut Bus, address_mode: AddressMode) -> u8 {
    // Store Y Register
    log_instruct("STY", &address_mode, Some(bus));
    let val = bus.read_address_with_mode(&address_mode);
    bus.write(val.address, bus.cpu.get(Registers::Y));
    bus.increment_pc(&address_mode);
    let cycles: u8 = match address_mode {
        AddressMode::ZeroPage => 3,
        AddressMode::ZeroPageX => 4,
        AddressMode::Absolute => 4,
        _ => 0,
    } + val.cycles;
    cycles
}

pub fn tax(bus: &mut Bus, address_mode: AddressMode) -> u8 {
    // Transfer Accumulator to X
    log_instruct("TAX", &address_mode, Some(bus));
    let val = bus.cpu.get(Registers::A);
    bus.cpu.set(Registers::X, val);
    bus.cpu.set_zero(val == 0);
    bus.cpu.set_negative(val & 0x80 != 0);
    bus.increment_pc(&address_mode);
    2 // TAX takes 2 cycles
}

pub fn tay(bus: &mut Bus, address_mode: AddressMode) -> u8 {
    // Transfer Accumulator to Y
    log_instruct("TAY", &address_mode, Some(bus));
    let val = bus.cpu.get(Registers::A);
    bus.cpu.set(Registers::Y, val);
    bus.cpu.set_zero(val == 0);
    bus.cpu.set_negative(val & 0x80 != 0);
    bus.increment_pc(&address_mode);
    2 // TAY takes 2 cycles
}

pub fn tsx(bus: &mut Bus, address_mode: AddressMode) -> u8 {
    // Transfer Stack Pointer to X
    log_instruct("TSX", &address_mode, Some(bus));
    let val = bus.cpu.get(Registers::S);
    bus.cpu.set(Registers::X, val);
    bus.cpu.set_zero(val == 0);
    bus.cpu.set_negative(val & 0x80 != 0);
    bus.increment_pc(&address_mode);
    2 // TSX takes 2 cycles
}

pub fn txa(bus: &mut Bus, address_mode: AddressMode) -> u8 {
    // Transfer X to Accumulator
    log_instruct("TXA", &address_mode, Some(bus));
    let val = bus.cpu.get(Registers::X);
    bus.cpu.set(Registers::A, val);
    bus.cpu.set_zero(val == 0);
    bus.cpu.set_negative(val & 0x80 != 0);
    bus.increment_pc(&address_mode);
    2 // TXA takes 2 cycles
}

pub fn txs(bus: &mut Bus, address_mode: AddressMode) -> u8 {
    // Transfer X to Stack Pointer
    log_instruct("TXS", &address_mode, Some(bus));
    let val = bus.cpu.get(Registers::X);
    bus.cpu.set(Registers::S, val);
    bus.increment_pc(&address_mode);
    2 // TXS takes 2 cycles
}

pub fn tya(bus: &mut Bus, address_mode: AddressMode) -> u8 {
    // Transfer Y to Accumulator
    log_instruct("TYA", &address_mode, Some(bus));
    let val = bus.cpu.get(Registers::Y);
    bus.cpu.set(Registers::A, val);
    bus.cpu.set_zero(val == 0);
    bus.cpu.set_negative(val & 0x80 != 0);
    bus.increment_pc(&address_mode);
    2 // TYA takes 2 cycles
}
