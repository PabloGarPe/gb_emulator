#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Result {
    pub value: u8,
    pub zero: Option<bool>,
    pub add_sub: Option<bool>,
    pub half_carry: Option<bool>,
    pub carry: Option<bool>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Result16 {
    pub value: u16,
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

pub fn inc(value: u8) -> Result{
    add(value,1)
}

pub fn dec(value: u8) -> Result{
    sub(value,1)
}

pub fn add_sp(value:u16, offset:u8) -> Result16{
    let (result, carry) = value.overflowing_add(offset as u16);
    Result16 {
        value: result as u16,
        zero: Some(false),
        add_sub: Some(false),
        half_carry: Some((value & 0x0F) + (offset as u16 & 0x0F) > 0x0F),
        carry: Some(carry),
    }
}

pub fn rlc(value: u8) -> Result{
    let carry = value & 0x80 != 0;
    let result = (value << 1) | (if carry { 1 } else { 0 });
    Result {
        value: result,
        zero: Some(result == 0),
        add_sub: Some(false),
        half_carry: Some(false),
        carry: Some(carry),
    }
}

pub fn rrc(value: u8) -> Result{
    let carry = value & 0x01 != 0;
    let result = (value >> 1) | (if carry { 0x80 } else { 0 });
    Result {
        value: result,
        zero: Some(result == 0),
        add_sub: Some(false),
        half_carry: Some(false),
        carry: Some(carry),
    }
}

pub fn rl(value: u8,flags: u8) -> Result{
    let seven = value >> 7 & 1 != 0;
    let carry = (flags & 0x10) >> 4;
    let result = (value << 1) | carry;
    Result {
        value: result,
        zero: Some(result == 0),
        add_sub: Some(false),
        half_carry: Some(false),
        carry: Some(seven),
    }
}

pub fn rr(value: u8,flags: u8) -> Result{
    let zero = value & 1 != 0;
    let carry = (flags & 0x10) >> 4;
    let result = (value >> 1) | (carry << 7);
    Result {
        value: result,
        zero: Some(result == 0),
        add_sub: Some(false),
        half_carry: Some(false),
        carry: Some(zero),
    }
}

pub fn sla(value:u8) -> Result {
    let carry = value & 0x80 != 0;
    let result = value << 1;
    Result {
        value: result,
        zero: Some(result == 0),
        add_sub: Some(false),
        half_carry: Some(false),
        carry: Some(carry),
    }
}

pub fn sra(value:u8) -> Result {
    let carry = value & 0x80 != 0;
    let result = (value >> 1) | (value & 0x80);
    Result {
        value: result,
        zero: Some(result == 0),
        add_sub: Some(false),
        half_carry: Some(false),
        carry: Some(carry),
    }
}

#[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_add() {
            let result = add(1, 1);
            assert_eq!(result.value, 2);
            assert_eq!(result.zero, Some(false));
            assert_eq!(result.add_sub, Some(false));
            assert_eq!(result.half_carry, Some(false));
            assert_eq!(result.carry, Some(false));
        }

        #[test]
        fn test_adc() {
            let result = adc(1, 1, true);
            assert_eq!(result.value, 3);
            assert_eq!(result.zero, Some(false));
            assert_eq!(result.add_sub, Some(false));
            assert_eq!(result.half_carry, Some(false));
            assert_eq!(result.carry, Some(false));
        }

        #[test]
        fn test_sub() {
            let result = sub(2, 1);
            assert_eq!(result.value, 1);
            assert_eq!(result.zero, Some(false));
            assert_eq!(result.add_sub, Some(true));
            assert_eq!(result.half_carry, Some(false));
            assert_eq!(result.carry, Some(false));
        }

        #[test]
        fn test_sbc() {
            let result = sbc(2, 1, true);
            assert_eq!(result.value, 0);
            assert_eq!(result.zero, Some(true));
            assert_eq!(result.add_sub, Some(true));
            assert_eq!(result.half_carry, Some(false));
            assert_eq!(result.carry, Some(false));
        }

        #[test]
        fn test_and() {
            let result = and(2, 3);
            assert_eq!(result.value, 2);
            assert_eq!(result.zero, Some(false));
            assert_eq!(result.add_sub, Some(false));
            assert_eq!(result.half_carry, Some(true));
            assert_eq!(result.carry, Some(false));
        }

        #[test]
        fn test_or() {
            let result = or(2, 1);
            assert_eq!(result.value, 3);
            assert_eq!(result.zero, Some(false));
            assert_eq!(result.add_sub, Some(false));
            assert_eq!(result.half_carry, Some(false));
            assert_eq!(result.carry, Some(false));
        }

        #[test]
        fn test_xor() {
            let result = xor(2, 3);
            assert_eq!(result.value, 1);
            assert_eq!(result.zero, Some(false));
            assert_eq!(result.add_sub, Some(false));
            assert_eq!(result.half_carry, Some(false));
            assert_eq!(result.carry, Some(false));
        }

        #[test]
        fn test_cp() {
            let result = cp(2, 2);
            assert_eq!(result.value, 2);
            assert_eq!(result.zero, Some(true));
            assert_eq!(result.add_sub, Some(true));
            assert_eq!(result.half_carry, Some(false));
            assert_eq!(result.carry, Some(false));
        }
    }
