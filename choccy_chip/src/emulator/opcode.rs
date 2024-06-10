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
    SkipEquals((Case, Register, Constant)),
    SkipRegisterEquals((Case, Register, Register)),
    Display(Option<(Constant, Constant, Constant)>), // Whether to clear or draw
    Return,                                          // NOTE: technically a flow control instruction
    Call(Address),                                   // NOTE: This is deprecated
    Flow(Case, Address),
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
            (3 | 4, register, _, _) => {
                let args = (
                    u8::try_from(digits.0).expect("Invalid case"),
                    u8::try_from(register).expect("Invalid register number"),
                    u8::try_from(value & 0x00FF).expect("Invalid constant"),
                );
                OpCode::SkipEquals(args)
            }
            (5 | 9, register_x, register_y, 0) => {
                let args = (
                    u8::try_from(digits.0).expect("Invalid case"),
                    u8::try_from(register_x).expect("Invalid register number"),
                    u8::try_from(register_y).expect("Invalid register number"),
                );
                OpCode::SkipRegisterEquals(args)
            }
            (0, 0, 0xE, 0) => OpCode::Display(None),
            (0, 0, 0xE, 0xE) => OpCode::Return, // technically a flow control instruction
            (1 | 2 | 0xB, _, _, _) => {
                let flow_case = u8::try_from(digits.0).expect("Invalid flow case");

                // let address = (digits.1 << 8) | digits.2 << 4 | digits.3;
                let address = value & 0x0FFF;

                OpCode::Flow(flow_case, address)
            }
            (8, register_x, register_y, constant) => {
                let args = (
                    u8::try_from(register_x).expect("Invalid register number"),
                    u8::try_from(register_y).expect("Invalid register number"),
                    u8::try_from(constant).expect("Invalid constant"),
                );
                OpCode::BitOp(args)
            }
            (0xD, x_pos, y_pos, constant) => {
                let args = (
                    u8::try_from(x_pos).expect("Invalid register number"),
                    u8::try_from(y_pos).expect("Invalid register number"),
                    u8::try_from(constant).expect("Invalid constant"),
                );
                OpCode::Display(Some(args))
            }
            _ => OpCode::Unknown,
        }
    }
}

impl Emu {
    #[must_use]
    /// Fetch the value from our game (loaded into RAM) at the memory address stored in our Program Counter.
    pub(crate) fn fetch_opcode(&mut self) -> OpCode {
        let pc = self.psuedo_registers.program_counter as usize;

        // An OpCode is 2 bytes long
        let higher_byte = u16::from(self.ram[pc]);
        let lower_byte = u16::from(self.ram[pc + 1]);
        let opcode = (higher_byte << 8) | lower_byte;

        // increment the program counter by 2
        self.psuedo_registers.program_counter += 2;

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
            OpCode::SkipEquals(args) => self.handle_skip_equals(*args),
            OpCode::SkipRegisterEquals(args) => self.handle_skip_register_equals(*args),
            OpCode::Call(_) => panic!("DEPRECATED!"), // NOTE: deprecated!
            OpCode::Display(_to_draw) => todo!(),
            OpCode::Return => self.handle_return(), // NOTE: technically a flow instruction
            OpCode::Flow(case, address) => self.handle_flow(*case, *address),
            OpCode::BitOp(args) => self.handle_bit_op(*args),
            OpCode::Unknown => unreachable!(),
        }
    }

    fn handle_bit_op(&self, (register_x, register_y, constant): (Register, Register, Case)) {
        todo!()
    }

    /// Handles the `SkipEquals` opcode.
    /// Check the case and skips based on the value of a register and a constant.
    /// # Arguments
    /// - `register`: The register to check.
    /// - `constant`: The constant to check against.
    fn handle_skip_equals(&mut self, (case, register, constant): (Case, Register, Constant)) {
        let register = self.general_registers.v[register as usize];
        match case {
            3 => {
                if register == constant {
                    self.psuedo_registers.program_counter += 2;
                }
            }
            4 => {
                if register != constant {
                    self.psuedo_registers.program_counter += 2;
                }
            }
            _ => unreachable!(),
        }
    }   

    /// Handles the `SkipRegisterEquals` opcode.
    /// Check the case and skips based on the values of two registers.
    /// # Arguments
    /// - `register_x`: The first register to check.
    /// - `register_y`: The second register to check.
    fn handle_skip_register_equals(&mut self, (case, register_x, register_y): (Case, Register, Register)) {
        let register_x = self.general_registers.v[register_x as usize];
        let register_y = self.general_registers.v[register_y as usize];
        match case {
            5 => {
                if register_x == register_y {
                    self.psuedo_registers.program_counter += 2;
                }
            }
            9 => {
                if register_x != register_y {
                    self.psuedo_registers.program_counter += 2;
                }
            }
            _ => unreachable!(),
        }
    }



    /// Handle a return instruction from a subroutine.
    ///
    /// The interpreter sets the program counter to the address at the top of the stack, then
    /// subtracts 1 from the stack pointer.
    fn handle_return(&mut self) {
        let return_address = self.pop_stack();
        self.set_program_counter(return_address);
    }

    /// Handle a flow instruction.
    ///
    /// # Arguments
    /// - `case`: The case to switch on.
    /// - `address`: The address where the flow instruction is acting upon.
    ///
    /// # Cases
    /// - 1: Jump (GOTO) to the address given.
    /// - 2: Call subroutine at the address given.
    /// - B or 11: Jumps to the address nnn plus V0.
    fn handle_flow(&mut self, case: Case, address: Address) {
        match case {
            //  The interpreter sets the program counter to nnn.
            1 => self.set_program_counter(address),
            //  The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.
            2 => {
                self.push_stack(self.program_counter());
                self.set_program_counter(address); // what now? KINDA confused
            }
            11 => {
                let v0 = u16::from(self.get_register_val(0));
                self.set_program_counter(address + v0);
            }
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Emu {
        let mut emu = Emu::new();
        emu.psuedo_registers.program_counter = 0; // just so we start with the same state
        emu
    }

    #[test]
    fn test_opcode_nop() {
        let mut emu = setup();
        emu.ram[0] = 0x00;
        emu.ram[1] = 0x00;

        let opcode = emu.fetch_opcode();
        assert_eq!(opcode, OpCode::Nop);
    }

    #[test]
    #[should_panic = "DEPRECATED!"]
    fn test_opcode_call() {
        let mut emu = setup();

        emu.ram[0] = 0x02;
        emu.ram[1] = 0x34;

        let opcode = emu.fetch_opcode();

        assert_eq!(opcode, OpCode::Call(0x234));

        emu.execute_opcode(&opcode);
    }

    #[test]
    #[should_panic = "not yet implemented"]
    fn test_opcode_display() {
        let mut emu = setup();

        emu.ram[0] = 0xD0;
        emu.ram[1] = 0x00;

        let opcode = emu.fetch_opcode();

        assert_eq!(opcode, OpCode::Display(Some((0, 0, 0))));

        emu.execute_opcode(&opcode);
    }

    #[test]
    fn test_opcode_return() {
        let mut emu = setup();

        emu.push_stack(0x200);

        emu.ram[0] = 0x00;
        emu.ram[1] = 0xEE;

        let opcode = emu.fetch_opcode();

        assert_eq!(opcode, OpCode::Return);

        emu.execute_opcode(&opcode);

        assert_eq!(emu.psuedo_registers.program_counter, 0x200);
    }

    #[test]
    fn test_opcode_flow_jump() {
        let mut emu = setup();

        emu.ram[0] = 0x12;
        emu.ram[1] = 0x34;

        let opcode = emu.fetch_opcode();

        assert_eq!(opcode, OpCode::Flow(1, 0x234));

        emu.execute_opcode(&opcode);

        assert_eq!(emu.psuedo_registers.program_counter, 0x234);
    }

    #[test]
    fn test_opcode_flow_call() {
        let mut emu = setup();

        emu.ram[0] = 0x23;
        emu.ram[1] = 0x45;

        let opcode = emu.fetch_opcode();

        assert_eq!(opcode, OpCode::Flow(2, 0x345));

        emu.execute_opcode(&opcode);

        assert_eq!(emu.psuedo_registers.program_counter, 0x345);
        let sp = emu.stack_pointer();
        assert_eq!(sp, 1);
        assert_eq!(emu.stack[sp as usize], 0);
    }

    #[test]
    fn test_opcode_flow_jump_v0() {
        let mut emu = setup();

        emu.set_register_val(0, 0x12);

        emu.ram[0] = 0xB3;
        emu.ram[1] = 0x45;

        let opcode = emu.fetch_opcode();

        assert_eq!(opcode, OpCode::Flow(11, 0x345));

        emu.execute_opcode(&opcode);

        assert_eq!(emu.psuedo_registers.program_counter, 0x357);
    }
}
