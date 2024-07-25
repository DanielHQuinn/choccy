//! This module contains the `OpCode` enum which represents the different opcodes that the CHIP-8 emulator can execute.
//! Additionally, it contains the `OpCodeError` enum which represents the different errors that can occur when executing an opcode.
//! Finally, it implments methods for the `OpCode` enum.
use core::fmt;
use std::fmt::Display;

use super::emulator::Emu;
type Address = u16; // an address
type Case = u8; // represents a number that can be used in a switch statement
type Constant = u8; // a 8 bit constant
type RegisterID = u8; // a 4 bit register number

/// The `OpCodeError` enum represents the different errors that can occur when executing an opcode.
#[derive(Debug, PartialEq)]
pub enum OpCodeError {
    /// The opcode is invalid.
    InvalidOpCode,
    /// The opcode is deprecated.
    DeprecatedOpCode,
    /// Some other error occurred.
    UnknownOpCode,
}

impl Display for OpCodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpCodeError::InvalidOpCode => write!(f, "Invalid opcode"),
            OpCodeError::DeprecatedOpCode => write!(f, "Deprecated opcode"),
            OpCodeError::UnknownOpCode => write!(f, "Unknown opcode"),
        }
    }
}

impl std::error::Error for OpCodeError {}

/// The `OpCode` enum represents the different opcodes that the CHIP-8 emulator can execute.
/// There are 35 different opcodes in total.
/// We decided to group them by their 'type'
#[derive(Debug, PartialEq)]
pub enum OpCode {
    /// The `Nop` opcode does nothing.
    Nop,
    /// The `Call` opcode calls a subroutine at the given address, but it is deprecated.
    Call(Address), // TODO: This is deprecated
    /// The `Display` opcode is used to draw sprites on the screen or clear the screen.
    Display(Option<(Constant, Constant, Constant)>),
    /// A flow control instruction that returns from a subroutine.
    Return, // NOTE: technically a flow control instruction
    /// A flow control instruction
    Flow(Case, Address),
    /// A conditional instruction that skips the next instruction if the value of a register is equal to a constant.
    SkipEquals((Case, RegisterID, Constant)),
    /// A conditional instruction that skips the next instruction if the value of two registers are equal.
    SkipRegEquals((Case, RegisterID, RegisterID)),
    /// An instruction that sets a register to a constant or increments the value of a register by a constant.
    Constant((Case, RegisterID, Constant)),
    /// An instruction that performs a bitwise operation on two registers.
    BitOp((RegisterID, RegisterID, Case)),
    /// An instruction that sets the I register to the given address. A memory control instruction.
    IOp(Address), // NOTE: technically a memory control instruction
    /// An instruction that performs a memory operation.
    MemoryOp((RegisterID, Case)),
    /// An instruction that fills a register with a random number.
    RandomOp((RegisterID, Constant)),
    /// An instruction skips the next instruction if a key is pressed or not pressed.
    KeyOpSkip(Case, RegisterID),
    /// An instruction that hangs until a key is pressed
    KeyOpWait(RegisterID),
    /// An instruction that deals with delay and sound timers.
    Timer((RegisterID, Case)),
    /// An instruction that stores the binary-coded decimal representation of a register in memory.
    Bcd(RegisterID),
    /// An unknown opcode.
    Unknown,
}

#[allow(clippy::too_many_lines)]
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
            (0xC, register_id, _, _) => {
                let args = (
                    u8::try_from(register_id).expect("Invalid register number"),
                    u8::try_from(value & 0x00FF).expect("Invalid constant"),
                );
                OpCode::RandomOp(args)
            }
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
                    (9, 0xE) => 0x9E, // Ex9E
                    (0xA, 1) => 0xA1, // ExA1
                    _ => return OpCode::Unknown,
                };

                OpCode::KeyOpSkip(case, reg_id)
            }
            (0xF, reg_id, 0, 0xA) => {
                let reg_id = u8::try_from(reg_id).expect("Invalid register number");
                OpCode::KeyOpWait(reg_id)
            }
            (0xF, reg_id, 1, 5 | 8) | (0xF, reg_id, 0, 7) => {
                let args = (
                    u8::try_from(reg_id).expect("Invalid register number"),
                    u8::try_from(digits.3).expect("Invalid case"),
                );
                OpCode::Timer(args)
            }
            (0xF, reg_id, 1 | 2 | 5 | 6, 0xE | 9 | 5) => {
                let reg_id = u8::try_from(reg_id).expect("Invalid register number");

                let case = match (digits.2, digits.3) {
                    (1, 0xE) => 0x1E, // Fx1E
                    (2, 9) => 29,     // Fx29
                    (5, 5) => 55,     // Fx55
                    (6, 5) => 65,     // Fx65
                    _ => return OpCode::Unknown,
                };

                OpCode::MemoryOp((reg_id, case))
            }
            (0xF, reg_id, 3, 3) => {
                let reg_id = u8::try_from(reg_id).expect("Invalid register number");
                OpCode::Bcd(reg_id)
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
    pub(crate) fn execute_opcode(&mut self, opcode: &OpCode) -> Result<(), OpCodeError> {
        match opcode {
            OpCode::Nop => Err(OpCodeError::InvalidOpCode), // TODO: should we sanitize addresses?
            OpCode::SkipEquals(args) | OpCode::SkipRegEquals(args) => self.handle_cond(*args),
            OpCode::Constant(args) => self.handle_const(*args),
            OpCode::Call(_) => Err(OpCodeError::DeprecatedOpCode),
            OpCode::Display(to_draw) => {
                self.handle_display(*to_draw);
                Ok(())
            }
            OpCode::Return => {
                self.handle_return();
                Ok(())
            } // NOTE: technically a flow instruction
            OpCode::Flow(case, address) => self.handle_flow(*case, *address),
            OpCode::BitOp(args) => self.handle_bit_op(*args),
            OpCode::IOp(address) => {
                self.handle_io(*address);
                Ok(())
            } // NOTE: technically a memory control instruction
            OpCode::MemoryOp(args) => self.handle_memory_op(*args),
            OpCode::KeyOpSkip(case, reg_id) => self.handle_keyop_skip(*case, *reg_id),
            OpCode::KeyOpWait(reg_id) => {
                self.handle_keyop_wait(*reg_id);
                Ok(())
            }
            OpCode::Timer(args) => self.handle_timer(*args),
            OpCode::RandomOp(args) => {
                self.handle_random_op(*args);
                Ok(())
            }
            OpCode::Bcd(reg_id) => {
                self.handle_bcd(*reg_id);
                Ok(())
            }
            OpCode::Unknown => Err(OpCodeError::UnknownOpCode),
        }
    }

    #[allow(clippy::doc_markdown)]
    /// Handles the `Display` type opcode.
    ///
    /// # Arguments
    /// - `to_draw`: An optional tuple containing the x, y, and height of the sprite to draw.
    ///    depending on this, we will either clear or draw
    ///
    /// Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
    /// The interpreter reads n bytes from memory, starting at the address stored in I. These bytes are
    /// then displayed as sprites on screen at coordinates (Vx, Vy). Sprites are "XORed" onto the
    /// existing screen. If this causes any pixels to be erased, VF is set to 1, otherwise it is set
    /// to 0. If the sprite is positioned so part of it is outside the coordinates of the display,
    /// it wraps around to the opposite side of the screen. See instruction 8xy3 for more
    /// information on XOR, and section 2.4, Display, for more information on the Chip-8 screen and
    /// sprites.
    fn handle_display(&mut self, to_draw: Option<(Constant, Constant, Constant)>) {
        match to_draw {
            Some((reg_x, reg_y, height)) => {
                let i_reg = self.i_register as usize;
                let x_val = u16::from(self.get_register_val(reg_x));
                let y_val = u16::from(self.get_register_val(reg_y));
                let (screen_width, screen_height) = self.screen_size();

                let mut collision = false;
                for row in 0..height.into() {
                    let sprite = self.ram[i_reg + row as usize];
                    for col in 0..8 {
                        // use a mask to fetch current's sprite bit
                        // only flip if a 1
                        if (sprite & (0x80 >> col)) != 0 {
                            let x = (x_val + col) as usize % screen_width;
                            let y = (y_val + row) as usize % screen_height;

                            let index = y * screen_width + x;

                            collision |= self.screen[index];
                            self.screen[index] ^= true;
                        }
                    }
                }
                self.set_register_val(0xF, u8::from(collision));
            }
            None => self.screen.fill(false),
        };
    }

    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    /// Handles the `BCD` opcode.
    ///
    /// # Arguments
    /// - `register_id`: The register to act upon.
    ///
    /// Stores the binary-coded decimal representation of VX, with the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.[22]
    fn handle_bcd(&mut self, register_id: RegisterID) {
        let register_val = f32::from(self.get_register_val(register_id));
        let (hundreds, tens, ones) = (
            (register_val / 100.0).floor() as u8,
            ((register_val / 10.0) % 10.0).floor() as u8,
            (register_val % 10.0) as u8,
        );

        let i_reg = self.i_register as usize;

        self.ram[i_reg] = hundreds;
        self.ram[i_reg + 1] = tens;
        self.ram[i_reg + 2] = ones;
    }

    /// Handles the `RandomOp` opcode.
    /// Sets register X to the result of a bitwise AND operation on a random number (0 to 255) and a constant.
    /// # Arguments
    /// - `register_id`: The register to act upon.
    /// - `constant`: The constant to act upon.
    fn handle_random_op(&mut self, (register_id, constant): (RegisterID, Constant)) {
        let random_number: u8 = rand::random();
        let result = random_number & constant;
        self.set_register_val(register_id, result);
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
    fn handle_memory_op(
        &mut self,
        (register_id, case): (RegisterID, Case),
    ) -> Result<(), OpCodeError> {
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
            _ => return Err(OpCodeError::InvalidOpCode),
        };
        Ok(())
    }

    #[allow(clippy::similar_names)]
    /// Handles the `Assig`,`BitOp`,`Math` opcodes.
    /// Check the case and skips based on the value of a register and a constant.
    /// # Arguments
    /// - `register_x`: A register operate on.
    /// - `register_y`: Another register operate on.
    /// - `case`: The case to switch on.
    /// # Cases
    /// - TODO
    fn handle_bit_op(
        &mut self,
        (register_x, register_y, case): (RegisterID, RegisterID, Case),
    ) -> Result<(), OpCodeError> {
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
            _ => return Err(OpCodeError::InvalidOpCode),
        };
        Ok(())
    }

    /// Handles the `Cond` opcode.
    /// Check the case and skips based on the value of a register and a constant.
    /// # Arguments
    /// - `register`: The register to check.
    /// - `constant` | `register`: The constant or register to check against.
    fn handle_cond(
        &mut self,
        (case, register, constant): (Case, RegisterID, u8),
    ) -> Result<(), OpCodeError> {
        let register_val = self.get_register_val(register);
        let condition_met = match case {
            3 => register_val == constant,
            4 => register_val != constant,
            5 => register_val == self.get_register_val(constant),
            9 => register_val != self.get_register_val(constant),
            _ => return Err(OpCodeError::InvalidOpCode),
        };
        if condition_met {
            self.psuedo_registers.program_counter += 2;
        };
        Ok(())
    }

    /// Handles the `Const` opcode.
    /// Check sets the value of a register to a constant or increments the value by a constant.
    /// # Arguments
    /// - `register`: The register to check.
    /// - `constant` The constant set or increment the register's value by
    fn handle_const(
        &mut self,
        (case, register, constant): (Case, RegisterID, u8),
    ) -> Result<(), OpCodeError> {
        match case {
            6 => {
                self.set_register_val(register, constant);
            }
            7 => {
                let register_val: u8 = self.get_register_val(register);
                let check = constant.wrapping_add(register_val); // TODO: make sure this is correct
                self.set_register_val(register, check);
            }
            _ => return Err(OpCodeError::InvalidOpCode),
        };
        Ok(())
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
    fn handle_flow(&mut self, case: Case, address: Address) -> Result<(), OpCodeError> {
        match case {
            //  The interpreter sets the program counter to nnn.
            1 => {
                self.set_program_counter(address);
                Ok(())
            }
            //  The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.
            2 => {
                self.push_stack(self.program_counter());
                self.set_program_counter(address); // what now? KINDA confused
                Ok(())
            }
            11 => {
                let v0 = u16::from(self.get_register_val(0));
                self.set_program_counter(address + v0);
                Ok(())
            }
            _ => Err(OpCodeError::InvalidOpCode),
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
    fn handle_keyop_skip(&mut self, case: u8, reg_id: u8) -> Result<(), OpCodeError> {
        let key = self.get_register_val(reg_id);
        let key_state = self.keys[key as usize];
        let skip = match case {
            0x9E => key_state,
            0xA1 => !key_state,
            _ => return Err(OpCodeError::InvalidOpCode),
        };
        if skip {
            self.psuedo_registers.program_counter += 2;
        }
        Ok(())
    }

    /// Handle a keyop wait operation
    /// Waits for a key press and stores the key in the given register.
    /// #Arguments
    /// - `reg_id`: The register to store the key in.
    ///
    /// # Notes
    /// - This is a blocking operation.
    /// - If multiple keys are pressed, the minimum is chosen.
    fn handle_keyop_wait(&mut self, reg_id: u8) {
        let mut pressed = false;
        for i in 0..self.keys.len() {
            if self.keys[i] {
                self.set_register_val(reg_id, u8::try_from(i).expect("Invalid key"));
                pressed = true;
                break;
            }
        }
        if !pressed {
            // Redo opcode
            self.psuedo_registers.program_counter -= 2;
        }
    }

    /// Handle opcodes related to the sound and delay timers.
    /// # Arguments
    /// - `reg_id`: The register to get or set.
    fn handle_timer(&mut self, (register_id, case): (RegisterID, Case)) -> Result<(), OpCodeError> {
        match case {
            7 => self.set_register_val(register_id, self.get_delay_timer()),
            5 => self.set_delay_timer(self.get_register_val(register_id)),
            8 => self.set_sound_timer(self.get_register_val(register_id)),
            _ => return Err(OpCodeError::InvalidOpCode),
        };
        Ok(())
    }
}
