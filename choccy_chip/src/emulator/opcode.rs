//! This module contains the `OpCode` enum which represents the different opcodes that the CHIP-8 emulator can execute.
//! Additionally, it contains the `OpCodeError` enum which represents the different errors that can occur when executing an opcode.
//! Finally, it implments methods for the `OpCode` enum.
use super::Emu;
type Address = u16; // an address
type Constant = u8; // a 8 bit constant
type Case = u8; // represents a number that can be used in a switch statement
type Register = u8; // a 4 bit register number

/// The `OpCode` enum represents the different opcodes that the CHIP-8 emulator can execute.
/// There are 35 different opcodes in total.
#[derive(Debug)]
pub(crate) enum OpCode {
    Nop,
    Call(Address),
    BitOp((Register, Register, Case)),
    Unknown,
}

impl From<u16> for OpCode {
    fn from(value: u16) -> Self {
        let digits = (
            (value & 0xF000) >> 12, // First digit
            (value & 0x0F00) >> 8,  // Second digit
            (value & 0x00F0) >> 4,  // Third digit
            value & 0x000F,         // Fourth digit
        );

        match digits {
            (0, 0, 0, 0) => OpCode::Nop,
            (0, _, _, _) => OpCode::Call(value & 0x0FFF), // Get rid of the first digit
            (8, register_x, register_y, constant) => {
                let args = (
                    u8::try_from(register_x).expect("Invalid register number"),
                    u8::try_from(register_y).expect("Invalid register number"),
                    u8::try_from(constant).expect("Invalid constant"),
                );
                OpCode::BitOp(args)
            }
            _ => OpCode::Unknown,
        }
    }
}

impl Emu {
    #[must_use]
    /// Fetch the value from our game (loaded into RAM) at the memory address stored in our Program Counter.
    pub(crate) fn fetch_opcode(&self) -> OpCode {
        let pc = self.psuedo_registers.program_counter as usize;

        // An OpCode is 2 bytes long
        let lower_byte = u16::from(self.ram[pc]);
        let upper_byte = u16::from(self.ram[pc + 1]);
        let opcode = (upper_byte << 8) | lower_byte;
        OpCode::from(opcode)
    }

    /// Execute an `OpCode`.
    ///
    /// # Arguments
    ///
    /// - `OpCode`: The `OpCode` to execute.
    pub(crate) fn execute_opcode(&mut self, opcode: &OpCode) {
        match opcode {
            OpCode::Nop => {}
            &OpCode::Call(address) => todo!(),
            OpCode::BitOp(args) => self.handle_bit_op(*args),
            OpCode::Unknown => unreachable!(),
        }
    }

    fn handle_bit_op(&self, (register_x, register_y, constant): (Register, Register, Case)) {
        todo!()
    }
}
