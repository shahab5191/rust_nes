mod hardware;
mod ui;

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
        27,   // jMP low byte
        0x00, // jMP high byte
        0x00, // nop
        0x00, // nop
        0x00, // nop
        0x00, // nop
        0x00, // nop
        0x00, // nop
        0x00, // nop
        0x00, // nop
        0x00, // nop
        0x00, // nop
        0x00, // nop
        0x00, // nop
        0x00, // nop
        0x00, // nop
        0x00, // nop
        0x00, // nop
        0x00, // nop
        0x00, // nop
        0x02,
    ]);
    let mut emulator_ui = ui::UI::new(device);
    emulator_ui.run();
}
