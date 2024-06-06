#![allow(dead_code)]
//! Here lies the CPU module, which contains the CPU struct and its methods to emulate the CHIP-8 CPU.
/// The registers module contains the ['registers'] struct and its methods.
pub mod registers;
// pub mod opcode;
// pub mod instructions;

/// The CHIP-8 CPU has 4096 bytes of memory.
pub const RAM_SIZE: usize = 4096;

/// The CHIP-8 CPU has 16 levels of stack.
pub const STACK_SIZE: usize = 16;

#[derive(Debug)]
/// The Emu struct is used to emulate the CHIP-8 CPU.
// TODO: consider whether this should be in topmost lib.rs and how API should be structured
pub struct Emu {
    /// Contains the program counter and stack pointer inside a `PsuedoRegisters` struct.
    psuedo_registers: registers::PsuedoRegisters,
    /// Contains the delay and sound timers inside a `SpecialRegisters` struct.
    special_registers: registers::SpecialRegisters,
    /// The CHIP-8 CPU has 16 general purpose registers.
    /// They are named V0, V1, ..., VE, VF.
    /// NOTE: The VF register is used as a flag in some instructions.
    general_registers: registers::GeneralRegisters,
    /// The I register is used to store memory addresses.
    i_register: u16,
    /// The ram size of the CHIP-8 emulator.
    ram: [u8; RAM_SIZE],
    /// The stack is used to store the address that the interpreter should return to when finished with a subroutine.
    stack: [u16; STACK_SIZE],
}

impl Emu {
    /// Where the program counter starts.
    const START_ADDRESS: u16 = 0x200;

    /// The start address of the ETI 660 CHIP-8 interpreter.
    const ETI_START_ADDRESS: u16 = 0x600;

    #[must_use]
    #[allow(clippy::new_without_default)]
    /// Creates a new instance of the Emu struct.
    ///
    /// # Returns
    /// A new instance of the Emu struct.
    pub fn new() -> Self {
        let psuedo_registers = registers::PsuedoRegisters {
            program_counter: Self::START_ADDRESS,
            stack_pointer: 0,
        };

        let special_registers = registers::SpecialRegisters::default();

        let general_registers = registers::GeneralRegisters::default();

        Self {
            psuedo_registers,
            special_registers,
            general_registers,
            i_register: 0,
            ram: [0; RAM_SIZE],
            stack: [0; STACK_SIZE],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let emu = Emu::new();

        assert_eq!(emu.psuedo_registers.program_counter, Emu::START_ADDRESS);
        assert_eq!(emu.psuedo_registers.stack_pointer, 0);
        assert_eq!(emu.i_register, 0);
        assert_eq!(emu.ram, [0; RAM_SIZE]);
        assert_eq!(emu.stack, [0; STACK_SIZE]);
    }
}
