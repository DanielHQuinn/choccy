use super::{ui::ui, App};
use super::{CurrentScreen, EmulateOpts, EmulateState};
use crate::tui;
use choccy_chip::emulator::emulator::Emu;
use color_eyre::eyre::WrapErr;
use color_eyre::Result;

impl App {
    /// Handle key events
    pub fn run(&mut self, terminal: &mut tui::Tui) -> Result<()> {
        while !self.quit {
            terminal.draw(|f| ui(f, self))?;
            match self.current_screen {
                CurrentScreen::Remap => {
                    todo!()
                    // self.handle_remap().wrap_err("Failed to handle remap")?;
                }
                _ => self.handle_event().wrap_err("Failed to handle event")?,
            }
        }
        Ok(())
    }

    pub fn new() -> Self {
        Self {
            emu: Emu::new(),
            current_screen: CurrentScreen::Home,
            state: EmulateState::Off,
            opts: EmulateOpts::default(),
            quit: false,
        }
    }
}
