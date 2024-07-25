//! The Emu struct is used to emulate the CHIP-8 CPU.
use super::{
    registers, input, input::InputError, NUM_KEYS, RAM_SIZE, SCREEN_HEIGHT, SCREEN_WIDTH, SPRITE_SET, SPRITE_SET_SIZE,
    STACK_SIZE,
};

#[derive(Debug)]
/// The Emu struct is used to emulate the CHIP-8 CPU.
// TODO: consider whether this should be in topmost lib.rs and how API should be structured
pub struct Emu {
    /// Contains the program counter and stack pointer inside a `PsuedoRegisters` struct.
    pub(crate) psuedo_registers: registers::PsuedoRegisters,
    /// Contains the delay and sound timers inside a `SpecialRegisters` struct.
    pub(crate) special_registers: registers::SpecialRegisters,
    /// The CHIP-8 CPU has 16 general purpose registers.
    /// They are named V0, V1, ..., VE, VF.
    /// NOTE: The VF register is used as a flag in some instructions.
    pub(crate) general_registers: registers::GeneralRegisters,
    /// The I register is used to store memory addresses.
    pub(crate) i_register: u16,
    /// The ram size of the CHIP-8 emulator.
    pub(crate) ram: [u8; RAM_SIZE],
    /// The stack is used to store the address that the interpreter should return to when finished with a subroutine.
    pub(crate) stack: [u16; STACK_SIZE],
    /// The keyboard is used to store the state of the CHIP-8 keyboard.
    pub(crate) keys: [bool; NUM_KEYS],
    /// The screen is used to store the state of the CHIP-8 screen.
    pub(crate) screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    /// The input struct is used to map keyboard inputs to CHIP-8 keys.
    pub(crate) keymapping: input::Input,
}

// pub enum EmuError {
//     RomLoadError,
//     OpCodeError,
//     OtherError,
// }

impl Emu {
    /// Where the program counter starts.
    const START_ADDRESS: u16 = 0x200;

    /// The start address of the ETI 660 CHIP-8 interpreter.
    const ETI_START_ADDRESS: u16 = 0x600;

    #[must_use]
    #[allow(clippy::new_without_default)]
    /// Creates a new instance of the Emu struct.
    ///
    /// # Returns
    /// A new instance of the Emu struct.
    pub fn new() -> Self {
        let psuedo_registers = registers::PsuedoRegisters {
            program_counter: Self::START_ADDRESS,
            stack_pointer: 0,
        };

        let special_registers = registers::SpecialRegisters::default();

        let general_registers = registers::GeneralRegisters::default();

        let mut emu = Self {
            psuedo_registers,
            special_registers,
            general_registers,
            i_register: 0,
            ram: [0; RAM_SIZE],
            stack: [0; STACK_SIZE],
            keys: [false; NUM_KEYS],
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            keymapping: input::Input::default(),
        };

        // fill the first 80 bytes of memory with the character set
        // this works because we start at 0x200
        emu.ram[0..SPRITE_SET_SIZE].copy_from_slice(&SPRITE_SET);

        emu
    }

    //
    // pub fn cycle() -> Result<EmuError> {
    //     // 1. fetch_opcode
    //     // 2. execute_opcode
    // }

    /// Sets the start address of the emulator.
    pub fn set_start_address(&mut self, address: u16) {
        self.psuedo_registers.program_counter = address;
    }

    /// Resets the emulator to its initial state.
    /// With character set loaded into memory as well.
    pub fn reset(&mut self) {
        self.psuedo_registers.program_counter = Self::START_ADDRESS;
        self.psuedo_registers.stack_pointer = 0;
        self.special_registers = registers::SpecialRegisters::default();
        self.general_registers = registers::GeneralRegisters::default();
        self.i_register = 0;
        self.ram = [0; RAM_SIZE];
        self.stack = [0; STACK_SIZE];
        self.keys = [false; NUM_KEYS];
        self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
        self.ram[0..SPRITE_SET_SIZE].copy_from_slice(&SPRITE_SET);
    }

    #[must_use]
    /// Returns the screen size.
    pub fn screen_size() -> (usize, usize) {
        (SCREEN_WIDTH, SCREEN_HEIGHT)
    }

    pub(crate) fn get_register_val(&self, register: u8) -> u8 {
        self.general_registers.v[register as usize]
    }

    pub(crate) fn set_register_val(&mut self, register: u8, val: u8) {
        self.general_registers.v[register as usize] = val;
    }

    pub(crate) fn program_counter(&self) -> u16 {
        self.psuedo_registers.program_counter
    }

    pub(crate) fn set_program_counter(&mut self, address: u16) {
        self.psuedo_registers.program_counter = address;
    }

    /// Returns the current stack pointer.
    pub(crate) fn stack_pointer(&self) -> u8 {
        self.psuedo_registers.stack_pointer
    }

    /// Pushes the val of the address onto the stack.
    ///
    /// # Arguments
    /// * `address`: the address to push onto the stack.
    pub(crate) fn push_stack(&mut self, address: u16) {
        let sp = self.stack_pointer();
        self.stack[sp as usize] = address;
        self.psuedo_registers.stack_pointer += 1;
    }

    /// Pops the topmost address from the stack.
    pub(crate) fn pop_stack(&mut self) -> u16 {
        self.psuedo_registers.stack_pointer -= 1;
        let sp = self.stack_pointer();
        self.stack[sp as usize]
    }

    /// Gets the value of the delay timer register.
    pub(crate) fn get_delay_timer(&self) -> u8 {
        self.special_registers.delay_timer
    }

    /// Sets the value of the delay timer register.
    ///
    /// # Arguments
    /// * `val`: the value to set the delay timer to.
    pub(crate) fn set_delay_timer(&mut self, val: u8) {
        self.special_registers.delay_timer = val;
    }

    /// Gets the value of the sound timer register.
    pub(crate) fn get_sound_timer(&self) -> u8 {
        self.special_registers.sound_timer
    }

    /// Sets the value of the sound timer register.
    ///
    /// # Arguments
    /// * `val`: the value to set the delay timer to.
    pub(crate) fn set_sound_timer(&mut self, val: u8) {
        self.special_registers.sound_timer = val;
    }

    /// Ticks the delay and sound timers if they are greater than 0.
    /// Plays a sound if the sound timer is greater than 0.
    pub(crate) fn tick_timers(&mut self) {
        if self.special_registers.delay_timer > 0 {
            self.special_registers.delay_timer -= 1;
        }

        if self.special_registers.sound_timer > 0 {
            // #[cfg(feature = "sound")]
            // self.sound.play();
            self.special_registers.sound_timer -= 1;
        }
    }

    /// Changes the state of a key to pressed.
    pub fn press_key(&mut self, key: usize) {
        self.keys[key] = true;
    }

    /// Changes the state of a key to unpressed.
    pub fn release_key(&mut self, key: usize) {
        self.keys[key] = false;
    }

    #[must_use]
    /// Returns the mapped Chip-8 key for a given keyboard input.
    pub fn get_key_mapping(&self, input: &str) -> Option<&usize> {
        self.keymapping.get_key_mapping(input)
    }
    
    /// Sets a new mapping for a keyboard input to a CHIP-8 key.
    /// 
    /// # Arguments
    /// * `input`: the keyboard input to map.
    /// * `key`: the CHIP-8 key to map to the input.
    ///    
    /// # Errors
    /// Returns an error if the input is already mapped to a key.
    pub fn set_key_mapping(&mut self, input: &str, key: usize) -> Result<(), InputError>{
      match self.keymapping.set_key_mapping(input, key) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }   
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let emu = Emu::new();

        assert_eq!(emu.psuedo_registers.program_counter, Emu::START_ADDRESS);
        assert_eq!(emu.psuedo_registers.stack_pointer, 0);
        assert_eq!(emu.special_registers.delay_timer, 0);
        assert_eq!(emu.special_registers.sound_timer, 0);
        assert_eq!(emu.i_register, 0);
        assert_eq!(emu.stack, [0; STACK_SIZE]);
    }

    #[test]
    fn test_stack_pointer() {
        let emu = Emu::new();

        assert_eq!(emu.stack_pointer(), 0);
    }

    #[test]
    fn test_push_stack() {
        let mut emu = Emu::new();

        emu.push_stack(0x200);

        assert_eq!(emu.stack_pointer(), 1);
        assert_eq!(emu.stack[0], 0x200);
    }

    #[test]
    fn test_pop_stack() {
        let mut emu = Emu::new();

        emu.push_stack(0x200); // stack pointer is now 1

        assert_eq!(emu.pop_stack(), 0x200); // stack pointer is now 0
        assert_eq!(emu.stack_pointer(), 0); // stack pointer is now 0
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn test_tick_timers() {
        let mut emu = Emu::new();

        emu.set_delay_timer(1);
        emu.set_sound_timer(1);

        emu.tick_timers();
        std::thread::sleep(std::time::Duration::from_millis(250));

        assert_eq!(emu.get_delay_timer(), 0);
        assert_eq!(emu.get_sound_timer(), 0);
    }
}
