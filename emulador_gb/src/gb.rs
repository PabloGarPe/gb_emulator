/// Register of the game boy CPU
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

/// Flags of the game boy CPU
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Flag {
    Z,
    N,
    H,
    C,
}

/// Get the bit of a flag
fn get_flag_bit(flag: Flag) -> u8 {
    match flag {
        Flag::Z => 1 << 7,
        Flag::N => 1 << 6,
        Flag::H => 1 << 5,
        Flag::C => 1 << 4,
    }
}

/// Implement the Register struct, setting the values of the registers to the default start values
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

/// TODO game boy memory
/// Just a placeholder for now
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Memory {
    data: [u8; 0x10000],
}

/// CPU struct, containing the registers and memory
pub struct CPU {
    registers: Register,
    memory: Memory,
}

/// Implement the CPU struct
impl CPU{
    
    /// Create a new CPU struct
    pub fn new() -> Self {
        CPU {
            registers: Register::new(),
            memory: Memory::new(),
        }
    }

    /// Get the value of a flag
    fn get_flag(&self, flag: Flag) -> bool {
        self.F & get_flag_bit(flag) != 0
    }
    
    /// Set the value of a flag
    fn set_flag(&mut self, flag: Flag, value: bool) {
        if value {
            self.F |= get_flag_bit(flag);
        } else {
            self.F &= !get_flag_bit(flag);
        }
    }

    /// Get the value of the registers a and f combined
    fn get_af(&self) -> u16 {
        (self.A as u16) << 8 | self.F as u16
    }

    /// Get the value of the registers b and c combined
    fn get_bc(&self) -> u16 {
        (self.B as u16) << 8 | self.C as u16
    }

    /// Get the value of the registers d and e combined
    fn get_de(&self) -> u16 {
        (self.D as u16) << 8 | self.E as u16
    }

    /// Get the value of the registers h and l combined
    fn get_hl(&self) -> u16 {
        (self.H as u16) << 8 | self.L as u16
    }

    /// Set the value of the registers a and f separately
    fn set_af(&mut self, value: u16) {
        self.A = (value >> 8) as u8;
        self.F = value as u8;
    }

    /// Set the value of the registers b and c separately
    fn set_bc(&mut self, value: u16) {
        self.B = (value >> 8) as u8;
        self.C = value as u8;
    }

    /// Set the value of the registers d and e separately
    fn set_de(&mut self, value: u16) {
        self.D = (value >> 8) as u8;
        self.E = value as u8;
    }

    /// Set the value of the registers h and l separately
    fn set_hl(&mut self, value: u16) {
        self.H = (value >> 8) as u8;
        self.L = value as u8;
    }
}
