use color_eyre::Result;
use color_eyre::eyre::{bail, WrapErr};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use super::{App, EmulateState};

impl App {
    pub fn handle_event(&mut self) -> Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => self
                .handle_key_event(key_event)
                .wrap_err_with(|| format!("handling key event failed:\n {key_event:#?}")),
            _ => Ok(()),
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Char('q') => self.handle_q(),
            KeyCode::Char('r') => self.handle_r(),
            _ =>  Ok(()),
        }
    }

    fn handle_r(&mut self) -> Result<()> {
        match self.state {
            EmulateState::Paused | EmulateState::Off => {
                self.state = EmulateState::Running;
                // TODO: Handle resume
                Ok(())
                // Handle resume
            }
            EmulateState::Error => {
                bail!("Error state, cannot resume");
            },
            EmulateState::Running => todo!(),
        }
    }

    fn handle_q(&mut self) -> Result<()>{
        match self.state {
            EmulateState::Running => {
                self.state = EmulateState::Paused;
                // Handle pause
                Ok(())
            }
            EmulateState::Paused | EmulateState::Off => {
                self.quit = true;
                // Handle quit
                Ok(())
            }
            EmulateState::Error => {
                bail!("Error state, cannot quit");
            }
        }
    }



}
