/// Defines the logic for the choocy TUI.
mod logic;
/// Creates the UI for the choocy TUI.
mod ui;
/// Handles key events for the choocy TUI.
mod key;
use choccy_chip::prelude::*;


#[derive(Debug)]
pub struct App {
    emu: Emu, // the actual emulator
    pub(crate) current_screen: CurrentScreen, // the current screen the user is looking at, and will later determine what is rendered.
    pub(crate) state: EmulateState,
    opts: EmulateOpts,
    // current_rom : Option<Rom>,
    quit: bool,
}

#[derive(Debug, Default)]
pub enum EmulateState {
    #[default]
    Off, // home
    Running, // running
    Paused, // (paused)
    Error,
}

// loop {
//  emu.cycle();
// }

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
    // Rom,  // maybe we want this, to load a roam from a path. Not sure
    Emulate, // Emulate the device
    Pause,
    // Remap,
    Remap,
}
