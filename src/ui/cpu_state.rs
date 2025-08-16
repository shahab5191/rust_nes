use std::collections::HashMap;

use gtk4::Box as GtkBox;
use gtk4::Label;
use gtk4::prelude::*;

pub fn create_cpu_register_box() -> (GtkBox, HashMap<String, Label>) {
    let a_label = Label::new(Some("A: 0x00"));
    a_label.set_widget_name("a_label");
    let x_label = Label::new(Some("X: 0x00"));
    x_label.set_widget_name("x_label");
    let y_label = Label::new(Some("Y: 0x00"));
    y_label.set_widget_name("y_label");
    let p_label = Label::new(Some("P: 0x00"));
    p_label.set_widget_name("p_label");
    let s_label = Label::new(Some("S: 0x00"));
    s_label.set_widget_name("s_label");
    let pc_label = Label::new(Some("PC: 0x0000"));
    pc_label.set_widget_name("pc_label");

    let labels = HashMap::from([
        ("a_label".to_string(), a_label.clone()),
        ("x_label".to_string(), x_label.clone()),
        ("y_label".to_string(), y_label.clone()),
        ("p_label".to_string(), p_label.clone()),
        ("s_label".to_string(), s_label.clone()),
        ("pc_label".to_string(), pc_label.clone()),
    ]);
    let vbox1 = GtkBox::new(gtk4::Orientation::Vertical, 5);
    let vbox2 = GtkBox::new(gtk4::Orientation::Vertical, 5);
    let cpu_box = GtkBox::new(gtk4::Orientation::Horizontal, 5);
    vbox1.append(&a_label);
    vbox1.append(&x_label);
    vbox1.append(&y_label);
    vbox2.append(&p_label);
    vbox2.append(&s_label);
    vbox2.append(&pc_label);
    cpu_box.append(&vbox1);
    cpu_box.append(&vbox2);
    (cpu_box, labels)
}

pub fn update_cpu_register_box(
    registers: &HashMap<String, Label>,
    a: u8,
    x: u8,
    y: u8,
    p: u8,
    s: u8,
    pc: u16,
) {
    registers
        .get("a_label")
        .map(|label| label.set_label(&format!("A: 0x{:02X}", a)));
    registers
        .get("x_label")
        .map(|label| label.set_label(&format!("X: 0x{:02X}", x)));
    registers
        .get("y_label")
        .map(|label| label.set_label(&format!("Y: 0x{:02X}", y)));
    registers
        .get("p_label")
        .map(|label| label.set_label(&format!("P: 0x{:02X}", p)));
    registers
        .get("s_label")
        .map(|label| label.set_label(&format!("S: 0x{:02X}", s)));
    registers
        .get("pc_label")
        .map(|label| label.set_label(&format!("PC: 0x{:04X}", pc)));
}
