use std::time::{Duration, Instant};

use crate::hardware::Hardware;
use crate::hardware::enums::Registers;
use iced::executor;
use iced::widget::{Button, Image, column, image, row, text};
use iced::{Application, Command, Element, Subscription, Theme, time};

#[derive(Default, Debug, Clone)]
pub struct CpuState {
    a: u8,
    x: u8,
    y: u8,
    p: u8,
    s: u8,
    pc: u16,
}

#[derive(Default, Debug, Clone)]
pub struct Nes {
    cpu_state: CpuState,
    emulator: Hardware,
    last_tick: Option<Instant>,
    running: bool,
    chr_1_buffer: Vec<u8>,
    chr_2_buffer: Vec<u8>,
}

#[derive(Debug, Clone)]
pub enum NesMessage {
    Tick,
    LoadRom(String),
    ChangeCpuState(CpuState),
}

impl Application for Nes {
    type Executor = executor::Default;
    type Message = NesMessage;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<NesMessage>) {
        // Initialize the application with a default state.
        (
            Nes {
                cpu_state: CpuState::default(),
                last_tick: None,
                running: false,
                emulator: Hardware::new(),
                chr_1_buffer: Vec::new(),
                chr_2_buffer: Vec::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("NES Emulator")
    }

    // The update method now handles the new `Tick` message
    fn update(&mut self, message: NesMessage) -> Command<NesMessage> {
        match message {
            NesMessage::LoadRom(path) => {
                self.emulator.load_rom(path.as_str()).unwrap_or_else(|err| {
                    eprintln!("Error loading ROM: {}", err);
                });
                self.chr_1_buffer = self.emulator.get_chr_image(0).to_vec();
                self.chr_2_buffer = self.emulator.get_chr_image(1).to_vec();
                self.running = true;
            }
            NesMessage::ChangeCpuState(new_state) => {
                self.cpu_state = new_state;
                println!("CPU state updated: {:?}", self.cpu_state);
            }
            NesMessage::Tick => {
                // This is where you would call your emulator's `tick` function.
                // For this example, we'll just update the PC and A registers.
                if self.running {
                    let now = Instant::now();
                    println!(
                        "Tick! Time since last tick: {:?}",
                        self.last_tick.map(|t| now - t)
                    );
                    self.last_tick = Some(now);
                    self.emulator.tick().unwrap_or_else(|err| {
                        eprintln!("Error during tick: {}", err);
                    });

                    self.cpu_state.a = self.emulator.get_cpu_reg(Registers::A);
                    self.cpu_state.x = self.emulator.get_cpu_reg(Registers::X);
                    self.cpu_state.y = self.emulator.get_cpu_reg(Registers::Y);
                    self.cpu_state.p = self.emulator.get_cpu_reg(Registers::P);
                    self.cpu_state.s = self.emulator.get_cpu_reg(Registers::S);
                    self.cpu_state.pc = self.emulator.get_pc();
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<NesMessage> {
        let cpu_state_text = text(format!(
            "A: {:02X}, X: {:02X}, Y: {:02X}, P: {:02X}, S: {:02X}, PC: {:04X}",
            self.cpu_state.a,
            self.cpu_state.x,
            self.cpu_state.y,
            self.cpu_state.p,
            self.cpu_state.s,
            self.cpu_state.pc
        ));

        let load_button = Button::new(text("Load ROM"))
            .on_press(NesMessage::LoadRom(String::from("roms/super-mario.nes")));

        let chr_1_handle = image::Handle::from_pixels(128, 128, self.chr_1_buffer.clone());
        let chr_2_handle = image::Handle::from_pixels(128, 128, self.chr_2_buffer.clone());

        let chr_1_image = Image::<image::Handle>::new(chr_1_handle)
            .width(512)
            .height(512);
        let chr_2_image = Image::<image::Handle>::new(chr_2_handle)
            .width(512)
            .height(512);

        let col1 = row![cpu_state_text, load_button];
        let col2 = row![chr_1_image, chr_2_image];
        column![col1, col2].into()
    }

    fn subscription(&self) -> Subscription<NesMessage> {
        // This is a common interval for NES emulation (60Hz refresh rate).
        time::every(Duration::from_millis(1000 / 60)).map(|_| NesMessage::Tick)
    }
}
