#![warn(missing_docs, missing_debug_implementations, rust_2018_idioms)]
#![warn(clippy::pedantic)]

//! Choccy TUI is a TUI for the Choccy Chip CHIP-8 emulator.

use color_eyre::Result;
/// Where the choocy app is defined. Includes the `App` struct and the `CurrentScreen` enum.
mod choocy;
/// Error handling for the TUI
mod errors;
/// The TUI module, where the `TUI` is initialized.
mod tui;

fn main() -> Result<()> {
    errors::install_hooks()?; // error handling
    let mut terminal = tui::init()?;

    // everything is handled in the app module
    // edit this!
    choocy::App::new().run(&mut terminal)?;


    tui::restore()?;
    Ok(())
}
