use crate::instruction::ArithmeticTarget;
use crate::instruction::Instruction;
use crate::register_bank::Register;
use crate::register_bank::RegisterBank;

#[derive(Debug, Clone, Copy)]
struct Cpu {
    registers: RegisterBank,
}

impl Cpu {
    pub fn exec(&mut self, ins: Instruction) {
        match ins {
            Instruction::Add(target) => match target {
                ArithmeticTarget::A => self.add(Register::A),
                ArithmeticTarget::B => self.add(Register::B),
                ArithmeticTarget::C => self.add(Register::C),
                ArithmeticTarget::D => self.add(Register::D),
                ArithmeticTarget::E => self.add(Register::E),
                ArithmeticTarget::H => self.add(Register::H),
                ArithmeticTarget::L => self.add(Register::L),
            },
        }
    }

    // helpers
    fn add(&mut self, reg: Register) {
        // get value from reg, overflowing_add, write to A
        let v = self.registers.read(reg);
        let old = self.registers.read(Register::A);
        let (new_v, overflow) = old.overflowing_add(v);
        // set flags
        self.registers.set_zero_bit(new_v == 0);
        self.registers.set_subtraction_bit(false);
        self.registers.set_carry_bit(overflow);
        // half-carry is set if the lower nibbles would carry
        let lower_carry = (v & 0xF) + (old & 0xF) > 0xF;
        self.registers.set_half_carry_bit(lower_carry);
        // we write back to accumulator register
        self.registers.write_register(Register::A, new_v).unwrap(); //todo
    }
}
