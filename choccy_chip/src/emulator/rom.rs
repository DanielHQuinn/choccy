use std::{fs::File, io::Read, path::PathBuf};
struct RomParser {
    file_path: PathBuf,
}

impl RomParser {
    /// Creates a new instance of the RomParser struct.
    ///
    /// # Arguments
    ///
    /// * `file_path` - The path to the ROM file.
    ///
    /// # Returns
    ///
    /// A new instance of the RomParser struct.
    pub fn new(file_path: PathBuf) -> Self {
        Self { file_path }
    }

    /// Reads the ROM file and returns a vector of bytes.
    ///
    /// # Returns
    ///
    /// A vector of bytes representing the ROM file.
    fn read(&self) -> Result<Vec<u8>, &str> {
        let mut file = File::open(&self.file_path).expect("File not found");
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).expect("Failed to read file");

        // If buffer is empty, return an error
        if buffer.is_empty() {
            return Err("Buffer is empty");
        }
        // If buffer is too large ( > 3584 bytes), return an error
        if buffer.len() > 3584 {
            return Err("Buffer size exceeds maximum limit");
        }
        Ok(buffer)
    }
}
