mod hardware;
mod ui;
mod utils;

fn main() {
    let mut device = hardware::Hardware::new();
    device
        .load_rom("roms/full_palette.nes")
        .expect("Failed to load ROM");
    let mut emulator_ui = ui::UI::new(device);
    emulator_ui.run();
}
