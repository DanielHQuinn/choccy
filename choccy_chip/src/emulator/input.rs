/// This module contains the input struct which maps keyboard inputs to the CHIP-8 keys.
use std::collections::HashMap;

#[derive(Debug)]
/// The Input struct is used to map keyboard inputs to CHIP-8 keys.
pub struct Input {
    keymapping: HashMap<String, u8>,
}

impl Default for Input {
    fn default() -> Self {
        let keys = [
            ("x".to_string(), 0x0),
            ("1".to_string(), 0x1),
            ("2".to_string(), 0x2),
            ("3".to_string(), 0x3),
            ("q".to_string(), 0x4),
            ("w".to_string(), 0x5),
            ("e".to_string(), 0x6),
            ("a".to_string(), 0x7),
            ("s".to_string(), 0x8),
            ("d".to_string(), 0x9),
            ("z".to_string(), 0xA),
            ("c".to_string(), 0xB),
            ("4".to_string(), 0xC),
            ("r".to_string(), 0xD),
            ("f".to_string(), 0xE),
            ("v".to_string(), 0xF),
        ];
        Self {
            keymapping: keys.iter().cloned().collect(),
        }
    }
}

impl Input {
    /// Sets a new mapping for a keyboard input to a CHIP-8 key.
    /// 
    /// # Arguments
    /// * `input`: the keyboard input to map.
    /// * `key`: the CHIP-8 key to map to the input.
    pub fn set_key_mapping(&mut self, input: &str, key: u8) {
        self.keymapping.insert(input.to_string(), key);
    }
    
    #[must_use]
    /// Gets the CHIP-8 key mapped to a keyboard input.
    pub fn get_key_mapping(&self, input: &str) -> Option<&u8> {
        self.keymapping.get(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let input = Input::default();
        assert_eq!(input.keymapping.len(), 16);
    }

    #[test]
    fn test_set_key_mapping() {
        let mut input = Input::default();
        input.set_key_mapping("t", 0x0);
        assert_eq!(input.keymapping.len(), 17);
    }

    #[test]
    fn test_get_key_mapping() {
        let input = Input::default();
        assert_eq!(input.get_key_mapping("x"), Some(&0x0));
        assert_eq!(input.get_key_mapping("t"), None);
    }
}