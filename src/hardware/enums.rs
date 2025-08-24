#[derive(Debug)]
pub enum Registers {
    A,
    X,
    Y,
    S,
    P,
}

#[derive(Debug)]
pub enum Flags {
    Carry,
    Zero,
    InterruptDisable,
    DecimalMode,
    BreakCommand,
    Overflow,
    Negative,
}
