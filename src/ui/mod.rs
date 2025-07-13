use gtk4::CssProvider;
use gtk4::Picture;
use gtk4::gdk::Display;
use gtk4::gdk::{MemoryFormat, MemoryTexture};
use gtk4::gdk_pixbuf::Pixbuf;
use gtk4::gio::Cancellable;
use gtk4::glib::ControlFlow;
use gtk4::glib::timeout_add_local;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Button, Label};
use std::cell::RefCell;
use std::rc::Rc;

use crate::hardware::Hardware;
use crate::utils::Color;

#[derive(Debug, Clone)]
struct MainWindow {
    window: ApplicationWindow,
    label: Label,
    memory_buffer: gtk4::TextBuffer,
    instruction_list: gtk4::ListBox,
    chr_picture: Picture,
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

    pub fn run(&mut self) {
        let emulator = Rc::new(RefCell::new(self.emulator.clone()));
        if let Some(main_window) = &self.main_window {
            let main_window = RefCell::new(main_window.clone());
            timeout_add_local(std::time::Duration::from_millis(1000), move || {
                let (asm, line) = emulator.borrow().get_assembly(20);
                UI::build_instruction_list(&main_window, asm, line);
                if let Err(err) = emulator.borrow_mut().tick() {
                    eprintln!("Error during tick: {}", err);
                    main_window
                        .borrow_mut()
                        .label
                        .set_label(&format!("Error: {}", err));
                    ControlFlow::Break
                } else {
                    main_window
                        .borrow_mut()
                        .memory_buffer
                        .set_text(emulator.borrow().get_memory_dump(0, 2048).as_str());
                    ControlFlow::Continue
                }
            });
        }
        self.application.run();
    }

    fn build_instruction_list(
        main_window: &RefCell<MainWindow>,
        instructions: Vec<String>,
        current_line: u16,
    ) {
        let main_window = main_window.borrow_mut();
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

        let vbox = gtk4::Box::new(gtk4::Orientation::Vertical, 5);
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

        let mut image_arr = self.emulator.ppu.get_chr_image(0);
        let pixbuf = UI::create_pixbuf_from_buffer(&mut image_arr, 128, 128);

        let chr_picture = Picture::for_pixbuf(&pixbuf);

        hbox.append(&memory_scrolled_window);
        hbox.append(&instruction_scrolled_window);

        vbox.append(&label);
        vbox.append(&hbox);
        vbox.append(&button);
        vbox.append(&chr_picture);
        window.set_child(Some(&vbox));
        window.set_visible(true);
        self.main_window = Some(MainWindow {
            window,
            label,
            memory_buffer,
            instruction_list,
            chr_picture,
        });

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
}
