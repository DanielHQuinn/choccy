#![allow(dead_code)]
//! Here lies the Emulator module, which contains the CPU struct and its methods to emulate the CHIP-8 CPU.
#[allow(clippy::module_inception)]
/// The emulator module contains the [`Emu`] struct and its methods.
pub mod emulator;
/// The opcode module contains the [`OpCode`] struct and its methods.
pub mod opcode;
/// The registers module contains [`GeneralRegisters`], [`PsuedoRegisters`], and [`SpecialRegisters`] structs and their methods.
pub mod registers;
/// The sound module contains the [`Sound`] struct and its methods.
pub mod sound;

/// width of the CHIP-8 screen
pub const SCREEN_WIDTH: usize = 64;
/// height of the CHIP-8 screen
pub const SCREEN_HEIGHT: usize = 32;

/// The CHIP-8 CPU has 4096 bytes of memory.
pub const RAM_SIZE: usize = 4096;

/// The CHIP-8 CPU has 16 levels of stack.
pub const STACK_SIZE: usize = 16;

/// Number of keys
pub const NUM_KEYS: usize = 16;

/// Size of Character Set
pub const SPRITE_SET_SIZE: usize = 80;

/// `SPRITE_SET` to draw characters 0-F
/// An image of each character is stored in memory at the locations 0x000-0x01F.
/// Each character is 5 bytes long so 8 x 5
pub const SPRITE_SET: [u8; SPRITE_SET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

#[cfg(test)]
mod opcode_tests;
