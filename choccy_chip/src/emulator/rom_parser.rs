use crate::emulator::RAM_SIZE;

use std::path::PathBuf;

use super::emulator::Emu;

/// This struct represents a ROM parser.
#[derive(Debug)]
pub struct RomParser {
    file_path: PathBuf,
}

/// Represents a valid ROM file.
#[derive(Debug, PartialEq)]
pub struct ValidRom(Vec<u8>);

impl ValidRom {
    /// Returns the ROM data as a vector of bytes.
    ///
    /// # Returns
    ///
    /// The ROM data as a vector of bytes.
    #[must_use]
    pub fn get_data(&self) -> &Vec<u8> {
        &self.0
    }
}

impl RomParser {
    /// Creates a new instance of the `RomParser` struct.
    ///
    /// # Arguments
    ///
    /// * `file_path` - The path to the ROM file.
    ///
    /// # Returns
    ///
    /// A new instance of the `RomParser` struct.
    #[must_use]
    pub fn new(file_path: PathBuf) -> Self {
        RomParser { file_path }
    }

    /// Reads the ROM file and returns a vector of bytes.
    ///
    /// # Arguments
    ///
    /// * `start_address` - The starting address of the ROM in memory.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `ValidRom` if the file is successfully read and the ROM is valid,
    /// or an error message if the file is not found or the ROM is invalid.
    ///
    /// # Errors
    ///
    /// Returns an error message if the file is not found or the ROM is invalid.
    pub fn read_rom(&self, start_address: u16) -> Result<ValidRom, String> {
        match std::fs::read(&self.file_path) {
            Ok(rom_data) => validate_rom(rom_data, start_address),
            Err(error) => {
                // If the file is not found or there was an error reading the file, return `Err(error_message)`
                Err(error.to_string())
            }
        }
    }
}

/// Validates the ROM data and returns a `ValidRom` if the ROM is valid.
///
/// # Arguments
///
/// * `rom_data` - The ROM data as a vector of bytes.
/// * `start_address` - The starting address of the ROM in memory.
///
/// # Returns
///
/// A `Result` containing a `ValidRom` if the ROM is valid, or an error message if the ROM is invalid.
fn validate_rom(rom_data: Vec<u8>, start_address: u16) -> Result<ValidRom, String> {
    if rom_data.len() < 2 {
        return Err("ROM file is too small".to_string());
    }
    if rom_data.len() > RAM_SIZE - start_address as usize {
        return Err("ROM file is too large".to_string());
    }
    Ok(new_valid_rom(rom_data))
}

fn new_valid_rom(rom_data: Vec<u8>) -> ValidRom {
    ValidRom(rom_data)
}

// Rom too small.
#[test]
fn test_get_rom_rom_too_small() {
    let rom_data = vec![0x00];
    let start_address = 0x200;
    let result = validate_rom(rom_data, start_address);
    assert_eq!(result, Err("ROM file is too small".to_string()));
}

// Rom too large.
#[test]
fn test_get_rom_rom_too_large() {
    let rom_data = vec![0x00; RAM_SIZE];
    let start_address = 0x200;
    let result = validate_rom(rom_data, start_address);
    assert_eq!(result, Err("ROM file is too large".to_string()));
}

// How rom_parser is used with emulator.
#[test]
fn test_load_rom() {
    let mut emu = Emu::new();
    let start_address = 0x200;
    let rom_bytes = vec![0x00, 0xE0, 0xA2, 0x1E, 0x60, 0x00, 0x61, 0x00];

    let valid_rom = new_valid_rom(rom_bytes.clone());
    emu.load_rom(&valid_rom);

    // Verify that the rom is loaded correctly into RAM
    for (i, &byte) in rom_bytes.iter().enumerate() {
        assert_eq!(emu.ram[start_address + i], byte);
    }
}
