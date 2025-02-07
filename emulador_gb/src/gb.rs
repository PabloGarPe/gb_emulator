use crate::operations::{add, dec, inc, adc, sub, sbc, and, or, xor, cp};

const MEMORY_SIZE: usize = 65536;
const ROM_BANK_0: usize = 0x0000; // ROM Bank 0 (32KB) HOME BANK
const ROM_BANK_1: usize = 0x4000; // ROM Bank 1 (32KB)
const VRAM: usize = 0x8000; // VRAM (8KB) Background tiles
const CARTRIDGE_RAM:usize = 0xA000;
const WORK_RAM: usize = 0xC000; // RAM Bank 0 (8KB)
// Space not used
const OAM: usize = 0xFE00; // OAM (Sprites) (160 bytes) also tiles
//Space not used
const IO_REGISTERS: usize = 0xFF00; // IO Registros (80 bytes)
const HIGH_RAM: usize = 0xFF80; // Memoria de alto rendimiento (128 bytes) //Acceso un ciclo mas rapido

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
    data: [u8; MEMORY_SIZE],
}

/// Implement the Memory struct
impl Memory {
    fn new() -> Self {
        Memory {
            data: [0; MEMORY_SIZE],
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
    fn execute(&mut self){
        match self.next_instruction() {
            0x00 => {
                // NOP
            },
            0x01 => {
                // LD BC, d16
                let value = self.next_instruction() as u16 | (self.next_instruction() as u16) << 8;
                self.set_bc(value);
                            },
            0x02 => {
                // LD (BC), A
                self.memory.data[self.get_bc() as usize] = self.registers.a;
                            },
            0x03 => {
                // INC BC
                self.set_bc(self.get_bc().wrapping_add(1));
                            },
            0x04 => {
                // INC B
                let result = inc(self.registers.b);
                self.registers.b = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, result.add_sub.unwrap());
                self.set_flag(Flag::H, result.half_carry.unwrap());
                            },
            0x05 => {
                // DEC B
                let result = dec(self.registers.b);
                self.registers.b = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, result.add_sub.unwrap());
                self.set_flag(Flag::H, result.half_carry.unwrap());
                            },
            0x06 => {
                // LD B, d8
                self.registers.b = self.next_instruction();
                            },
            0x07 => {
                // RLCA
                let carry = self.registers.a >> 7;
                self.set_flag(Flag::C, carry == 1);
                self.registers.a = (self.registers.a << 1) | carry;
                self.set_flag(Flag::Z, false);
                self.set_flag(Flag::N, false);
                self.set_flag(Flag::H, false);
                },
            0x08 => {
                // LD (a16), SP
                let address = self.next_instruction() as u16 | (self.next_instruction() as u16) << 8;
                self.memory.data[address as usize] = self.registers.sp as u8;
                self.memory.data[(address + 1) as usize] = (self.registers.sp >> 8) as u8;
                            },
            0x09 => {
                // ADD HL, BC
                let result = self.get_hl().wrapping_add(self.get_bc());
                self.set_hl(result);
                self.set_flag(Flag::N, false);
                self.set_flag(Flag::H, (self.get_hl() & 0xFFF) + (self.get_bc() & 0xFFF) > 0xFFF);
                self.set_flag(Flag::C, self.get_hl() as u32 + self.get_bc() as u32 > 0xFFFF);
                            },
            0x0A => {
                // LD A, (BC)
                self.registers.a = self.memory.data[self.get_bc() as usize];
                            },
            0x0B => {
                // DEC BC
                self.set_bc(self.get_bc().wrapping_sub(1));
            },
            0x0C => {
                // INC C
                let result = inc(self.registers.c);
                self.registers.c = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, result.add_sub.unwrap());
                self.set_flag(Flag::H, result.half_carry.unwrap());
            },
            0x0D => {
                // DEC C
                let result = dec(self.registers.c);
                self.registers.c = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, result.add_sub.unwrap());
                self.set_flag(Flag::H, result.half_carry.unwrap());
            },
            0x0E => {
                // LD C, d8
                self.registers.c = self.next_instruction();
            },
            0x0F => {
                // RRCA
                let carry = self.registers.a & 1;
                self.set_flag(Flag::C, carry == 1);
                self.registers.a = (self.registers.a >> 1) | (carry << 7);
                self.set_flag(Flag::Z, false);
                self.set_flag(Flag::N, false);
                self.set_flag(Flag::H, false);
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
            },
            0x11 => {
                // LD DE, d16
                let value = self.next_instruction() as u16 | (self.next_instruction() as u16) << 8;
                self.set_de(value);
            },
            0x12 => {
                // LD (DE), A
                self.memory.data[self.get_de() as usize] = self.registers.a;
            },
            0x13 => {
                // INC DE
                self.set_de(self.get_de().wrapping_add(1));
            },
            0x14 => {
                // INC D
                let result = inc(self.registers.d);
                self.registers.d = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, result.add_sub.unwrap());
                self.set_flag(Flag::H, result.half_carry.unwrap());
            },
            0x15 => {
                // DEC D
                let result = dec(self.registers.d);
                self.registers.d = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, result.add_sub.unwrap());
                self.set_flag(Flag::H, result.half_carry.unwrap());
            },
            0x16 => {
                // LD D, d8
                self.registers.d = self.next_instruction();
            },
            0x17 => {
                // RLA
                let carry = self.registers.a >> 7;
                self.registers.a = (self.registers.a << 1) | self.get_flag(Flag::C) as u8;
                self.set_flag(Flag::C, carry == 1);
                self.set_flag(Flag::Z, false);
                self.set_flag(Flag::N, false);
                self.set_flag(Flag::H, false);
            },
            0x18 => {
                // JR s8
                let offset = self.next_instruction() as u8 as u16;
                self.registers.pc = self.registers.pc.wrapping_add(offset);
            },
            0x19 => {
                // ADD HL, DE
                let result = self.get_hl().wrapping_add(self.get_de());
                self.set_hl(result);
                self.set_flag(Flag::N, false);
                self.set_flag(Flag::H, (self.get_hl() & 0xFFF) + (self.get_de() & 0xFFF) > 0xFFF);
                self.set_flag(Flag::C, self.get_hl() as u32 + self.get_de() as u32 > 0xFFFF);
            },
            0x1A => {
                // LD A, (DE)
                self.registers.a = self.memory.data[self.get_de() as usize];
            },
            0x1B => {
                // DEC DE
                self.set_de(self.get_de().wrapping_sub(1));
            },
            0x1C => {
                // INC E
                let result = inc(self.registers.e);
                self.registers.e = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, result.add_sub.unwrap());
                self.set_flag(Flag::H, result.half_carry.unwrap());
            },
            0x1D => {
                // DEC E
                let result = dec(self.registers.e);
                self.registers.e = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, result.add_sub.unwrap());
                self.set_flag(Flag::H, result.half_carry.unwrap());
            },
            0x1E => {
                // LD E, d8
                self.registers.e = self.next_instruction();
            },
            0x1F => {
                // RRA
                let carry = self.registers.a & 1;
                self.registers.a = (self.registers.a >> 1) | (self.get_flag(Flag::C) as u8) << 7;
                self.set_flag(Flag::C, carry == 1);
                self.set_flag(Flag::Z, false);
                self.set_flag(Flag::N, false);
                self.set_flag(Flag::H, false);
                
            },
            0x20 => {
                // JR NZ, s8
                let offset = self.next_instruction() as u8 as u16;
                if !self.get_flag(Flag::Z) {
                    self.registers.pc = self.registers.pc.wrapping_add(offset);
                }
            },
            0x21 => {
                // LD HL, d16
                let value = self.next_instruction() as u16 | (self.next_instruction() as u16) << 8;
                self.set_hl(value);
            },
            0x22 => {
                // LDI (HL), A
                self.memory.data[self.get_hl() as usize] = self.registers.a;
                self.set_hl(self.get_hl().wrapping_add(1));
            },
            0x23 => {
                // INC HL
                self.set_hl(self.get_hl().wrapping_add(1));
            },
            0x24 => {
                // INC H
                let result = inc(self.registers.h);
                self.registers.h = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, result.add_sub.unwrap());
                self.set_flag(Flag::H, result.half_carry.unwrap());
            },
            0x25 => {
                // DEC H
                let result = dec(self.registers.h);
                self.registers.h = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, result.add_sub.unwrap());
                self.set_flag(Flag::H, result.half_carry.unwrap());
            },
            0x26 => {
                // LD H, d8
                self.registers.h = self.next_instruction();
            },
            0x27 => {
                // DAA
                let mut a = self.registers.a;
                let mut adjust: u8 = 0;
                if self.get_flag(Flag::H) || (!self.get_flag(Flag::N) && (a & 0xF) > 9){
                    adjust |= 0x06;
                }
                if self.get_flag(Flag::C) || (!self.get_flag(Flag::N) && a > 0x99){
                    adjust |= 0x60;
                    self.set_flag(Flag::C, true);
                }
                if self.get_flag(Flag::N){
                    a = a.wrapping_sub(adjust);
                } else {
                    a = a.wrapping_add(adjust);
                }
                self.set_flag(Flag::Z, a == 0);
                self.set_flag(Flag::H, false);
                self.registers.a = a;
            },
            0x28 => {
                // JR Z, s8
                let offset = self.next_instruction() as u8 as u16;
                if self.get_flag(Flag::Z) {
                    self.registers.pc = self.registers.pc.wrapping_add(offset);
                }
            },
            0x29 => {
                // ADD HL, HL
                let result = self.get_hl().wrapping_add(self.get_hl());
                self.set_hl(result);
                self.set_flag(Flag::N, false);
                self.set_flag(Flag::H, (self.get_hl() & 0xFFF) + (self.get_hl() & 0xFFF) > 0xFFF);
                self.set_flag(Flag::C, self.get_hl() as u32 + self.get_hl() as u32 > 0xFFFF);
            },
            0x2A => {
                // LDI A, (HL)
                self.registers.a = self.memory.data[self.get_hl() as usize];
                self.set_hl(self.get_hl().wrapping_add(1));
            },
            0x2B => {
                // DEC HL
                self.set_hl(self.get_hl().wrapping_sub(1));
            },
            0x2C => {
                // INC L
                let result = inc(self.registers.l);
                self.registers.l = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, result.add_sub.unwrap());
                self.set_flag(Flag::H, result.half_carry.unwrap());
            },
            0x2D => {
                // DEC L
                let result = dec(self.registers.l);
                self.registers.l = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, result.add_sub.unwrap());
                self.set_flag(Flag::H, result.half_carry.unwrap());
            },
            0x2E => {
                // LD L, d8
                self.registers.l = self.next_instruction();
            },
            0x2F => {
                // CPL
                let a = !self.registers.a;
                self.registers.a = a;
                self.set_flag(Flag::N, true);
            },
            0x30 => {
                // JR NC, s8
                let offset = self.next_instruction() as u8 as u16;
                if !self.get_flag(Flag::C) {
                    self.registers.pc = self.registers.pc.wrapping_add(offset);
                }
            },
            0x31 => {
                // LD SP, d16
                let value = self.next_instruction() as u16 | (self.next_instruction() as u16) << 8;
                self.registers.sp = value;
            },
            0x32 => {
                // LDD (HL), A
                self.memory.data[self.get_hl() as usize] = self.registers.a;
                self.set_hl(self.get_hl().wrapping_sub(1));
            },
            0x33 => {
                // INC SP
                self.registers.sp = self.registers.sp.wrapping_add(1);
            },
            0x34 => {
                // INC (HL)
                let result = inc(self.memory.data[self.get_hl() as usize]);
                self.memory.data[self.get_hl() as usize] = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, result.add_sub.unwrap());
                self.set_flag(Flag::H, result.half_carry.unwrap());
            },
            0x35 => {
                // DEC (HL)
                let result = dec(self.memory.data[self.get_hl() as usize]);
                self.memory.data[self.get_hl() as usize] = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, result.add_sub.unwrap());
                self.set_flag(Flag::H, result.half_carry.unwrap());
            },
            0x36 => {
                // LD (HL), d8
                self.memory.data[self.get_hl() as usize] = self.next_instruction();
            },
            0x37 => {
                // SCF
                self.set_flag(Flag::N, false);
                self.set_flag(Flag::H, false);
                self.set_flag(Flag::C, true);
            },
            0x38 => {
                // JR C, s8
                let offset = self.next_instruction() as u8 as u16;
                if self.get_flag(Flag::C) {
                    self.registers.pc = self.registers.pc.wrapping_add(offset);
                }
            },
            0x39 => {
                // ADD HL, SP
                let result = self.get_hl().wrapping_add(self.registers.sp);
                self.set_hl(result);
                self.set_flag(Flag::N, false);
                self.set_flag(Flag::H, (self.get_hl() & 0xFFF) + (self.registers.sp & 0xFFF) > 0xFFF);
                self.set_flag(Flag::C, self.get_hl() as u32 + self.registers.sp as u32 > 0xFFFF);
            },
            0x3A => {
                // LDD A, (HL)
                self.registers.a = self.memory.data[self.get_hl() as usize];
                self.set_hl(self.get_hl().wrapping_sub(1));
            },
            0x3B => {
                // DEC SP
                self.registers.sp = self.registers.sp.wrapping_sub(1);
            },
            0x3C => {
                // INC A
                let result = inc(self.registers.a);
                self.registers.a = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, result.add_sub.unwrap());
                self.set_flag(Flag::H, result.half_carry.unwrap());
            },
            0x3D => {
                // DEC A
                let result = dec(self.registers.a);
                self.registers.a = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, result.add_sub.unwrap());
                self.set_flag(Flag::H, result.half_carry.unwrap());
            },
            0x3E => {
                // LD A, d8
                self.registers.a = self.next_instruction();
            },
            0x3F => {
                // CCF
                let c = self.get_flag(Flag::C);
                self.set_flag(Flag::N, false);
                self.set_flag(Flag::H, false);
                self.set_flag(Flag::C, !c);
            },
            0x40 => {
                // LD B, B
                self.registers.b = self.registers.b;
            },
            0x41 => {
                // LD B, C
                self.registers.b = self.registers.c;
            },
            0x42 => {
                // LD B, D
                self.registers.b = self.registers.d;
            },
            0x43 => {
                // LD B, E
                self.registers.b = self.registers.e;
            },
            0x44 => {
                // LD B, H
                self.registers.b = self.registers.h;
            },
            0x45 => {
                // LD B, L
                self.registers.b = self.registers.l;
            },
            0x46 => {
                // LD B, (HL)
                self.registers.b = self.memory.data[self.get_hl() as usize];
            },
            0x47 => {
                // LD B, A
                self.registers.b = self.registers.a;
            },
            0x48 => {
                // LD C, B
                self.registers.c = self.registers.b;
            },
            0x49 => {
                // LD C, C
                self.registers.c = self.registers.c;
            },
            0x4A => {
                // LD C, D
                self.registers.c = self.registers.d;
            },
            0x4B => {
                // LD C, E
                self.registers.c = self.registers.e;
            },
            0x4C => {
                // LD C, H
                self.registers.c = self.registers.h;
            },
            0x4D => {
                // LD C, L
                self.registers.c = self.registers.l;
            },
            0x4E => {
                // LD C, (HL)
                self.registers.c = self.memory.data[self.get_hl() as usize];
            },
            0x4F => {
                // LD C, A
                self.registers.c = self.registers.a;
            },
            0x50 => {
                // LD D, B
                self.registers.d = self.registers.b;
            },
            0x51 => {
                // LD D, C
                self.registers.d = self.registers.c;
            },
            0x52 => {
                // LD D, D
                self.registers.d = self.registers.d;
            },
            0x53 => {
                // LD D, E
                self.registers.d = self.registers.e;
            },
            0x54 => {
                // LD D, H
                self.registers.d = self.registers.h;
            },
            0x55 => {
                // LD D, L
                self.registers.d = self.registers.l;
            },
            0x56 => {
                // LD D, (HL)
                self.registers.d = self.memory.data[self.get_hl() as usize];
            },
            0x57 => {
                // LD D, A
                self.registers.d = self.registers.a;
            },
            0x58 => {
                // LD E, B
                self.registers.e = self.registers.b;
            },
            0x59 => {
                // LD E, C
                self.registers.e = self.registers.c;
            },
            0x5A => {
                // LD E, D
                self.registers.e = self.registers.d;
            },
            0x5B => {
                // LD E, E
                self.registers.e = self.registers.e;
            },
            0x5C => {
                // LD E, H
                self.registers.e = self.registers.h;
            },
            0x5D => {
                // LD E, L
                self.registers.e = self.registers.l;
            },
            0x5E => {
                // LD E, (HL)
                self.registers.e = self.memory.data[self.get_hl() as usize];
            },
            0x5F => {
                // LD E, A
                self.registers.e = self.registers.a;
            },
            0x60 => {
                // LD H, B
                self.registers.h = self.registers.b;
            },
            0x61 => {
                // LD H, C
                self.registers.h = self.registers.c;
            },
            0x62 => {
                // LD H, D
                self.registers.h = self.registers.d;
            },
            0x63 => {
                // LD H, E
                self.registers.h = self.registers.e;
            },
            0x64 => {
                // LD H, H
                self.registers.h = self.registers.h;
            },
            0x65 => {
                // LD H, L
                self.registers.h = self.registers.l;
            },
            0x66 => {
                // LD H, (HL)
                self.registers.h = self.memory.data[self.get_hl() as usize];
            },
            0x67 => {
                // LD H, A
                self.registers.h = self.registers.a;
            },
            0x68 => {
                // LD L, B
                self.registers.l = self.registers.b;
            },
            0x69 => {
                // LD L, C
                self.registers.l = self.registers.c;
            },
            0x6A => {
                // LD L, D
                self.registers.l = self.registers.d;
            },
            0x6B => {
                // LD L, E
                self.registers.l = self.registers.e;
            },
            0x6C => {
                // LD L, H
                self.registers.l = self.registers.h;
            },
            0x6D => {
                // LD L, L
                self.registers.l = self.registers.l;
            },
            0x6E => {
                // LD L, (HL)
                self.registers.l = self.memory.data[self.get_hl() as usize];
            },
            0x6F => {
                // LD L, A
                self.registers.l = self.registers.a;
            },
            0x70 => {
                // LD (HL), B
                self.memory.data[self.get_hl() as usize] = self.registers.b;
            },
            0x71 => {
                // LD (HL), C
                self.memory.data[self.get_hl() as usize] = self.registers.c;
            },
            0x72 => {
                // LD (HL), D
                self.memory.data[self.get_hl() as usize] = self.registers.d;
            },
            0x73 => {
                // LD (HL), E
                self.memory.data[self.get_hl() as usize] = self.registers.e;
            },
            0x74 => {
                // LD (HL), H
                self.memory.data[self.get_hl() as usize] = self.registers.h;
            },
            0x75 => {
                // LD (HL), L
                self.memory.data[self.get_hl() as usize] = self.registers.l;
            },
            0x76 =>{
                // HALT
                //TODO After a HALT instruction is executed, the system clock is stopped and HALT mode is entered. Although the system clock is stopped in this status, the oscillator circuit and LCD controller continue to operate.

                // In addition, the status of the internal RAM register ports remains unchanged.

                // HALT mode is cancelled by an interrupt or reset signal.

                // The program counter is halted at the step after the HALT instruction. If both the interrupt request flag and the corresponding interrupt enable flag are set, HALT mode is exited, even if the interrupt master enable flag is not set.

                // Once HALT mode is cancelled, the program starts from the address indicated by the program counter.

                // If the interrupt master enable flag is set, the contents of the program coounter are pushed to the stack and control jumps to the starting address of the interrupt.

                // If the RESET terminal goes LOW in HALT mode, the mode becomes that of a normal reset.
            },
            0x77 => {
                // LD (HL), A
                self.memory.data[self.get_hl() as usize] = self.registers.a;
            },
            0x78 => {
                // LD A, B
                self.registers.a = self.registers.b;
            },
            0x79 => {
                // LD A, C
                self.registers.a = self.registers.c;
            },
            0x7A => {
                // LD A, D
                self.registers.a = self.registers.d;
            },
            0x7B => {
                // LD A, E
                self.registers.a = self.registers.e;
            },
            0x7C => {
                // LD A, H
                self.registers.a = self.registers.h;
            },
            0x7D => {
                // LD A, L
                self.registers.a = self.registers.l;
            },
            0x7E => {
                // LD A, (HL)
                self.registers.a = self.memory.data[self.get_hl() as usize];
            },
            0x7F => {
                // LD A, A
                self.registers.a = self.registers.a;
            },
            0x80 => {
                // ADD A, B
                let result = add(self.registers.a, self.registers.b);
                self.registers.a = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, false);
                self.set_flag(Flag::H, result.half_carry.unwrap());
                self.set_flag(Flag::C, result.carry.unwrap());
            }
            0x81 => {
                // ADD A, C
                let result = add(self.registers.a, self.registers.c);
                self.registers.a = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, false);
                self.set_flag(Flag::H, result.half_carry.unwrap());
                self.set_flag(Flag::C, result.carry.unwrap());
            },
            0x82 => {
                // ADD A, D
                let result = add(self.registers.a, self.registers.d);
                self.registers.a = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, false);
                self.set_flag(Flag::H, result.half_carry.unwrap());
                self.set_flag(Flag::C, result.carry.unwrap());
            },
            0x83 => {
                // ADD A, E
                let result = add(self.registers.a, self.registers.e);
                self.registers.a = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, false);
                self.set_flag(Flag::H, result.half_carry.unwrap());
                self.set_flag(Flag::C, result.carry.unwrap());
            },
            0x84 => {
                // ADD A, H
                let result = add(self.registers.a, self.registers.h);
                self.registers.a = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, false);
                self.set_flag(Flag::H, result.half_carry.unwrap());
                self.set_flag(Flag::C, result.carry.unwrap());
            },
            0x85 => {
                // ADD A, L
                let result = add(self.registers.a, self.registers.l);
                self.registers.a = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, false);
                self.set_flag(Flag::H, result.half_carry.unwrap());
                self.set_flag(Flag::C, result.carry.unwrap());
            },
            0x86 => {
                // ADD A, (HL)
                let result = add(self.registers.a, self.memory.data[self.get_hl() as usize]);
                self.registers.a = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, false);
                self.set_flag(Flag::H, result.half_carry.unwrap());
                self.set_flag(Flag::C, result.carry.unwrap());
            },
            0x87 => {
                // ADD A, A
                let result = add(self.registers.a, self.registers.a);
                self.registers.a = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, false);
                self.set_flag(Flag::H, result.half_carry.unwrap());
                self.set_flag(Flag::C, result.carry.unwrap());
            },
            0x88 => {
                // ADC A, B
                let result = adc(self.registers.a, self.registers.b, self.get_flag(Flag::C));
                self.registers.a = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, false);
                self.set_flag(Flag::H, result.half_carry.unwrap());
                self.set_flag(Flag::C, result.carry.unwrap());
            },
            0x89 => {
                // ADC A, C
                let result = adc(self.registers.a, self.registers.c, self.get_flag(Flag::C));
                self.registers.a = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, false);
                self.set_flag(Flag::H, result.half_carry.unwrap());
                self.set_flag(Flag::C, result.carry.unwrap());
            },
            0x8A => {
                // ADC A, D
                let result = adc(self.registers.a, self.registers.d, self.get_flag(Flag::C));
                self.registers.a = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, false);
                self.set_flag(Flag::H, result.half_carry.unwrap());
                self.set_flag(Flag::C, result.carry.unwrap());
            },
            0x8B => {
                // ADC A, E
                let result = adc(self.registers.a, self.registers.e, self.get_flag(Flag::C));
                self.registers.a = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, false);
                self.set_flag(Flag::H, result.half_carry.unwrap());
                self.set_flag(Flag::C, result.carry.unwrap());
            },
            0x8C => {
                // ADC A, H
                let result = adc(self.registers.a, self.registers.h, self.get_flag(Flag::C));
                self.registers.a = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, false);
                self.set_flag(Flag::H, result.half_carry.unwrap());
                self.set_flag(Flag::C, result.carry.unwrap());
            },
            0x8D => {
                // ADC A, L
                let result = adc(self.registers.a, self.registers.l, self.get_flag(Flag::C));
                self.registers.a = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, false);
                self.set_flag(Flag::H, result.half_carry.unwrap());
                self.set_flag(Flag::C, result.carry.unwrap());
            },
            0x8E => {
                // ADC A, (HL)
                let result = adc(self.registers.a, self.memory.data[self.get_hl() as usize], self.get_flag(Flag::C));
                self.registers.a = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, false);
                self.set_flag(Flag::H, result.half_carry.unwrap());
                self.set_flag(Flag::C, result.carry.unwrap());
            },
            0x8F => {
                // ADC A, A
                let result = adc(self.registers.a, self.registers.a, self.get_flag(Flag::C));
                self.registers.a = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, false);
                self.set_flag(Flag::H, result.half_carry.unwrap());
                self.set_flag(Flag::C, result.carry.unwrap());
            },
            0x90 => {
                // SUB B
                let result = sub(self.registers.a, self.registers.b);
                self.registers.a = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, true);
                self.set_flag(Flag::H, result.half_carry.unwrap());
                self.set_flag(Flag::C, result.carry.unwrap());
            },
            0x91 => {
                // SUB C
                let result = sub(self.registers.a, self.registers.c);
                self.registers.a = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, true);
                self.set_flag(Flag::H, result.half_carry.unwrap());
                self.set_flag(Flag::C, result.carry.unwrap());
            },
            0x92 => {
                // SUB D
                let result = sub(self.registers.a, self.registers.d);
                self.registers.a = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, true);
                self.set_flag(Flag::H, result.half_carry.unwrap());
                self.set_flag(Flag::C, result.carry.unwrap());
            },  
            0x93 => {
                // SUB E
                let result = sub(self.registers.a, self.registers.e);
                self.registers.a = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, true);
                self.set_flag(Flag::H, result.half_carry.unwrap());
                self.set_flag(Flag::C, result.carry.unwrap());
            },
            0x94 => {
                // SUB H
                let result = sub(self.registers.a, self.registers.h);
                self.registers.a = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, true);
                self.set_flag(Flag::H, result.half_carry.unwrap());
                self.set_flag(Flag::C, result.carry.unwrap());
            },
            0x95 => {
                // SUB L
                let result = sub(self.registers.a, self.registers.l);
                self.registers.a = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, true);
                self.set_flag(Flag::H, result.half_carry.unwrap());
                self.set_flag(Flag::C, result.carry.unwrap());
            },
            0x96 =>{
                // SUB (HL)
                let result = sub(self.registers.a, self.memory.data[self.get_hl() as usize]);
                self.registers.a = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, true);
                self.set_flag(Flag::H, result.half_carry.unwrap());
                self.set_flag(Flag::C, result.carry.unwrap());
            },
            0x97 => {
                // SUB A
                let result = sub(self.registers.a, self.registers.a);
                self.registers.a = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, true);
                self.set_flag(Flag::H, result.half_carry.unwrap());
                self.set_flag(Flag::C, result.carry.unwrap());
            },
            0x98 => {
                // SBC A, B
                let result = sbc(self.registers.a, self.registers.b, self.get_flag(Flag::C));
                self.registers.a = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, true);
                self.set_flag(Flag::H, result.half_carry.unwrap());
                self.set_flag(Flag::C, result.carry.unwrap());
            },
            0x99 => {
                // SBC A, C
                let result = sbc(self.registers.a, self.registers.c, self.get_flag(Flag::C));
                self.registers.a = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, true);
                self.set_flag(Flag::H, result.half_carry.unwrap());
                self.set_flag(Flag::C, result.carry.unwrap());
            },
            0x9A => {
                // SBC A, D
                let result = sbc(self.registers.a, self.registers.d, self.get_flag(Flag::C));
                self.registers.a = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, true);
                self.set_flag(Flag::H, result.half_carry.unwrap());
                self.set_flag(Flag::C, result.carry.unwrap());
            },
            0x9B => {
                // SBC A, E
                let result = sbc(self.registers.a, self.registers.e, self.get_flag(Flag::C));
                self.registers.a = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, true);
                self.set_flag(Flag::H, result.half_carry.unwrap());
                self.set_flag(Flag::C, result.carry.unwrap());
            },
            0x9C => {
                // SBC A, H
                let result = sbc(self.registers.a, self.registers.h, self.get_flag(Flag::C));
                self.registers.a = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, true);
                self.set_flag(Flag::H, result.half_carry.unwrap());
                self.set_flag(Flag::C, result.carry.unwrap());
            },
            0x9D => {
                // SBC A, L
                let result = sbc(self.registers.a, self.registers.l, self.get_flag(Flag::C));
                self.registers.a = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, true);
                self.set_flag(Flag::H, result.half_carry.unwrap());
                self.set_flag(Flag::C, result.carry.unwrap());
            },
            0x9E => {
                // SBC A, (HL)
                let result = sbc(self.registers.a, self.memory.data[self.get_hl() as usize], self.get_flag(Flag::C));
                self.registers.a = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, true);
                self.set_flag(Flag::H, result.half_carry.unwrap());
                self.set_flag(Flag::C, result.carry.unwrap());
            },
            0x9F => {
                // SBC A, A
                let result = sbc(self.registers.a, self.registers.a, self.get_flag(Flag::C));
                self.registers.a = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, true);
                self.set_flag(Flag::H, result.half_carry.unwrap());
                self.set_flag(Flag::C, result.carry.unwrap());
            },
            0xA0 => {
                // AND B
                let result = and(self.registers.a, self.registers.b);
                self.registers.a = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, false);
                self.set_flag(Flag::H, true);
                self.set_flag(Flag::C, false);
            },
            0xA1 => {
                // AND C
                let result = and(self.registers.a, self.registers.c);
                self.registers.a = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, false);
                self.set_flag(Flag::H, true);
                self.set_flag(Flag::C, false);
            },
            0xA2 => {
                // AND D
                let result = and(self.registers.a, self.registers.d);
                self.registers.a = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, false);
                self.set_flag(Flag::H, true);
                self.set_flag(Flag::C, false);
            },
            0xA3 => {
                // AND E
                let result = and(self.registers.a, self.registers.e);
                self.registers.a = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, false);
                self.set_flag(Flag::H, true);
                self.set_flag(Flag::C, false);
            },
            0xA4 => {
                // AND H
                let result = and(self.registers.a, self.registers.h);
                self.registers.a = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, false);
                self.set_flag(Flag::H, true);
                self.set_flag(Flag::C, false);
            },
            0xA5 => {
                // AND L
                let result = and(self.registers.a, self.registers.l);
                self.registers.a = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, false);
                self.set_flag(Flag::H, true);
                self.set_flag(Flag::C, false);
            },
            0xA6 => {
                // AND (HL)
                let result = and(self.registers.a, self.memory.data[self.get_hl() as usize]);
                self.registers.a = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, false);
                self.set_flag(Flag::H, true);
                self.set_flag(Flag::C, false);
            },
            0xA7 => {
                // AND A
                let result = and(self.registers.a, self.registers.a);
                self.registers.a = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, false);
                self.set_flag(Flag::H, true);
                self.set_flag(Flag::C, false);
            },
            0xA8 => {
                // XOR B
                let result = xor(self.registers.a, self.registers.b);
                self.registers.a = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, false);
                self.set_flag(Flag::H, false);
                self.set_flag(Flag::C, false);
            },
            0xA9 => {
                // XOR C
                let result = xor(self.registers.a, self.registers.c);
                self.registers.a = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, false);
                self.set_flag(Flag::H, false);
                self.set_flag(Flag::C, false);
            },
            0xAA => {
                // XOR D
                let result = xor(self.registers.a, self.registers.d);
                self.registers.a = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, false);
                self.set_flag(Flag::H, false);
                self.set_flag(Flag::C, false);
            },
            0xAB => {
                // XOR E
                let result = xor(self.registers.a, self.registers.e);
                self.registers.a = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, false);
                self.set_flag(Flag::H, false);
                self.set_flag(Flag::C, false);
            },
            0xAC => {
                // XOR H
                let result = xor(self.registers.a, self.registers.h);
                self.registers.a = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, false);
                self.set_flag(Flag::H, false);
                self.set_flag(Flag::C, false);
            },
            0xAD => {
                // XOR L
                let result = xor(self.registers.a, self.registers.l);
                self.registers.a = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, false);
                self.set_flag(Flag::H, false);
                self.set_flag(Flag::C, false);
            },
            0xAE => {
                // XOR (HL)
                let result = xor(self.registers.a, self.memory.data[self.get_hl() as usize]);
                self.registers.a = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, false);
                self.set_flag(Flag::H, false);
                self.set_flag(Flag::C, false);
            },
            0xAF => {
                // XOR A
                let result = xor(self.registers.a, self.registers.a);
                self.registers.a = result.value;
                self.set_flag(Flag::Z, result.zero.unwrap());
                self.set_flag(Flag::N, false);
                self.set_flag(Flag::H, false);
                self.set_flag(Flag::C, false);
            },
            _ => {
                // Unhandled instruction
                
            }
        }
            
        }
}
