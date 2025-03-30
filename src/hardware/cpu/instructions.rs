use std::fmt::{self, Display, Formatter, UpperHex};

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
    IndexedIndirect,
    IndirectIndexed,
}

impl Display for AddressMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AddressMode::Relative => write!(f, ""),
            AddressMode::Implicit => write!(f, ""),
            AddressMode::Immidiate => write!(f, "#"),
            AddressMode::Accumulator => write!(f, "A"),
            AddressMode::ZeroPage => write!(f, "$"),
            AddressMode::ZeroPageX => write!(f, "$"),
            AddressMode::ZeroPageY => write!(f, "$"),
            AddressMode::Absolute => write!(f, "$"),
            AddressMode::AbsoluteX => write!(f, "$"),
            AddressMode::AbsoluteY => write!(f, "$"),
            AddressMode::Indirect => write!(f, ""),
            AddressMode::IndexedIndirect => write!(f, ""),
            AddressMode::IndirectIndexed => write!(f, ""),
        }
    }
}

fn format_with_address_mode<T: UpperHex>(address_mode: &AddressMode, operand: T) -> String {
    match *address_mode {
        AddressMode::Implicit => "".to_string(),
        AddressMode::Immidiate => format!("#{:X}", operand),
        AddressMode::Relative => format!("{:X}", operand),
        AddressMode::Accumulator => "A".to_string(),
        AddressMode::ZeroPage => format!("${:X}", operand),
        AddressMode::ZeroPageX => format!("${:X},X", operand),
        AddressMode::ZeroPageY => format!("${:X},Y", operand),
        AddressMode::Absolute => format!("${:X}", operand),
        AddressMode::AbsoluteX => format!("${:X},X", operand),
        AddressMode::AbsoluteY => format!("${:X},Y", operand),
        AddressMode::Indirect => format!("(${:X})", operand),
        AddressMode::IndexedIndirect => format!("(${:X},X)", operand),
        AddressMode::IndirectIndexed => format!("(${:X}),Y", operand),
    }
}

fn log_instruct<T: UpperHex>(instruct: &str, address_mode: &AddressMode, operand: T) {
    let address_format = format_with_address_mode(address_mode, operand);
    println!("[ {0} {1} ] {2:=<50}", instruct, address_format, "");
}

pub fn adc(bus: &mut Bus, address_mode: AddressMode, operand: u8) {
    // Add with carry
    log_instruct("ADC", &address_mode, operand);
    bus.increment_pc(&address_mode);
    let (val, _) = bus.read_address_with_mode(address_mode, operand as u16);
    let result: u16 =
        (val as u16) + (bus.cpu.get(Registers::A) as u16) + (bus.cpu.get_carry() as u16);
    bus.cpu.set_carry(result > 0xFF);
    bus.cpu.set_zero(result & 0xFF == 0);
    bus.cpu.set_overflow(
        ((bus.cpu.get(Registers::A) ^ result as u8) & (val ^ result as u8) & 0x80) != 0,
    );
    bus.cpu.set_negative(result & 0x80 != 0);
    bus.cpu.set(Registers::A, result as u8);
}

pub fn and(bus: &mut Bus, address_mode: AddressMode, operand: u8) {
    // Logical AND
    log_instruct("AND", &address_mode, operand);
    bus.increment_pc(&address_mode);
    let (val, _) = bus.read_address_with_mode(address_mode, operand as u16);
    let result = bus.cpu.get(Registers::A) & val;
    bus.cpu.set_zero(result == 0);
    bus.cpu.set_negative(result & 0x80 != 0);
    bus.cpu.set(Registers::A, result);
}

pub fn asl(bus: &mut Bus, address_mode: AddressMode, operand: u8) {
    // Arithmetic shift left
    log_instruct("ASL", &address_mode, operand);
    bus.increment_pc(&address_mode);
    let (val, _) = bus.read_address_with_mode(address_mode, operand as u16);
    let result = val << 1;
    bus.cpu.set_carry(val & 0x80 != 0);
    bus.cpu.set_zero(result == 0);
    bus.cpu.set_negative(result & 0x80 != 0);
    bus.cpu.set(Registers::A, result);
}

pub fn bcc(bus: &mut Bus, address_mode: AddressMode, operand: i8) {
    // Branch if carry clear
    log_instruct("BCC", &address_mode, operand);
    bus.increment_pc(&address_mode);
    if bus.cpu.get_carry() == 0 {
        bus.cpu
            .set_counter((bus.cpu.get_counter() as i16 + operand as i16) as u16);
    }
}

pub fn bcs(bus: &mut Bus, address_mode: AddressMode, operand: i8) {
    // Branch if carry set
    log_instruct("BCS", &address_mode, operand);
    bus.increment_pc(&address_mode);
    if bus.cpu.get_carry() != 0 {
        bus.cpu
            .set_counter((bus.cpu.get_counter() as i16 + operand as i16) as u16);
    }
}

pub fn beq(bus: &mut Bus, address_mode: AddressMode, operand: i8) {
    // Branch if equal
    log_instruct("BEQ", &address_mode, operand);
    bus.increment_pc(&address_mode);
    if bus.cpu.get_zero() != 0 {
        bus.cpu
            .set_counter((bus.cpu.get_counter() as i16 + operand as i16) as u16);
    }
}

pub fn bit(bus: &mut Bus, address_mode: AddressMode, operand: u8) {
    // Bit test
    log_instruct("BIT", &address_mode, operand);
    bus.increment_pc(&address_mode);
    let (val, _) = bus.read_address_with_mode(address_mode, operand as u16);
    let result = bus.cpu.get(Registers::A) & val;
    bus.cpu.set_zero(result == 0);
    bus.cpu.set_overflow(val & 0x40 != 0);
    bus.cpu.set_negative(val & 0x80 != 0);
}

pub fn bmi(bus: &mut Bus, address_mode: AddressMode, operand: i8) {
    // Branch if minus
    log_instruct("BMI", &address_mode, operand);
    bus.increment_pc(&address_mode);
    if bus.cpu.get_negative() != 0 {
        bus.cpu
            .set_counter((bus.cpu.get_counter() as i16 + operand as i16) as u16);
    }
}

pub fn bne(bus: &mut Bus, address_mode: AddressMode, operand: i8) {
    // Branch if not Equal
    log_instruct("BNE", &address_mode, operand);
    bus.increment_pc(&address_mode);
    if bus.cpu.get_zero() == 0 {
        bus.cpu
            .set_counter((bus.cpu.get_counter() as i16 + operand as i16) as u16);
    }
}

pub fn bpl(bus: &mut Bus, address_mode: AddressMode, operand: i8) {
    // Branch if positive
    log_instruct("BPL", &address_mode, operand);
    bus.increment_pc(&address_mode);
    if bus.cpu.get_negative() == 0 {
        bus.cpu
            .set_counter((bus.cpu.get_counter() as i16 + operand as i16) as u16);
    }
}

pub fn brk(bus: &mut Bus, address_mode: AddressMode, operand: u8) {
    // Force intrupt
    log_instruct("BRK", &address_mode, operand);
    bus.increment_pc_by(2);
    bus.stack_push_word(bus.cpu.get_counter());
    bus.stack_push(bus.cpu.get(Registers::P));
    bus.cpu.set_break(true);
    let low_byte = bus.read(0xFFFE) as u16;
    let high_byte = bus.read(0xFFFF) as u16;
    bus.cpu.set_counter((high_byte << 8) + low_byte);
    bus.cpu.set_interrupt_disable(true);
}

pub fn bvc(bus: &mut Bus, address_mode: AddressMode, operand: i8) {
    // Branch if Overflow clear
    log_instruct("BVC", &address_mode, operand);
    bus.increment_pc(&address_mode);
    if bus.cpu.get_overflow() == 0 {
        bus.cpu
            .set_counter(((bus.cpu.get_counter() as i16) + (operand as i16)) as u16);
    }
}

pub fn bvs(bus: &mut Bus, address_mode: AddressMode, operand: i8) {
    // Branch if Overflow set
    log_instruct("BVS", &address_mode, operand);
    bus.increment_pc(&address_mode);
    if bus.cpu.get_overflow() != 0 {
        bus.cpu
            .set_counter(((bus.cpu.get_counter() as i16) + (operand as i16)) as u16);
    }
}

pub fn clc(bus: &mut Bus, address_mode: AddressMode, operand: u8) {
    // Clear Carry Flag
    log_instruct("CLC", &address_mode, operand);
    bus.increment_pc(&address_mode);
    bus.cpu.set_carry(false);
}

pub fn cld(bus: &mut Bus, address_mode: AddressMode, operand: u8) {
    // Clear Decimal Mode
    log_instruct("CLD", &address_mode, operand);
    bus.increment_pc(&address_mode);
    bus.cpu.set_decimal(false);
}

pub fn cli(bus: &mut Bus, address_mode: AddressMode, operand: u8) {
    // Set Interrupt Disable
    log_instruct("CLI", &address_mode, operand);
    bus.increment_pc(&address_mode);
    bus.cpu.set_overflow(false);
}

pub fn clv(bus: &mut Bus, address_mode: AddressMode, operand: u8) {
    // Clear Overflow FLag
    log_instruct("CLV", &address_mode, operand);
    bus.increment_pc(&address_mode);
    bus.cpu.set_overflow(true);
}

pub fn cmp(bus: &mut Bus, address_mode: AddressMode, operand: u8) {
    // Compare
    log_instruct("CMP", &address_mode, operand);
    bus.increment_pc(&address_mode);
    let (val, _) = bus.read_address_with_mode(address_mode, operand as u16);
    let acc = bus.cpu.a;
    let res = acc as i8 - val as i8;
    if res >= 0 {
        bus.cpu.set_carry(true);
        if res == 0 {
            bus.cpu.set_zero(true);
        }
    }
    bus.cpu.set_negative(res < 0);
}

pub fn cpx(bus: &mut Bus, address_mode: AddressMode, operand: u8) {
    // Compare X Register
    log_instruct("CPX", &address_mode, operand);
    bus.increment_pc(&address_mode);
    let (val, _) = bus.read_address_with_mode(address_mode, operand as u16);
    let reg = bus.cpu.x;
    let res = reg as i8 - val as i8;
    if res >= 0 {
        bus.cpu.set_carry(true);
        if res == 0 {
            bus.cpu.set_zero(true);
        }
    }
    bus.cpu.set_negative(res < 0);
}

pub fn cpy(bus: &mut Bus, address_mode: AddressMode, operand: u8) {
    // Compare Y Register
    log_instruct("CPY", &address_mode, operand);
    bus.increment_pc(&address_mode);
    let (val, _) = bus.read_address_with_mode(address_mode, operand as u16);
    let reg = bus.cpu.y;
    let res = reg as i8 - val as i8;
    if res >= 0 {
        bus.cpu.set_carry(true);
        if res == 0 {
            bus.cpu.set_zero(true);
        }
    }
    bus.cpu.set_negative(res < 0);
}

pub fn dec(bus: &mut Bus, address_mode: AddressMode, operand: u16) {
    // Decrement Memory
    log_instruct("DEC", &address_mode, operand);
    bus.increment_pc(&address_mode);
    let is_accumulator = address_mode == AddressMode::Accumulator;
    let (val, address) = bus.read_address_with_mode(address_mode, operand);
    let res = val.wrapping_sub(1);
    bus.cpu.set_negative(res & 0x80 != 0);
    bus.cpu.set_zero(res == 0);
    if is_accumulator {
        bus.cpu.a = res;
    } else {
        bus.write(address, res as u8);
    }
}

pub fn dex(bus: &mut Bus, address_mode: AddressMode, operand: u8) {
    // Decrement X Register
    if address_mode != AddressMode::Implicit {
        panic!("Only Implicit address mode is acceptable for DEX!");
    }
    log_instruct("DEX", &address_mode, operand);
    bus.increment_pc(&address_mode);
    let val = bus.cpu.x;
    let res = val.wrapping_sub(1);
    bus.cpu.set_zero(res == 0);
    bus.cpu.set_negative(res & 0x80 != 0);
    bus.cpu.x = res;
}

pub fn dey(bus: &mut Bus, address_mode: AddressMode, operand: u8) {
    // Decrement Y Register
    if address_mode != AddressMode::Implicit {
        panic!("Only Implicit address mode is acceptable for DEY!");
    }
    log_instruct("DEY", &address_mode, operand);
    bus.increment_pc(&address_mode);
    let val = bus.cpu.y;
    let res = val.wrapping_sub(1);
    bus.cpu.set_zero(res == 0);
    bus.cpu.set_negative(res & 0x80 != 0);
    bus.cpu.y = res;
}

pub fn eor(bus: &mut Bus, address_mode: AddressMode, operand: u8) {
    // Exclusive OR
    log_instruct("EOR", &address_mode, operand);
    bus.increment_pc(&address_mode);
    let acc = bus.cpu.a;
    let (val, _) = bus.read_address_with_mode(address_mode, operand as u16);
    let res = acc ^ val;
    bus.cpu.set_negative(res & 0x80 != 0);
    bus.cpu.set_zero(res == 0);
    bus.cpu.a = res;
}

pub fn inc(bus: &mut Bus, address_mode: AddressMode, operand: u8) {
    // Increment Memory
    log_instruct("INC", &address_mode, operand);
    bus.increment_pc(&address_mode);
    let is_accumulator = address_mode == AddressMode::Accumulator;
    let (val, address) = bus.read_address_with_mode(address_mode, operand as u16);
    let res = val.wrapping_add(1);
    bus.cpu.set_zero(res == 0);
    bus.cpu.set_negative(res & 0x80 != 0);
    if is_accumulator {
        bus.cpu.a = res;
    } else {
        bus.write(address, res);
    }
}

pub fn inx(bus: &mut Bus, address_mode: AddressMode, operand: u8) {
    // Increment X Register
    log_instruct("INX", &address_mode, operand);
    bus.increment_pc(&address_mode);
    let reg = bus.cpu.x;
    let res = reg.wrapping_add(1);
    bus.cpu.set_zero(res == 0);
    bus.cpu.set_negative(res & 0x80 != 0);
    bus.cpu.x = res;
}

pub fn iny(bus: &mut Bus, address_mode: AddressMode, operand: u8) {
    // Increment Y Register
    log_instruct("INY", &address_mode, operand);
    bus.increment_pc(&address_mode);
    let reg = bus.cpu.y;
    let res = reg.wrapping_add(1);
    bus.cpu.set_zero(res == 0);
    bus.cpu.set_negative(res & 0x80 != 0);
    bus.cpu.y = res;
}

pub fn jmp(bus: &mut Bus, address_mode: AddressMode, operand: u16) {
    // Jump
    log_instruct("JMP", &address_mode, operand);
    bus.increment_pc(&address_mode);
    let (_, address) = bus.read_address_with_mode(address_mode, operand);
    bus.cpu.set_counter(address);
}

pub fn jsr(bus: &mut Bus, address_mode: AddressMode, operand: u16) {
    // Jump to Subroutine
    log_instruct("JSR", &address_mode, operand);
    bus.increment_pc(&address_mode);
    let (_, address) = bus.read_address_with_mode(address_mode, operand);
    bus.stack_push_word(bus.cpu.get_counter() - 1);
    bus.cpu.set_counter(address);
}

pub fn lda(bus: &mut Bus, address_mode: AddressMode, operand: u8) {
    // Load Accumulator
    log_instruct("LDA", &address_mode, operand);
    bus.increment_pc(&address_mode);
    let (val, _) = bus.read_address_with_mode(address_mode, operand as u16);
    bus.cpu.set_zero(val == 0);
    bus.cpu.set_negative(val & 0x80 != 0);
    bus.cpu.set(Registers::A, val);
}

pub fn ldx(bus: &mut Bus, address_mode: AddressMode, operand: u8) {
    // Load X Register
    log_instruct("LDX", &address_mode, operand);
    bus.increment_pc(&address_mode);
    let (val, _) = bus.read_address_with_mode(address_mode, operand as u16);
    bus.cpu.set_zero(val == 0);
    bus.cpu.set_negative(val & 0x80 != 0);
    bus.cpu.set(Registers::X, val);
}

pub fn ldy(bus: &mut Bus, address_mode: AddressMode, operand: u8) {
    // Load Y Register
    log_instruct("LDY", &address_mode, operand);
    bus.increment_pc(&address_mode);
    let (val, _) = bus.read_address_with_mode(address_mode, operand as u16);
    bus.cpu.set_zero(val == 0);
    bus.cpu.set_negative(val & 0x80 != 0);
    bus.cpu.set(Registers::Y, val);
}

pub fn lsr(bus: &mut Bus, address_mode: AddressMode, operand: u16) {
    // Logical Shift Right
    log_instruct("LSR", &address_mode, operand);
    bus.increment_pc(&address_mode);
    let is_accumulator = address_mode == AddressMode::Accumulator;
    let (val, address) = bus.read_address_with_mode(address_mode, operand as u16);
    let res = val >> 1;
    bus.cpu.set_carry(val & 0x01 != 0);
    bus.cpu.set_zero(res == 0);
    bus.cpu.set_negative(false);
    if is_accumulator {
        bus.cpu.set(Registers::A, res);
    } else {
        bus.write(address, res);
    }
}

pub fn nop(bus: &mut Bus, address_mode: AddressMode) {
    // No Operation
    log_instruct("NOP", &address_mode, 0x00);
    bus.increment_pc(&address_mode);
}

pub fn ora(bus: &mut Bus, address_mode: AddressMode, operand: u16) {
    // Logical Inclusive OR
    log_instruct("ORA", &address_mode, operand);
    bus.increment_pc(&address_mode);
    let (val, _) = bus.read_address_with_mode(address_mode, operand);
    let res = bus.cpu.get(Registers::A) | val;
    bus.cpu.set_zero(res == 0);
    bus.cpu.set_negative(res & 0x80 != 0);
    bus.cpu.set(Registers::A, res);
}

pub fn pha(bus: &mut Bus, address_mode: AddressMode, operand: u16) {
    // Push Accumulator
    log_instruct("PHA", &address_mode, operand);
    bus.increment_pc(&address_mode);
    bus.stack_push(bus.cpu.get(Registers::A));
}

pub fn php(bus: &mut Bus, address_mode: AddressMode, operand: u16) {
    // Push Processor Status
    log_instruct("PHP", &address_mode, operand);
    bus.increment_pc(&address_mode);
    bus.stack_push(bus.cpu.get(Registers::P));
}

pub fn pla(bus: &mut Bus, address_mode: AddressMode, operand: u16) {
    // Pull Accumulator
    log_instruct("PLA", &address_mode, operand);
    bus.increment_pc(&address_mode);
    let val = bus.stack_pull();
    bus.cpu.set_zero(val == 0);
    bus.cpu.set_negative(val & 0x80 != 0);
    bus.cpu.set(Registers::A, val);
}

pub fn plp(bus: &mut Bus, address_mode: AddressMode, operand: u16) {
    // Pull Processor Status
    log_instruct("PLP", &address_mode, operand);
    bus.increment_pc(&address_mode);
    let mut val = bus.stack_pull();
    let interrupt_disable = val & 0b0000_0100;
    // TODO: Add check in next cpu cycle
    bus.cpu.delayed_interrupt = Some(interrupt_disable != 0);
    val = (val & 0b1111_1011) | (bus.cpu.get(Registers::P) & 0b0000_0100);
    bus.cpu.set(Registers::P, val);
}

pub fn rol(bus: &mut Bus, address_mode: AddressMode, operand: u16) {
    // Rotate Left
    log_instruct("ROL", &address_mode, operand);
    bus.increment_pc(&address_mode);
    let is_accumulator = address_mode == AddressMode::Accumulator;
    let (val, address) = bus.read_address_with_mode(address_mode, operand);
    let res = (val << 1) | bus.cpu.get_carry();
    bus.cpu.set_carry(val & 0x80 != 0);
    bus.cpu.set_zero(res == 0);
    bus.cpu.set_negative(res & 0x80 != 0);
    if is_accumulator {
        bus.cpu.set(Registers::A, res);
    } else {
        bus.write(address, res);
    }
}

pub fn ror(bus: &mut Bus, address_mode: AddressMode, operand: u16) {
    // Rotate Right
    log_instruct("ROR", &address_mode, operand);
    bus.increment_pc(&address_mode);
    let is_accumulator = address_mode == AddressMode::Accumulator;
    let (val, address) = bus.read_address_with_mode(address_mode, operand);
    let res = (val >> 1) | (bus.cpu.get_carry() << 7);
    bus.cpu.set_carry(val & 0x01 != 0);
    bus.cpu.set_zero(res == 0);
    bus.cpu.set_negative(res & 0x80 != 0);
    if is_accumulator {
        bus.cpu.set(Registers::A, res);
    } else {
        bus.write(address, res);
    }
}

pub fn rti(bus: &mut Bus, address_mode: AddressMode, operand: u16) {
    // Return from Interrupt
    log_instruct("RTI", &address_mode, operand);
    let flags = bus.stack_pull();
    bus.cpu.set(Registers::P, flags);
    let pc = bus.stack_pull_word();
    bus.cpu.set_counter(pc);
}

pub fn rts(bus: &mut Bus, address_mode: AddressMode, operand: u16) {
    // Return from Subroutine
    log_instruct("RTS", &address_mode, operand);
    let pc = bus.stack_pull_word();
    bus.cpu.set_counter(pc + 1);
}

pub fn sbc(bus: &mut Bus, address_mode: AddressMode, operand: u16) {
    // Subtract with Carry
    log_instruct("SBC", &address_mode, operand);
    bus.increment_pc(&address_mode);
    let (val, _) = bus.read_address_with_mode(address_mode, operand);
    let acc = bus.cpu.get(Registers::A);
    let carry = bus.cpu.get_carry();
    let result = acc as i16 + !val as i16 + carry as i16;
    bus.cpu.set_carry(result > 0xFF);
    bus.cpu.set_zero(result & 0xFF == 0);
    bus.cpu
        .set_overflow(((acc ^ result as u8) & (!val as u8 ^ result as u8) & 0x80) != 0);
    bus.cpu.set_negative(result & 0x80 != 0);
    bus.cpu.set(Registers::A, result as u8);
}

pub fn sec(bus: &mut Bus, address_mode: AddressMode, operand: u16) {
    // Set Carry Flag
    log_instruct("SEC", &address_mode, operand);
    bus.increment_pc(&address_mode);
    bus.cpu.set_carry(true);
}

pub fn sed(bus: &mut Bus, address_mode: AddressMode, operand: u16) {
    // Set Decimal Flag
    log_instruct("SED", &address_mode, operand);
    bus.increment_pc(&address_mode);
    bus.cpu.set_decimal(true);
}

pub fn sei(bus: &mut Bus, address_mode: AddressMode, operand: u16) {
    // Set Interrupt Disable
    log_instruct("SEI", &address_mode, operand);
    bus.increment_pc(&address_mode);
    bus.cpu.set_interrupt_disable(true);
}

pub fn sta(bus: &mut Bus, address_mode: AddressMode, operand: u16) {
    // Store Accumulator
    log_instruct("STA", &address_mode, operand);
    bus.increment_pc(&address_mode);
    let (_, address) = bus.read_address_with_mode(address_mode, operand);
    bus.write(address, bus.cpu.get(Registers::A));
}

pub fn stx(bus: &mut Bus, address_mode: AddressMode, operand: u16) {
    // Store X Register
    log_instruct("STX", &address_mode, operand);
    bus.increment_pc(&address_mode);
    let (_, address) = bus.read_address_with_mode(address_mode, operand);
    bus.write(address, bus.cpu.get(Registers::X));
}

pub fn sty(bus: &mut Bus, address_mode: AddressMode, operand: u16) {
    // Store Y Register
    log_instruct("STY", &address_mode, operand);
    bus.increment_pc(&address_mode);
    let (_, address) = bus.read_address_with_mode(address_mode, operand);
    bus.write(address, bus.cpu.get(Registers::Y));
}

pub fn tax(bus: &mut Bus, address_mode: AddressMode, operand: u16) {
    // Transfer Accumulator to X
    log_instruct("TAX", &address_mode, operand);
    bus.increment_pc(&address_mode);
    let val = bus.cpu.get(Registers::A);
    bus.cpu.set(Registers::X, val);
    bus.cpu.set_zero(val == 0);
    bus.cpu.set_negative(val & 0x80 != 0);
}

pub fn tay(bus: &mut Bus, address_mode: AddressMode, operand: u16) {
    // Transfer Accumulator to Y
    log_instruct("TAY", &address_mode, operand);
    bus.increment_pc(&address_mode);
    let val = bus.cpu.get(Registers::A);
    bus.cpu.set(Registers::Y, val);
    bus.cpu.set_zero(val == 0);
    bus.cpu.set_negative(val & 0x80 != 0);
}

pub fn tsx(bus: &mut Bus, address_mode: AddressMode, operand: u16) {
    // Transfer Stack Pointer to X
    log_instruct("TSX", &address_mode, operand);
    bus.increment_pc(&address_mode);
    let val = bus.cpu.get(Registers::S);
    bus.cpu.set(Registers::X, val);
    bus.cpu.set_zero(val == 0);
    bus.cpu.set_negative(val & 0x80 != 0);
}

pub fn txa(bus: &mut Bus, address_mode: AddressMode, operand: u16) {
    // Transfer X to Accumulator
    log_instruct("TXA", &address_mode, operand);
    bus.increment_pc(&address_mode);
    let val = bus.cpu.get(Registers::X);
    bus.cpu.set(Registers::A, val);
    bus.cpu.set_zero(val == 0);
    bus.cpu.set_negative(val & 0x80 != 0);
}

pub fn txs(bus: &mut Bus, address_mode: AddressMode, operand: u16) {
    // Transfer X to Stack Pointer
    log_instruct("TXS", &address_mode, operand);
    bus.increment_pc(&address_mode);
    let val = bus.cpu.get(Registers::X);
    bus.cpu.set(Registers::S, val);
}

pub fn tya(bus: &mut Bus, address_mode: AddressMode, operand: u16) {
    // Transfer Y to Accumulator
    log_instruct("TYA", &address_mode, operand);
    bus.increment_pc(&address_mode);
    let val = bus.cpu.get(Registers::Y);
    bus.cpu.set(Registers::A, val);
    bus.cpu.set_zero(val == 0);
    bus.cpu.set_negative(val & 0x80 != 0);
}

pub fn invalid(bus: &mut Bus, address_mode: AddressMode, operand: u16) {
    log_instruct("INVALID", &address_mode, operand);
    panic!("Invalid instruction!");
}
