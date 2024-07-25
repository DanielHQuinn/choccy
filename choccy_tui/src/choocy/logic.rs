use super::{ui::ui, App};
use super::{CurrentScreen, EmulateOpts, EmulateState};
use crate::tui;
use choccy_chip::emulator::emulator::Emu;
use color_eyre::eyre::WrapErr;
use color_eyre::Result;

impl App {
    /// Handle key events
    pub fn run(&mut self, terminal: &mut tui::Tui) -> Result<()> {
        // step 1. init the emulator
        //  - init the screen / (I don't mean render)

        while !self.quit {
            // step 2. we render the screen
            // - we need to render the home screen, not the emulator
            terminal.draw(|f| ui(f, self))?; // Charlie

            // step 3. handle key inputs
            // - case work:
            //  - 0. home screen (Albert)
            //  - 1. remaping is entered by some key (Albert)
            //  - 2. rom loaded (After albert is done, Danny)
            //  - 3. emulator running (any)
            match self.current_screen {
                // <c-q> to quit  or <blackslash>
                CurrentScreen::Remap => {
                    // 1.remap
                    self.handle_remap().wrap_err("Failed to handle event")?;
                    // self.handle_remap().wrap_err("Failed to handle remap")?;
                }
                CurrentScreen::Home => self.handle_home(), // 0. home screen
                // Options:
                // 1. start emulator - s
                // 2. quit tui - ctrl + q
                //       when emulating: pause and then ctrl + q again to quit
                //       when remapping: return to home screen
                // 3. remap keys - r
                // 4. load rom - l


                _ => todo!(), // CurrentScreen::Emulate => self.handle_emulate().wrap_err("Failed to handle emulate")?, // 3. emulator running
                              // CurrentScreen::Rom
            }

            let condition: bool = true;

            // step 4. emulate i.e., fetch and execute
            if self.state == EmulateState::Running && condition {
                // charlie is handling, emu error and cycle
                // self.emu.cycle().wrap_err("Failed to cycle")?;
                //
                // albert
                // audio
                // call tick timer, a bool for audio
                // if true, play audio
            }

            // at this point, if the emulator is running, we made a cycle
            // if not, we handled everything
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
