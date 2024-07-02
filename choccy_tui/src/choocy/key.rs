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
            // _ => {emu.fetch(); emu.excute} // our library needs to tell us when we need an input
            _ => Ok(()),
        }
    }

    pub fn handle_key_event(&mut self, key_event:KeyEvent) -> Result<()> {
        todo!()
    }



    // fn correct_map(remap: self.HashMap<"og key", "user remaped key">, key) -> Key {
    //     return ramap.get(key);
    // }
    //
    // fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
    //     // TODO; figure how we are going to handle custom inputs
    //     let key = correct_map(key_event.code);
    //     handle_key(key)
    // }
    //
    // fn handle_key(key: KeyCode) {
    //     match self.state {
    //         EmulateState::Running => {
    //             // emu.handle_key(key)
    //         }
    //         EmulateState::Paused => {
    //             match key {
    //                 KeyCode::Char('r') => self.handle_r(),
    //                 _ => Ok(()),
    //             }
    //         }
    //         EmulateState::Off => {
    //             match key {
    //                 KeyCode::Char('q') => self.handle_q(),
    //                 KeyCode::Char('r') => self.handle_r(),
    //                 _ => Ok(()),
    //             }
    //         }
    //         EmulateState::Error => {
    //             bail!("Error state, cannot handle key");
    //         }
    //     }
    // }

}
