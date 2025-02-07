#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Register {
    A: u8,
    B: u8,
    C: u8,
    D: u8,
    E: u8,
    H: u8,
    L: u8,
    F: u8,
    SP: u16,
    PC: u16,
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



pub struct CPU {
    registers: Register,
}

impl CPU{
    pub fn new() -> Self {
        CPU {
            registers: Register::new(),
        }
    }
    fn get_flag(&self, flag: Flag) -> bool {
        self.F & get_flag_bit(flag) != 0
    }
    
    fn set_flag(&mut self, flag: Flag, value: bool) {
        if value {
            self.F |= get_flag_bit(flag);
        } else {
            self.F &= !get_flag_bit(flag);
        }
    }

    fn get_af(&self) -> u16 {
        (self.A as u16) << 8 | self.F as u16
    }

    fn get_bc(&self) -> u16 {
        (self.B as u16) << 8 | self.C as u16
    }

    fn get_de(&self) -> u16 {
        (self.D as u16) << 8 | self.E as u16
    }

    fn get_hl(&self) -> u16 {
        (self.H as u16) << 8 | self.L as u16
    }

}


