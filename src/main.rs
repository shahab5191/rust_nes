mod hardware;

fn main() {
    let mut device = hardware::Hardware::new();
    device.load_program(vec![
        0xE6, // INC Zero Page
        0x06, // Zero Page Address
        0xA2, // LDX Immediate
        0x02, // Immediate Value
        0x86, // STX Zero Page
        0x00, // Zero Page Address
        0x4B, // Jmp after INC
        0x00, // jMP low byte
        0x00, // jMP high byte
    ]);
    device.run();
}
