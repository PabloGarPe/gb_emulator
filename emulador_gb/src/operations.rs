#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Result {
    pub value: u8,
    pub zero: Option<bool>,
    pub add_sub: Option<bool>,
    pub half_carry: Option<bool>,
    pub carry: Option<bool>,
}

fn half_carry_sum(a: u8, b: u8) -> bool {
    ((a & 0x0F) + (b & 0x0F)) & 0x10 == 0x10
}

fn half_carry_sub(a: u8, b: u8) -> bool {
    (a & 0x0F) < (b & 0x0F)
}

pub fn add(a: u8, b: u8) -> Result {
    let (value, carry) = a.overflowing_add(b);
    Result {
        value,
        zero: Some(value == 0),
        add_sub: Some(false),
        half_carry: Some(half_carry_sum(a, b)),
        carry: Some(carry),
    }
}

pub fn adc(a: u8, b: u8, carry: bool) -> Result {
    let (value, carry1) = a.overflowing_add(b);
    let (value, carry2) = value.overflowing_add(if carry { 1 } else { 0 });
    Result {
        value,
        zero: Some(value == 0),
        add_sub: Some(false),
        half_carry: Some(half_carry_sum(a, b)),
        carry: Some(carry1 || carry2),
    }
}

pub fn sub(a: u8, b: u8) -> Result {
    let (value, carry) = a.overflowing_sub(b);
    Result {
        value,
        zero: Some(value == 0),
        add_sub: Some(true),
        half_carry: Some(half_carry_sub(a, b)),
        carry: Some(carry),
    }
}

pub fn sbc(a: u8, b: u8, carry: bool) -> Result {
    let (value, carry1) = a.overflowing_sub(b);
    let (value, carry2) = value.overflowing_sub(if carry { 1 } else { 0 });
    Result {
        value,
        zero: Some(value == 0),
        add_sub: Some(true),
        half_carry: Some(half_carry_sub(a, b)),
        carry: Some(carry1 || carry2),
    }
}

pub fn and(a: u8, b: u8) -> Result {
    let value = a & b;
    Result {
        value,
        zero: Some(value == 0),
        add_sub: Some(false),
        half_carry: Some(true),
        carry: Some(false),
    }
}

pub fn or(a: u8, b: u8) -> Result {
    let value = a | b;
    Result {
        value,
        zero: Some(value == 0),
        add_sub: Some(false),
        half_carry: Some(false),
        carry: Some(false),
    }
}

pub fn xor(a: u8, b: u8) -> Result {
    let value = a ^ b;
    Result {
        value,
        zero: Some(value == 0),
        add_sub: Some(false),
        half_carry: Some(false),
        carry: Some(false),
    }
}

pub fn cp(a: u8, b: u8) -> Result {
    let (_, carry) = a.overflowing_sub(b);
    Result {
        value: a,
        zero: Some(a == b),
        add_sub: Some(true),
        half_carry: Some(half_carry_sub(a, b)),
        carry: Some(carry),
    }
}