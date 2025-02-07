#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Register {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    F,
    SP,
    PC,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Flag {
    Z,
    N,
    H,
    C,
}

fn get_flag_bit(flag: Flag) -> u8 {
    match flag {
        Flag::Z => 1 << 7,
        Flag::N => 1 << 6,
        Flag::H => 1 << 5,
        Flag::C => 1 << 4,
    }
}

impl Register {
    fn new() -> Self {
        Register::A = 0x01;
        Register::B = 0x00;
        Register::C = 0x13;
        Register::D = 0x00;
        Register::E = 0xD8;
        Register::H = 0x01;
        Register::L = 0x4D;
        Register::F = 0xB0;
        Register::SP = 0xFFFE;
        Register::PC = 0x0100;
    }
}
