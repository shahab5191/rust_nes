mod cpu_state;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use glib::ControlFlow;
use glib::timeout_add_local;
use gtk4::CssProvider;
use gtk4::Picture;
use gtk4::gdk::Display;
use gtk4::gdk_pixbuf::Pixbuf;
use gtk4::gio::Cancellable;
use gtk4::prelude::*;
use gtk4::subclass::application;
use gtk4::{Application, ApplicationWindow, Button, Label};

use crate::hardware::Hardware;
use crate::hardware::enums::Registers;

#[derive(Debug, Clone)]
struct MainWindow {
    window: ApplicationWindow,
    step_button: Button,
    label: Label,
    memory_buffer: gtk4::TextBuffer,
    instruction_list: gtk4::ListBox,
    cpu_registers: HashMap<String, Label>,
}

#[derive(Debug, Clone)]
pub struct UI {
    emulator: Hardware,
    application: Application,
    main_window: Option<MainWindow>,
}

impl UI {
    pub fn new(emulator: Hardware) -> Self {
        let application = Application::builder()
            .application_id("com.example.emulator6502")
            .build();
        let mut ui = UI {
            emulator,
            application,
            main_window: None,
        };
        ui.build_ui();
        ui
    }

    pub fn run(&self) {
        let emulator = Rc::new(RefCell::new(self.emulator.clone()));
        let self_rc = Rc::new(RefCell::new(self.clone()));
        timeout_add_local(std::time::Duration::from_millis(1000), move || {
            if let Err(err) = emulator.borrow_mut().tick() {
                eprintln!("Error during tick: {}", err);
                return ControlFlow::Break;
            }
            self_rc.borrow_mut().update_ui();
            ControlFlow::Continue
        });
        self.application.run();
    }

    fn build_instruction_list(
        main_window: &MainWindow,
        instructions: Vec<String>,
        current_line: u16,
    ) {
        while let Some(child) = main_window.instruction_list.first_child() {
            main_window.instruction_list.remove(&child);
        }
        for (i, instruction) in instructions.iter().enumerate() {
            let label = Label::new(Some(&instruction));
            if i as u16 == current_line {
                label.set_css_classes(&vec!["current-instruction"]);
            } else {
                label.set_css_classes(&vec!["normal-instruction"]);
            }
            main_window.instruction_list.append(&label);
        }
    }

    fn build_ui(&mut self) {
        self.application
            .register(Cancellable::NONE)
            .expect("Failed to register application");
        gtk4::init().expect("Failed to initialize GTK.");
        // Create a new CSS provider
        let provider = CssProvider::new();

        // Load CSS from a file
        provider.load_from_path("assets/style.css");

        // Add the provider to the default display
        let display = Display::default().expect("No display available");
        gtk4::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        let window = ApplicationWindow::builder()
            .application(&self.application)
            .title("6502 Emulator")
            .default_width(800)
            .default_height(600)
            .build();

        let label = Label::new(Some("6502 Emulator Running..."));

        let memory_buffer = gtk4::TextBuffer::new(None);
        let memory_view = gtk4::TextView::new();
        memory_view.set_buffer(Some(&memory_buffer));

        let instruction_list = gtk4::ListBox::new();

        let button = Button::with_label("Click Me");

        let hbox = gtk4::Box::new(gtk4::Orientation::Horizontal, 5);

        let memory_scrolled_window = gtk4::ScrolledWindow::new();
        memory_scrolled_window.set_min_content_height(400);
        memory_scrolled_window.set_max_content_height(400);
        memory_scrolled_window.set_child(Some(&memory_view));
        memory_scrolled_window.set_hexpand(true);

        let instruction_scrolled_window = gtk4::ScrolledWindow::new();
        instruction_scrolled_window.set_min_content_height(400);
        instruction_scrolled_window.set_max_content_height(400);
        instruction_scrolled_window.set_min_content_width(200);
        instruction_scrolled_window.set_child(Some(&instruction_list));

        let mut chr_1_image_arr = self.emulator.get_chr_image(0);
        let mut chr_2_image_arr = self.emulator.get_chr_image(1);

        let chr_1_pixbuf = UI::create_pixbuf_from_buffer(&mut chr_1_image_arr, 128, 128);
        let chr_2_pixbuf = UI::create_pixbuf_from_buffer(&mut chr_2_image_arr, 128, 128);

        let chr_1_picture = Picture::for_pixbuf(&chr_1_pixbuf);
        let chr_2_picture = Picture::for_pixbuf(&chr_2_pixbuf);
        let chr_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 5);
        chr_box.append(&chr_1_picture);
        chr_box.append(&chr_2_picture);
        chr_box.set_halign(gtk4::Align::Center);
        chr_box.set_hexpand(true);
        chr_box.set_vexpand(true);

        let (cpu_state_box, cpu_registers) = cpu_state::create_cpu_register_box();

        let state_box = gtk4::Box::new(gtk4::Orientation::Vertical, 5);
        state_box.set_vexpand(true);
        state_box.append(&label);
        state_box.append(&chr_box);
        state_box.append(&cpu_state_box);
        state_box.append(&button);

        hbox.append(&state_box);
        hbox.append(&instruction_scrolled_window);

        window.set_child(Some(&hbox));
        window.set_visible(true);

        self.update_ui();
        self.main_window = Some(MainWindow {
            window,
            step_button: button,
            label,
            memory_buffer,
            instruction_list,
            cpu_registers,
        });
        let self_ref = Rc::new(RefCell::new(self.clone()));
        let main_window_ref = self_ref.borrow().main_window.clone();
        if let Some(main_window) = main_window_ref {
            main_window.step_button.connect_clicked(move |_| {
                if let Err(err) = self_ref.borrow_mut().emulator.tick() {
                    eprintln!("Error during tick: {}", err);
                    if let Some(main_window) = &self_ref.borrow().main_window {
                        main_window.label.set_label(&format!("Error: {}", err));
                    }
                }
                self_ref.borrow_mut().update_ui();
            });
        }
        self.application.activate();
    }

    fn create_pixbuf_from_buffer(framebuffer: &[u8], width: i32, height: i32) -> Pixbuf {
        let bytes = glib::Bytes::from(framebuffer);
        Pixbuf::from_bytes(
            &bytes,
            gtk4::gdk_pixbuf::Colorspace::Rgb,
            true,
            8,
            width,
            height,
            width * 4,
        )
    }

    // fn tick(&mut self) {
    //     let emulator = Rc::new(RefCell::new(self.emulator.clone()));
    //     if let Some(main_window) = &self.main_window {
    //         let main_window = RefCell::new(main_window.clone());
    //         timeout_add_local(std::time::Duration::from_millis(1000), move || {
    //             let (asm, line) = emulator.borrow().get_assembly(20);
    //             UI::build_instruction_list(&main_window, asm, line);
    //             if let Err(err) = emulator.borrow_mut().tick() {
    //                 eprintln!("Error during tick: {}", err);
    //                 main_window
    //                     .borrow_mut()
    //                     .label
    //                     .set_label(&format!("Error: {}", err));
    //                 ControlFlow::Break
    //             } else {
    //                 main_window
    //                     .borrow_mut()
    //                     .memory_buffer
    //                     .set_text(emulator.borrow().get_memory_dump(0, 2048).as_str());
    //                 ControlFlow::Continue
    //             }
    //         });
    //     }
    // }

    fn update_ui(&mut self) {
        if let Some(win) = &self.main_window {
            win.memory_buffer
                .set_text(self.emulator.get_memory_dump(0, 2048).as_str());
            let (asm, line) = self.emulator.get_assembly(20);
            UI::build_instruction_list(win, asm, line);
            let _ = cpu_state::update_cpu_register_box(
                &win.cpu_registers,
                self.emulator.get_cpu_reg(Registers::A),
                self.emulator.get_cpu_reg(Registers::X),
                self.emulator.get_cpu_reg(Registers::Y),
                self.emulator.get_cpu_reg(Registers::P),
                self.emulator.get_cpu_reg(Registers::S),
                self.emulator.get_pc(),
            );
        } else {
            eprintln!("Main window is not initialized.");
        }
    }
}
