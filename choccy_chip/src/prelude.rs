//! The prelude module contains common imports and re-exports them for
//! convenience. This module is meant to be glob imported in the root of the crate.
//!
//! Usage:
//! ```
//! use choccy_chip::prelude::*;
//! ```
pub use crate::emulator::emulator::Emu;
pub use crate::emulator::opcode::OpCode;
pub use crate::emulator::{SCREEN_HEIGHT, SCREEN_WIDTH, SPRITE_SET_SIZE, SPRITE_SET};
