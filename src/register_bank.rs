#[derive(Default, Copy, Clone, Debug)]
struct RegisterBank {
    // registers
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Register {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum RegisterTwoBytes {
    AF,
    BC,
    DE,
    HL,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Flag {
    Zero,        // z
    Subtraction, // n
    HalfCarry,   // h
    Carry,       // c
}

impl RegisterBank {
    pub fn read(&self, register: Register) -> u8 {
        match register {
            Register::A => self.a,
            Register::B => self.b,
            Register::C => self.c,
            Register::D => self.d,
            Register::E => self.e,
            Register::F => self.f,
            Register::H => self.h,
            Register::L => self.l,
        }
    }

    pub fn write_register(&mut self, register: Register, val: u8) -> Result<(), &str> {
        match register {
            Register::A => self.a = val,
            Register::B => self.b = val,
            Register::C => self.c = val,
            Register::D => self.d = val,
            Register::E => self.e = val,
            Register::F => {
                // do we validate? only the top 4 bits are writable
                if val & 0xF0 > 0 {
                    return Err("Invalid register F value");
                }
                self.f = val;
            }
            Register::H => self.h = val,
            Register::L => self.l = val,
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::register_bank::Register;
    use crate::register_bank::RegisterBank;
    #[test]
    fn test_read() {
        let register_bank = RegisterBank::default();
        assert_eq!(register_bank.read(Register::A), 0);
    }

    #[test]
    fn test_write_should_update_register_bank() {
        let mut register_bank = RegisterBank::default();
        register_bank.write_register(Register::A, 5).unwrap();
        assert_eq!(register_bank.read(Register::A), 5);
    }

    #[test]
    fn test_write_register_valid() {
        let mut register_bank = RegisterBank::default();
        assert!(
            register_bank.write_register(Register::A, 5).is_ok(),
            "Expected ok"
        );
    }

    #[test]
    fn test_write_register_invalid() {
        let mut register_bank = RegisterBank::default();
        let invalid_input: u8 = 0xFF.into();
        assert!(
            register_bank
                .write_register(Register::F, invalid_input)
                .is_err(),
            "Expected an error for writing 0xFF to Register F",
        );
    }
}
