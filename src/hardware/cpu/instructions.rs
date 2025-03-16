use crate::hardware::{CPU, memory::Memory};

#[derive(PartialEq, Eq)]
pub enum AddressMode {
    immidiate,
    zero_page,
    zero_page_x,
    zero_page_y,
    absolute_x,
    absolute_y,
    indexed_indirect,
    indirect_indexed,
}

fn ADC(cpu: &mut CPU, memory: Memory, address_mode: AddressMode, operand: u8) {
    // Add with carry
    let mut value: u8;
    if address_mode == AddressMode::immidiate {
        value = operand
    } else {
        let address = cpu.get_address_with_mode(address_mode, operand);
        value = memory.get(address);
    }

    cpu.a = cpu.a + value;
}
