//! This module contains the `OpCode` enum which represents the different opcodes that the CHIP-8 emulator can execute.
//! Additionally, it contains the `OpCodeError` enum which represents the different errors that can occur when executing an opcode.
//! Finally, it implments methods for the `OpCode` enum.
use std::env::Args;

use super::Emu;
type Address = u16; // an address
type Case = u8; // represents a number that can be used in a switch statement
type Constant = u8; // a 8 bit constant
type RegisterID = u8; // a 4 bit register number

/// The `OpCode` enum represents the different opcodes that the CHIP-8 emulator can execute.
/// There are 35 different opcodes in total.
/// We decided to group them by their 'type'
#[derive(Debug, PartialEq)]
pub(crate) enum OpCode {
    Nop,
    SkipEquals((Case, RegisterID, Constant)),
    SkipRegEquals((Case, RegisterID, RegisterID)),
    Constant((Case, RegisterID, Constant)),
    Display(Option<(Constant, Constant, Constant)>), // Whether to clear or draw
    Return,                                          // NOTE: technically a flow control instruction
    Call(Address),                                   // NOTE: This is deprecated
    Flow(Case, Address),
    BitOp((RegisterID, RegisterID, Case)),
    IOp(Address), // NOTE: technically a memory control instruction
    MemoryOp((RegisterID, Case)),
    Unknown,
    KeyOpSkip(Case, RegisterID),
    KeyOpWait(RegisterID),
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
            (0, 0, 0xE, 0) => OpCode::Display(None),
            (0, 0, 0xE, 0xE) => OpCode::Return, // technically a flow control instruction
            (0, _, _, _) => OpCode::Call(value & 0x0FFF), // Get rid of the first digit
            (1 | 2 | 0xB, _, _, _) => {
                let flow_case = u8::try_from(digits.0).expect("Invalid flow case");
                let address = value & 0x0FFF;
                OpCode::Flow(flow_case, address)
            }
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
                OpCode::SkipRegEquals(args)
            }
            (6 | 7, register_x, _, _) => {
                let args = (
                    u8::try_from(digits.0).expect("Invalid case"),
                    u8::try_from(register_x).expect("Invalid register number"),
                    u8::try_from(value & 0x00FF).expect("Invalid register number"),
                );
                OpCode::Constant(args)
            }
            (8, register_x, register_y, constant) => {
                let args = (
                    u8::try_from(register_x).expect("Invalid register number"),
                    u8::try_from(register_y).expect("Invalid register number"),
                    u8::try_from(constant).expect("Invalid constant"),
                );
                OpCode::BitOp(args)
            }
            (0xA, _, _, _) => OpCode::IOp(value & 0x0FFF), // NOTE: technically a memory control instruction
            (0xD, register_x, register_y, constant) => {
                let args = (
                    u8::try_from(register_x).expect("Invalid register number"),
                    u8::try_from(register_y).expect("Invalid register number"),
                    u8::try_from(constant).expect("Invalid constant"),
                );
                OpCode::Display(Some(args))
            }
            (0xE, reg_id, 9 | 0xA, 0xE | 1) => {
                let reg_id = u8::try_from(reg_id).expect("Invalid register number");

                let case = match (digits.2, digits.3) {
                    (9, 0xE) => 0x9E,    // Ex9E
                    (0xA, 1) => 0xA1,    // ExA1
                    _ => unreachable!(),
                };

                OpCode::KeyOpSkip(case, reg_id)
            }
            
            (0xF, reg_id, 0, 0xA) => {
                let reg_id = u8::try_from(reg_id).expect("Invalid register number");
                OpCode::KeyOpWait(reg_id)
            },

            (0xF, reg_id, 1 | 2 | 5 | 6, 0xE | 9 | 5) => {
                let reg_id = u8::try_from(reg_id).expect("Invalid register number");

                let case = match (digits.2, digits.3) {
                    (1, 0xE) => 0x1E,    // Fx1E
                    (2, 9) => 29,        // Fx29
                    (5, 5) => 55,        // Fx55
                    (6, 5) => 65,        // Fx65
                    _ => unreachable!(), // TODO: handle this error
                };

                OpCode::MemoryOp((reg_id, case))
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
            OpCode::SkipEquals(args) | OpCode::SkipRegEquals(args) => self.handle_cond(*args),
            OpCode::Constant(args) => self.handle_const(*args),
            OpCode::Call(_) => panic!("DEPRECATED!"), // TODO: deprecated! error handle later on
            OpCode::Display(_to_draw) => todo!(),
            OpCode::Return => self.handle_return(), // NOTE: technically a flow instruction
            OpCode::Flow(case, address) => self.handle_flow(*case, *address),
            OpCode::BitOp(args) => self.handle_bit_op(*args),
            OpCode::IOp(address) => self.handle_io(*address), // NOTE: technically a memory control instruction
            OpCode::MemoryOp(args) => self.handle_memory_op(*args),
            OpCode::Unknown => unreachable!(),
            OpCode::KeyOpSkip(case, reg_id) => self.handle_keyop_skip(*case, *reg_id),
            OpCode::KeyOpWait(reg_id) => todo!(),
        }
    }

    /// Handles the `IOp` opcode, which Sets I to the address.
    ///
    /// # Arguments
    /// - `address`: The address to act upon.
    fn handle_io(&mut self, address: Address) {
        self.i_register = address;
    }

    /// Handles the `MemoryOp` opcode.
    ///
    /// # Arguments
    /// - `register`: The register to act upon.
    /// - `case`: The case to switch on.
    ///
    /// # Cases
    /// - 0x1E: Adds the value of register X to I. VF is not affected.
    /// - 29: Sets I to the location of the sprite for the character in register X. Characters 0-F
    ///     (in hexadecimal) are represented by a 4x5 font.
    /// - 55: Stores V0 to VX in memory starting at address I. With an offset increment of 1
    /// - 65: Fills V0 to VX with values from memory starting at address I. With an offset increment of 1
    fn handle_memory_op(&mut self, (register_id, case): (RegisterID, Case)) {
        match case {
            0x1E => {
                let register_val = u16::from(self.get_register_val(register_id));
                self.i_register = self.i_register.wrapping_add(register_val);
            }
            29 => {
                let register_val = u16::from(self.get_register_val(register_id));
                self.i_register = register_val * 5; // each character sprite is 5 bytes long
            }
            55 => {
                let i_reg = self.i_register as usize;
                for curr_reg in 0..=register_id {
                    self.ram[i_reg + curr_reg as usize] = self.get_register_val(curr_reg);
                }
            }
            65 => {
                let i_reg = self.i_register as usize;
                for curr_reg in 0..=register_id {
                    let val = self.ram[i_reg + curr_reg as usize];
                    self.set_register_val(curr_reg, val);
                }
            }
            _ => unreachable!(), // TODO: handle this error
        }
    }

    /// Handles the `Assig`,`BitOp`,`Math` opcodes.
    /// Check the case and skips based on the value of a register and a constant.
    /// # Arguments
    /// - `register_x`: A register operate on.
    /// - `register_y`: Another register operate on.
    /// - `case`: The case to switch on.
    /// # Cases
    /// - TODO
    fn handle_bit_op(&mut self, (register_x, register_y, case): (RegisterID, RegisterID, Case)) {
        let register_x_val = self.get_register_val(register_x);
        let register_y_val = self.get_register_val(register_y);

        match case {
            0x0 => {
                // Vx = Vy
                self.set_register_val(register_x, register_y_val);
            }
            0x1 => {
                // Vx |= Vy
                self.set_register_val(register_x, register_x_val | register_y_val);
            }
            0x2 => {
                // Vx &= Vy
                self.set_register_val(register_x, register_x_val & register_y_val);
            }
            0x3 => {
                // Vx ^= Vy
                self.set_register_val(register_x, register_x_val ^ register_y_val);
            }
            0x4 => {
                // Vx += Vy
                // set Vf to 1 when overflow, 0 otherwise
                let (result, overflow) = register_x_val.overflowing_add(register_y_val);
                self.set_register_val(register_x, result);
                self.set_register_val(0xF, u8::from(overflow));
            }
            0x5 => {
                // Vx -= Vy
                // set Vf to 0 when underflow, 1 otherwise
                let (result, overflow) = register_x_val.overflowing_sub(register_y_val);
                self.set_register_val(register_x, result);
                self.set_register_val(0xF, u8::from(!overflow));
            }
            0x6 => {
                // Shift VX right by 1 and stores lsb of VX before shift into VF
                self.set_register_val(0xF, register_x_val & 0x1);
                self.set_register_val(register_x, register_x_val >> 1);
            }
            0x7 => {
                // Vy -= Vx
                // set Vf to 0 when underflow, 1 otherwise
                let (result, overflow) = register_y_val.overflowing_sub(register_x_val);
                self.set_register_val(register_x, result);
                self.set_register_val(0xF, u8::from(!overflow));
            }
            0xE => {
                // Shift VX left by 1 and stores msb of VX before shift into VF
                self.set_register_val(0xF, (register_x_val >> 7) & 0x1);
                self.set_register_val(register_x, register_x_val << 1);
            }
            _ => unreachable!(),
        }
    }

    /// Handles the `Cond` opcode.
    /// Check the case and skips based on the value of a register and a constant.
    /// # Arguments
    /// - `register`: The register to check.
    /// - `constant` | `register`: The constant or register to check against.
    fn handle_cond(&mut self, (case, register, constant): (Case, RegisterID, u8)) {
        let register_val = self.get_register_val(register);
        let condition_met = match case {
            3 => register_val == constant,
            4 => register_val != constant,
            5 => register_val == self.get_register_val(constant),
            9 => register_val != self.get_register_val(constant),
            _ => unreachable!(),
        };
        if condition_met {
            self.psuedo_registers.program_counter += 2;
        }
    }

    /// Handles the `Const` opcode.
    /// Check sets the value of a register to a constant or increments the value by a constant.
    /// # Arguments
    /// - `register`: The register to check.
    /// - `constant` The constant set or increment the register's value by
    fn handle_const(&mut self, (case, register, constant): (Case, RegisterID, u8)) {
        match case {
            6 => {
                self.set_register_val(register, constant);
            }
            7 => {
                let register_val: u8 = self.get_register_val(register);
                self.set_register_val(register, constant + register_val);
            }
            _ => unreachable!(),
        };
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
            _ => unreachable!(),
        }
    }

    /// Handle a keyop skip operation.
    /// Skips the next instruction if the key stored in register X is pressed (or not pressed).
    /// # Arguments
    /// - `case`: The case to switch on.
    /// - `reg_id`: The register to check.
    /// # Cases
    /// - 0x9E: Skips the next instruction if the key stored in register X is pressed.
    /// - 0xA1: Skips the next instruction if the key stored in register X is not pressed.
    fn handle_keyop_skip(&mut self, case: u8, reg_id: u8) {
        let key = self.get_register_val(reg_id);
        let key_state = self.keys[key as usize];
        let skip = match case {
            0x9E => key_state,
            0xA1 => !key_state,
            _ => unreachable!(),
        };
        if skip {
            self.psuedo_registers.program_counter += 2;
        }
    }

    fn handle_keyop_wait(&mut self, reg_id: u8) {
        todo!()
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

    #[test]
    fn test_opcode_skip_equals() {
        let mut emu = setup();

        emu.set_register_val(0, 0x12);

        emu.ram[0] = 0x30;
        emu.ram[1] = 0x12;

        let opcode = emu.fetch_opcode();

        assert_eq!(opcode, OpCode::SkipEquals((3, 0, 0x12)));

        emu.execute_opcode(&opcode);

        assert_eq!(emu.psuedo_registers.program_counter, 4);
    }

    #[test]
    fn test_opcode_skip_not_equals() {
        let mut emu = setup();

        emu.set_register_val(0, 0x12);

        emu.ram[0] = 0x40;
        emu.ram[1] = 0x34;

        let opcode = emu.fetch_opcode();

        assert_eq!(opcode, OpCode::SkipEquals((4, 0, 0x34)));

        emu.execute_opcode(&opcode);

        assert_eq!(emu.psuedo_registers.program_counter, 4);
    }
    #[test]
    fn test_opcode_skip_register_equals() {
        let mut emu = setup();

        emu.set_register_val(0, 0x12);
        emu.set_register_val(1, 0x12);

        emu.ram[0] = 0x50;
        emu.ram[1] = 0x10;

        let opcode = emu.fetch_opcode();

        assert_eq!(opcode, OpCode::SkipRegEquals((5, 0, 1)));

        emu.execute_opcode(&opcode);

        assert_eq!(emu.psuedo_registers.program_counter, 4);
    }

    #[test]
    fn test_opcode_skip_register_not_equals() {
        let mut emu = setup();

        emu.set_register_val(0, 0x12);
        emu.set_register_val(1, 0x34);

        emu.ram[0] = 0x90;
        emu.ram[1] = 0x10;

        let opcode = emu.fetch_opcode();

        assert_eq!(opcode, OpCode::SkipRegEquals((9, 0, 1)));

        emu.execute_opcode(&opcode);

        assert_eq!(emu.psuedo_registers.program_counter, 4);
    }

    #[test]
    fn test_opcode_set_const() {
        let mut emu = setup();
        emu.set_register_val(0, 0x12);
        emu.ram[0] = 0x60;
        emu.ram[1] = 0x34;
        let opcode = emu.fetch_opcode();
        assert_eq!(opcode, OpCode::Constant((6, 0, 0x34)));
        emu.execute_opcode(&opcode);
        assert_eq!(emu.get_register_val(0), 0x34);
    }

    #[test]
    fn test_opcode_add_const() {
        let mut emu = setup();
        emu.set_register_val(0, 0x12);
        emu.ram[0] = 0x70;
        emu.ram[1] = 0x34;
        let opcode = emu.fetch_opcode();
        assert_eq!(opcode, OpCode::Constant((7, 0, 0x34)));
        emu.execute_opcode(&opcode);
        assert_eq!(emu.get_register_val(0), 0x46);
    }

    //TODO: FIX BITOP TESTS
    #[test]
    fn test_opcode_bit_op0() {
        let mut emu = setup();
        emu.set_register_val(0, 0x12);
        emu.set_register_val(1, 0x34);
        emu.ram[0] = 0x80;
        emu.ram[1] = 0x10;
        let opcode = emu.fetch_opcode();
        assert_eq!(opcode, OpCode::BitOp((0, 1, 0)));
        emu.execute_opcode(&opcode);
        assert_eq!(emu.get_register_val(0), 0x34);
    }

    #[test]
    fn test_opcode_bit_op1() {
        let mut emu = setup();
        emu.set_register_val(0, 0x12);
        emu.set_register_val(1, 0x36);
        emu.ram[0] = 0x80;
        emu.ram[1] = 0x11;
        let opcode = emu.fetch_opcode();
        assert_eq!(opcode, OpCode::BitOp((0, 1, 1)));
        emu.execute_opcode(&opcode);
        assert_eq!(emu.get_register_val(0), 0x36);
    }

    #[test]
    fn test_opcode_bit_op2() {
        let mut emu = setup();
        emu.set_register_val(0, 0x12);
        emu.set_register_val(1, 0x12);
        emu.ram[0] = 0x80;
        emu.ram[1] = 0x12;
        let opcode = emu.fetch_opcode();
        assert_eq!(opcode, OpCode::BitOp((0, 1, 2)));
        emu.execute_opcode(&opcode);
        assert_eq!(emu.get_register_val(0), 0x12);
    }

    #[test]
    fn test_opcode_bit_op3() {
        let mut emu = setup();
        emu.set_register_val(0, 0x12);
        emu.set_register_val(1, 0x34);
        emu.ram[0] = 0x80;
        emu.ram[1] = 0x13;
        let opcode = emu.fetch_opcode();
        assert_eq!(opcode, OpCode::BitOp((0, 1, 3)));
        emu.execute_opcode(&opcode);
        assert_eq!(emu.get_register_val(0), 0x26);
    }

    #[test]
    fn test_opcode_bit_op4() {
        let mut emu = setup();
        emu.set_register_val(0, 0x12);
        emu.set_register_val(1, 0x34);
        emu.ram[0] = 0x80;
        emu.ram[1] = 0x14;
        let opcode = emu.fetch_opcode();
        assert_eq!(opcode, OpCode::BitOp((0, 1, 4)));
        emu.execute_opcode(&opcode);
        assert_eq!(emu.get_register_val(0), 0x46);
    }

    #[test]
    fn test_opcode_bit_op5() {
        let mut emu = setup();
        emu.set_register_val(0, 0x20);
        emu.set_register_val(1, 0x10);
        emu.ram[0] = 0x80;
        emu.ram[1] = 0x15;
        let opcode = emu.fetch_opcode();
        assert_eq!(opcode, OpCode::BitOp((0, 1, 5)));
        emu.execute_opcode(&opcode);
        assert_eq!(emu.get_register_val(0), 0x10);
    }

    #[test]
    fn test_opcode_bit_op6() {
        let mut emu = setup();
        emu.set_register_val(0, 0x12);
        emu.set_register_val(1, 0x00);
        emu.ram[0] = 0x80;
        emu.ram[1] = 0x16;
        let opcode = emu.fetch_opcode();
        assert_eq!(opcode, OpCode::BitOp((0, 1, 6)));
        emu.execute_opcode(&opcode);
        assert_eq!(emu.get_register_val(0), 0x09);
    }

    #[test]
    fn test_opcode_bit_op7() {
        let mut emu = setup();
        emu.set_register_val(0, 0x12);
        emu.set_register_val(1, 0x34);
        emu.ram[0] = 0x80;
        emu.ram[1] = 0x17;
        let opcode = emu.fetch_opcode();
        assert_eq!(opcode, OpCode::BitOp((0, 1, 7)));
        emu.execute_opcode(&opcode);
        assert_eq!(emu.get_register_val(0), 0x22);
    }

    #[test]
    fn test_opcode_bit_ope() {
        let mut emu = setup();
        emu.set_register_val(0, 0x12);
        emu.set_register_val(1, 0x00);
        emu.ram[0] = 0x80;
        emu.ram[1] = 0x1E;
        let opcode = emu.fetch_opcode();
        assert_eq!(opcode, OpCode::BitOp((0, 1, 0xE)));
        emu.execute_opcode(&opcode);
        assert_eq!(emu.get_register_val(0), 0x24);
    }

    #[test]
    fn test_opcode_iop() {
        let mut emu = setup();

        emu.ram[0] = 0xA2;
        emu.ram[1] = 0x34;

        let opcode = emu.fetch_opcode();

        assert_eq!(opcode, OpCode::IOp(0x234));

        emu.execute_opcode(&opcode);

        assert_eq!(emu.i_register, 0x234);
    }

    #[test]
    fn test_opcode_memory_op1e() {
        let mut emu = setup();

        emu.set_register_val(0, 0x12);
        emu.i_register = 0x34;

        emu.ram[0] = 0xF0;
        emu.ram[1] = 0x1E;

        let opcode = emu.fetch_opcode();

        assert_eq!(opcode, OpCode::MemoryOp((0, 0x1E)));

        emu.execute_opcode(&opcode);

        assert_eq!(emu.i_register, 0x46);

        emu.set_register_val(0, 0x1);

        emu.i_register = 0xFFFF; // this can be upto 0xFFFF

        emu.execute_opcode(&opcode);

        assert_eq!(emu.i_register, 0x0);
    }

    #[test]
    fn test_opcode_memory_op29() {
        let mut emu = setup();

        emu.set_register_val(0, 0x1);

        emu.ram[0] = 0xF0;
        emu.ram[1] = 0x29;

        let opcode = emu.fetch_opcode();

        assert_eq!(opcode, OpCode::MemoryOp((0, 29))); // here 29 is just 29 and not 0x29

        emu.execute_opcode(&opcode);

        assert_eq!(emu.i_register, 0x5);
    }

    #[test]
    fn test_opcode_memory_op55() {
        let mut emu = setup();

        emu.set_register_val(0, 0x1);
        emu.set_register_val(1, 0x2);
        emu.set_register_val(2, 0x3);
        emu.set_register_val(3, 0x4);

        emu.i_register = 0x34;

        emu.ram[0] = 0xF3;
        emu.ram[1] = 0x55;

        let opcode = emu.fetch_opcode();

        assert_eq!(opcode, OpCode::MemoryOp((3, 55))); // here 55 is just 55 and not 0x55

        emu.execute_opcode(&opcode);

        // now, the following are in memory
        assert_eq!(emu.ram[0x34], 0x1);
        assert_eq!(emu.ram[0x35], 0x2);
        assert_eq!(emu.ram[0x36], 0x3);
        assert_eq!(emu.ram[0x37], 0x4);
    }

    #[test]
    fn test_opcode_memory_op65() {
        let mut emu = setup();

        emu.i_register = 0x34;

        emu.ram[0] = 0xF3;
        emu.ram[1] = 0x65;

        emu.ram[0x34] = 0x1;
        emu.ram[0x35] = 0x2;
        emu.ram[0x36] = 0x3;
        emu.ram[0x37] = 0x4;

        let opcode = emu.fetch_opcode();

        assert_eq!(opcode, OpCode::MemoryOp((3, 65))); // here 65 is just 65 and not 0x65

        emu.execute_opcode(&opcode);

        assert_eq!(emu.get_register_val(0), 0x1);
        assert_eq!(emu.get_register_val(1), 0x2);
        assert_eq!(emu.get_register_val(2), 0x3);
        assert_eq!(emu.get_register_val(3), 0x4);
    }

    #[test]
    fn test_opcode_keyop_skip_equals() {
        let mut emu = setup();

        emu.set_register_val(0, 0x1);
        emu.keys[0x1] = true;

        emu.ram[0] = 0xE0;
        emu.ram[1] = 0x9E;

        let opcode = emu.fetch_opcode();

        assert_eq!(opcode, OpCode::KeyOpSkip(0x9E, 0));

        emu.execute_opcode(&opcode);

        assert_eq!(emu.psuedo_registers.program_counter, 4);
    }

    #[test]
    fn test_opcode_keyop_skip_not_equals() {
        let mut emu = setup();

        emu.set_register_val(0, 0x1);
        emu.keys[0x1] = false;

        emu.ram[0] = 0xE0;
        emu.ram[1] = 0xA1;

        let opcode = emu.fetch_opcode();

        assert_eq!(opcode, OpCode::KeyOpSkip(0xA1, 0));

        emu.execute_opcode(&opcode);

        assert_eq!(emu.psuedo_registers.program_counter, 4);
    }
}


