pub struct CPU {
    a: u8,
    x: u8,
    y: u8,
    status: u8,
    pc: u16,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            a: 0,
            x: 0,
            y: 0,
            status: 0,
            pc: 0,
        }
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
        println!("Interpreting")
    }
}
