/*
 * pandocs: https://gbdev.io/pandocs/CPU_Registers_and_Flags.html
 * https://rgbds.gbdev.io/docs/v0.8.0/gbz80.7#LD__r16_,A
 */

// arithmetic instructions can specify a target register.
// some instructions will target an 8bit register, others 16bit
// TODO - do I need the flag register? Probably not.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Add(ArithmeticTarget), // adds what's in target to register A
}
