use std::time::{Duration, Instant};

use super::{ui::ui, App};
use super::{AppState, EmulateState, Speed};
use crate::tui;
use choccy_chip::prelude::Emu;
use color_eyre::eyre::WrapErr;
use color_eyre::Result;

impl Speed {
    fn as_tick_rate(&self) -> Duration {
        match self {
            Speed::Slow => Duration::from_millis(100),
            Speed::Normal => Duration::from_millis(50),
            Speed::Fast => Duration::from_millis(10),
        }
    }
}

impl App {
    /// Handle key events
    pub fn run(&mut self, terminal: &mut tui::Tui) -> Result<()> {
        // step 1. init the emulator. ALREADY DONE
        // - we just need to init the tick timer

        let mut last_tick = Instant::now();
        let tick_rate = self.speed.as_tick_rate();

        loop {
            if self.app_state == AppState::Quit {
                return Ok(());
            }

            // step 2. we render the screen
            // - we need to render the home screen, not the emulator
            terminal.draw(|f| ui(f, self))?; // Charlie

            // step 3. handle key inputs
            // - case work:
            //  - 0. home screen (Albert)
            //  - 1. remaping is entered by some key (Albert)
            //  - 2. rom loaded (After albert is done, Danny)
            //  - 3. emulator running (any)
            match self.app_state {
                // <c-q> to quit  or <blackslash>
                AppState::Remap => {
                    // 1.remap
                    todo!()
                    // self.handle_remap().wrap_err("Failed to handle remap")?;
                }
                AppState::Home => self.handle_event().wrap_err("Failed to handle event")?, // 0. home screen
                _ => todo!(), // AppState::Emulate => self.handle_emulate().wrap_err("Failed to handle emulate")?, // 3. emulator running
                              // AppState::Rom
            }

            // step 4. emulate i.e., fetch and execute
            if self.emu_state == EmulateState::Running && last_tick.elapsed() >= tick_rate {
                // charlie is handling, emu error and cycle
                // self.emu.cycle().wrap_err("Failed to cycle")?;
                //
                // albert
                // audio
                // call tick timer, a bool for audio
                // if true, play audio

                last_tick = Instant::now();
            }

            //     // at this point, if the emulator is running, we made a cycle
            //     // if not, we handled everything
        }
    }

    pub fn new() -> Self {
        Self {
            emu: Emu::new(),
            app_state: AppState::Home,
            emu_state: EmulateState::Off,
            sound: false,
            debug: false,
            rom: None,
            speed: Speed::Normal,
        }
    }
}
