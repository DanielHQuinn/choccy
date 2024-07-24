/// Handles key events for the choocy TUI.
mod key;
/// Defines the logic for the choocy TUI.
mod logic;
/// Creates the UI for the choocy TUI.
mod ui;
use choccy_chip::prelude::*;

#[derive(Debug)]
pub struct App {
    emu: Emu,                                 // the actual emulator
    pub(crate) current_screen: CurrentScreen, // the current screen the user is looking at, and will later determine what is rendered.
    pub(crate) state: EmulateState,
    opts: EmulateOpts,
    // current_rom : Option<Rom>,
    rom_path: String, // path to the rom
    quit: bool,
}

#[derive(Debug, Default, PartialEq)]
pub enum EmulateState {
    #[default]
    Off, // home
    Running, // running
    Paused,  // (paused)
    Error,
}

#[derive(Debug, Default)]
pub struct EmulateOpts {
    pub sound: bool,
    pub debug: bool,
    // pub remap: HashMap<KeyCode, KeyCode>,
    // pub speed: u8,
    // pub rom: Option<Rom>,
}

#[derive(Debug, Default, Clone, Copy)]
pub enum CurrentScreen {
    #[default]
    Home, // press r to start, q to quit
    Rom,     // maybe we want this, to load a roam from a path. Not sure
    Emulate, // Emulate the device
    Remap,
}

// danny needs to do rom,
// -f rom_path,
// - you need to 1. use something like clap to parse the args
// - and also, write the logic to load the rom
// document what the hrz is because you looked through other emulators
