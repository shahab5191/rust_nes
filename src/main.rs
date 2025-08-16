use iced::{Application, Settings};
use ui_iced::Nes;

mod hardware;
mod ui_iced;
mod utils;

fn main() {
    Nes::run(Settings::default()).expect("Failed to run NES application");
}
