#![allow(dead_code)]
//! This module contains the registers struct and its methods for the CHIP-8 CPU.

#[derive(Debug, Default)]
#[allow(clippy::module_name_repetitions)]
/// `PsuedoRegisters` are registers that are not accessible to programs but the emulator
pub struct PsuedoRegisters {
    /// This is a 16-bit value used to store the current opcode being executed.
    pub(crate) program_counter: u16,
    /// A byte used to point to the topmost of the stack.
    pub(crate) stack_pointer: u8,
}

#[derive(Debug, Default)]
#[allow(clippy::module_name_repetitions)]
/// `SpecialRegisters` struct contains the delay and sound timers.
pub struct SpecialRegisters {
    /// The delay timer is used for timing events.
    delay_timer: u8,
    /// The sound timer is used for sound effects.
    sound_timer: u8,
}

#[derive(Debug, Default)]
#[allow(clippy::module_name_repetitions)]
/// `GeneralRegisters` struct contains the 16 general purpose registers.
/// They are named V0, V1, ..., VE, VF.
/// Plus a 16-bit register called I.
/// NOTE: The VF register is used as a flag in some instructions.
pub struct GeneralRegisters {
    /// The general purpose registers.
    v: [u8; 16],
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pseudo_registers() {
        let psuedo_registers = PsuedoRegisters {
            program_counter: 0x200,
            stack_pointer: 0,
        };

        assert_eq!(psuedo_registers.program_counter, 0x200);
        assert_eq!(psuedo_registers.stack_pointer, 0);
    }

    #[test]
    fn test_special_registers() {
        let special_registers = SpecialRegisters {
            delay_timer: 0,
            sound_timer: 0,
        };

        assert_eq!(special_registers.delay_timer, 0);
        assert_eq!(special_registers.sound_timer, 0);
    }

    #[test]
    fn test_general_registers() {
        let general_registers = GeneralRegisters {
            v: [0; 16],
        };

        assert_eq!(general_registers.v, [0; 16]);
    }
}
