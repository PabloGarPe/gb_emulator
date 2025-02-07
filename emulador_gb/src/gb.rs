use crate::operations::{inc, dec};
/// Register of the game boy CPU
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Register {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    f: u8,
    sp: u16,
    pc: u16,
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
        Register {
            a: 0x01,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,
            f: 0xB0,
            sp: 0xFFFE,
            pc: 0x0100,
        }
    }
}

/// TODO game boy memory
/// Just a placeholder for now
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Memory {
    data: [u8; 0x10000],
}

/// Implement the Memory struct
impl Memory {
    fn new() -> Self {
        Memory {
            data: [0; 0x10000],
        }
    }
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
        self.registers.f & get_flag_bit(flag) != 0
    }
    
    /// Set the value of a flag
    fn set_flag(&mut self, flag: Flag, value: bool) {
        if value {
            self.registers.f |= get_flag_bit(flag);
        } else {
            self.registers.f &= !get_flag_bit(flag);
        }
    }

    /// Get the value of the registers a and f combined
    fn get_af(&self) -> u16 {
        (self.registers.a as u16) << 8 | self.registers.f as u16
    }

    /// Get the value of the registers b and c combined
    fn get_bc(&self) -> u16 {
        (self.registers.b as u16) << 8 | self.registers.c as u16
    }

    /// Get the value of the registers d and e combined
    fn get_de(&self) -> u16 {
        (self.registers.d as u16) << 8 | self.registers.e as u16
    }

    /// Get the value of the registers h and l combined
    fn get_hl(&self) -> u16 {
        (self.registers.h as u16) << 8 | self.registers.l as u16
    }

    /// Set the value of the registers a and f separately
    fn set_af(&mut self, value: u16) {
        self.registers.a = (value >> 8) as u8;
        self.registers.f = value as u8;
    }

    /// Set the value of the registers b and c separately
    fn set_bc(&mut self, value: u16) {
        self.registers.b = (value >> 8) as u8;
        self.registers.c = value as u8;
    }

    /// Set the value of the registers d and e separately
    fn set_de(&mut self, value: u16) {
        self.registers.d = (value >> 8) as u8;
        self.registers.e = value as u8;
    }

    /// Set the value of the registers h and l separately
    fn set_hl(&mut self, value: u16) {
        self.registers.h = (value >> 8) as u8;
        self.registers.l = value as u8;
    }

    /// Get the value of the next instruction
    fn next_instruction(&mut self) -> u8 {
        let instruction: u8 = self.memory.data[self.registers.pc as usize];
        self.registers.pc = self.registers.pc.wrapping_add(1);
        instruction
    }

    /// Execute the next instruction
    fn execute(&mut self) -> u8{
        match self.next_instruction() {
            0x00 => {
                // NOP
                1
            },
            0x01 => {
                // LD BC, d16
                let value = self.next_instruction() as u16 | (self.next_instruction() as u16) << 8;
                self.set_bc(value);
                1
            },
            0x02 => {
                // LD (BC), A
                self.memory.data[self.get_bc() as usize] = self.registers.a;
                1
            },
            0x03 => {
                // INC BC
                self.set_bc(self.get_bc().wrapping_add(1));
                1
            },
            0x04 => {
                // INC B
                let result = inc(self.registers.b);
                self.registers.b = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, result.add_sub.unwrap());
                self.set_flag(Flag::H, result.half_carry.unwrap());
                1
            },
            0x05 => {
                // DEC B
                let result = dec(self.registers.b);
                self.registers.b = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, result.add_sub.unwrap());
                self.set_flag(Flag::H, result.half_carry.unwrap());
                1
            },
            0x06 => {
                // LD B, d8
                self.registers.b = self.next_instruction();
                1
            },
            0x07 => {
                // RLCA
                let carry = self.registers.a >> 7;
                self.set_flag(Flag::C, carry == 1);
                self.registers.a = (self.registers.a << 1) | carry;
                self.set_flag(Flag::Z, false);
                self.set_flag(Flag::N, false);
                self.set_flag(Flag::H, false);
                1
            },
            0x08 => {
                // LD (a16), SP
                let address = self.next_instruction() as u16 | (self.next_instruction() as u16) << 8;
                self.memory.data[address as usize] = self.registers.sp as u8;
                self.memory.data[(address + 1) as usize] = (self.registers.sp >> 8) as u8;
                1
            },
            0x09 => {
                // ADD HL, BC
                let result = self.get_hl().wrapping_add(self.get_bc());
                self.set_hl(result);
                self.set_flag(Flag::N, false);
                self.set_flag(Flag::H, (self.get_hl() & 0xFFF) + (self.get_bc() & 0xFFF) > 0xFFF);
                self.set_flag(Flag::C, self.get_hl() as u32 + self.get_bc() as u32 > 0xFFFF);
                1
            },
            0x0A => {
                // LD A, (BC)
                self.registers.a = self.memory.data[self.get_bc() as usize];
                1
            },
            0x0B => {
                // DEC BC
                self.set_bc(self.get_bc().wrapping_sub(1));
                1
            },
            0x0C => {
                // INC C
                let result = inc(self.registers.c);
                self.registers.c = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, result.add_sub.unwrap());
                self.set_flag(Flag::H, result.half_carry.unwrap());
                1
            },
            0x0D => {
                // DEC C
                let result = dec(self.registers.c);
                self.registers.c = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, result.add_sub.unwrap());
                self.set_flag(Flag::H, result.half_carry.unwrap());
                1
            },
            0x0E => {
                // LD C, d8
                self.registers.c = self.next_instruction();
                1
            },
            0x0F => {
                // RRCA
                let carry = self.registers.a & 1;
                self.set_flag(Flag::C, carry == 1);
                self.registers.a = (self.registers.a >> 1) | (carry << 7);
                self.set_flag(Flag::Z, false);
                self.set_flag(Flag::N, false);
                self.set_flag(Flag::H, false);
                1
            },
            0x10 => {
                // STOP
                // TODO: Implement Execution of a STOP instruction stops both the system clock and oscillator circuit.
                // STOP mode is entered and the LCD controller also stops. However, the status of the internal RAM register ports remains unchanged.
                // STOP mode can be cancelled by a reset signal.
                // If the RESET terminal goes LOW in STOP mode, it becomes that of a normal reset status.
                // The following conditions should be met before a STOP instruction is executed and stop mode is entered:
                // All interrupt-enable (IE) flags are reset.
                // Input to P10-P13 is LOW for all.
                1
            },
            0x11 => {
                // LD DE, d16
                let value = self.next_instruction() as u16 | (self.next_instruction() as u16) << 8;
                self.set_de(value);
                1
            },
            0x12 => {
                // LD (DE), A
                self.memory.data[self.get_de() as usize] = self.registers.a;
                1
            },
            0x13 => {
                // INC DE
                self.set_de(self.get_de().wrapping_add(1));
                1
            },
            0x14 => {
                // INC D
                let result = inc(self.registers.d);
                self.registers.d = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, result.add_sub.unwrap());
                self.set_flag(Flag::H, result.half_carry.unwrap());
                1
            },
            0x15 => {
                // DEC D
                let result = dec(self.registers.d);
                self.registers.d = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, result.add_sub.unwrap());
                self.set_flag(Flag::H, result.half_carry.unwrap());
                1
            },
            0x16 => {
                // LD D, d8
                self.registers.d = self.next_instruction();
                1
            },
            0x17 => {
                // RLA
                let carry = self.registers.a >> 7;
                self.registers.a = (self.registers.a << 1) | self.get_flag(Flag::C) as u8;
                self.set_flag(Flag::C, carry == 1);
                self.set_flag(Flag::Z, false);
                self.set_flag(Flag::N, false);
                self.set_flag(Flag::H, false);
                1
            },
            0x18 => {
                // JR s8
                let offset = self.next_instruction() as u8 as u16;
                self.registers.pc = self.registers.pc.wrapping_add(offset);
                1
            },
            0x19 => {
                // ADD HL, DE
                let result = self.get_hl().wrapping_add(self.get_de());
                self.set_hl(result);
                self.set_flag(Flag::N, false);
                self.set_flag(Flag::H, (self.get_hl() & 0xFFF) + (self.get_de() & 0xFFF) > 0xFFF);
                self.set_flag(Flag::C, self.get_hl() as u32 + self.get_de() as u32 > 0xFFFF);
                1
            },
            0x1A => {
                // LD A, (DE)
                self.registers.a = self.memory.data[self.get_de() as usize];
                1
            },
            0x1B => {
                // DEC DE
                self.set_de(self.get_de().wrapping_sub(1));
                1
            },
            0x1C => {
                // INC E
                let result = inc(self.registers.e);
                self.registers.e = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, result.add_sub.unwrap());
                self.set_flag(Flag::H, result.half_carry.unwrap());
                1
            },
            0x1D => {
                // DEC E
                let result = dec(self.registers.e);
                self.registers.e = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, result.add_sub.unwrap());
                self.set_flag(Flag::H, result.half_carry.unwrap());
                1
            },
            0x1E => {
                // LD E, d8
                self.registers.e = self.next_instruction();
                1
            },
            0x1F => {
                // RRA
                let carry = self.registers.a & 1;
                self.registers.a = (self.registers.a >> 1) | (self.get_flag(Flag::C) as u8) << 7;
                self.set_flag(Flag::C, carry == 1);
                self.set_flag(Flag::Z, false);
                self.set_flag(Flag::N, false);
                self.set_flag(Flag::H, false);
                1
            },
            _ => {
                // Unhandled instruction
                0
            }
        }
            
        }
}
