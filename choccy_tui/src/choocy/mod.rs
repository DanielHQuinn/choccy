/// Handles key events for the choocy TUI.
mod key;
/// Defines the logic for the choocy TUI.
mod logic;
/// Creates the UI for the choocy TUI.
mod ui;
use choccy_chip::prelude::*;

#[derive(Debug, Default)]
pub struct App {
    emu: Emu,                                 // the actual emulator
    pub(crate) app_state: AppState, // the current state of the app
    pub(crate) emu_state: EmulateState,
    // remap: HashMap<Key, Key>,
    sound: bool,
    debug: bool,
    rom: Option<String>,
    speed: Speed,
}

#[derive(Debug, Default)]
pub enum Speed {
    Slow,
    #[default]
    Normal,
    Fast,
}

// this is for the state of the emulator. think logic
#[derive(Debug, Default, PartialEq)]
pub enum EmulateState {
    #[default]
    Off, // home
    Running, // running
    Paused,  // (paused)
    Error,
}

// this is for the state of the app. think UI
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum AppState {
    #[default]
    Home, // press r to start, q to quit
    Rom,     // maybe we want this, to load a roam from a path. Not sure
    Emulate, // Emulate the device
    Remap,
    Pause,
    Quit,
}

// danny needs to do rom,
// -f rom_path,
// - you need to 1. use something like clap to parse the args
// - and also, write the logic to load the rom
// document what the hrz is because you looked through other emulators
