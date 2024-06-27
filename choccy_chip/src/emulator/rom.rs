use crate::emulator::RAM_SIZE;
use std::{fs::File, io::Read, path::PathBuf};
struct RomParser {
    file_path: PathBuf,
}

/// Represents a validated ROM file.
pub type ValidatedRomBytes = Vec<u8>;

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
    pub fn new(file_path: PathBuf) -> Self {
        Self { file_path }
    }

    /// Reads the ROM file and returns a vector of bytes.
    ///
    /// # Returns
    ///
    /// A vector of bytes representing the ROM file.
    fn read(&self) -> Result<ValidatedRomBytes, &str> {
        // The start address of the CHIP-8 interpreter.
        const START: usize = 0x200;
        // The maximum size of the ROM file.
        const MAX_ROM_SIZE: usize = RAM_SIZE - START;

        let mut file = File::open(&self.file_path).expect("File not found");
        let mut rom_buffer = Vec::new();
        file.read_to_end(&mut rom_buffer)
            .expect("Failed to read file");

        // If buffer is empty, return an error
        if rom_buffer.is_empty() {
            return Err("rom is empty");
        }

        // If buffer is too large ( > MAX_ROM_SIZE bytes), return an error
        if rom_buffer.len() > MAX_ROM_SIZE {
            return Err("Rom size exceeds maximum limit");
        }
        Ok(rom_buffer)
    }
}
