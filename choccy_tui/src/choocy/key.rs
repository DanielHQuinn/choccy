use color_eyre::Result;
use color_eyre::eyre::{bail, WrapErr};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use super::{App, CurrentScreen, EmulateState};

impl App {
    pub fn handle_home(&mut self) {
        match event::read() {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Ok(Event::Key(key_event)) if key_event.kind == KeyEventKind::Press => self
                .handle_home_key_event(key_event),
                //.wrap_err_with(|| format!("handling key event failed:\n {key_event:#?}")),
            // _ => {emu.fetch(); emu.excute} // our library needs to tell us when we need an input
            _ => {},
        }
    }
    
    /// Handles key events for the home screen.
    pub fn handle_home_key_event(&mut self, key_event:KeyEvent) {
        if let KeyCode::Char(c) = key_event.code {
            let key_str = c.to_string();
            match key_str.as_str() {
                "s" => {
                    // match on rom: if none send to rom screen
                    self.current_screen = CurrentScreen::Emulate;
                    self.state = EmulateState::Running;
                }
                "q" => {
                    if key_event.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) {
                        self.quit = true;
                    }
                }
                "r" => {
                    self.current_screen = CurrentScreen::Remap;
                }
                "l" => {
                    self.current_screen = CurrentScreen::Rom;
                }
                _ => {}
            }
        }
    }

    pub fn handle_remap(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_remap_event(key_event)
                    .wrap_err_with(|| format!("handling key remap event failed:\n {key_event:#?}"))
            }
            _ => Ok(()),
        }
    }

    /// Handles key events for the remap screen.
    pub fn handle_key_remap_event(&mut self, key_event: KeyEvent) -> Result<()> {
        if let KeyCode::Char(c) = key_event.code {
            let key_str = c.to_string();
            // Return to home screen if ctrl + q is pressed
            if key_str == "q" && key_event.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) {
                self.current_screen = CurrentScreen::Home;
                return Ok(());
            }
            // Remap the key that was pressed
            if let Some(&_chip8_key) = self.emu.get_key_mapping(&key_str) {
                match event::read()? {
                    Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                        if let KeyCode::Char(c) = key_event.code {
                            let key_str = c.to_string();
                            if let Some(&chip8_key) = self.emu.get_key_mapping(&key_str) {
                                let err = self.emu.set_key_mapping(&key_str, chip8_key);
                                if let Err(e) = err {
                                    bail!("Failed to remap key: {}", e);
                                }
                            }
                        }
                    }
                    _ => return Ok(()),
                
                }
            }
        }
        Ok(())
    }

    pub fn handle_emulate(&mut self) {
        match event::read() {
            Ok(Event::Key(key_event)) if key_event.kind == KeyEventKind::Press => {
                self.handle_emulate_key_event(key_event, true);
                    //.wrap_err_with(|| format!("handling key event failed:\n {key_event:#?}"))
            }
            Ok(Event::Key(key_event)) if key_event.kind == KeyEventKind::Release => {
                self.handle_emulate_key_event(key_event, false);
                    //.wrap_err_with(|| format!("handling key event failed:\n {key_event:#?}"))
            }
            _ => {},
        }
    }

    /// Handles key events for the emulator screen.
    /// Ctrl + q will quit the emulator.
    pub fn handle_emulate_key_event(&mut self, key_event: KeyEvent, state: bool) {
        if let KeyCode::Char(c) = key_event.code {
            let key_str = c.to_string();
            match key_str.as_str() {
                "q" => {
                    // Quit the emulator if ctrl + q is pressed
                    if key_event.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) && state {
                        self.quit = true;
                    } else if let Some(&chip8_key) = self.emu.get_key_mapping(&key_str) {
                        if state {
                            self.emu.press_key(chip8_key);
                        } else {
                            self.emu.release_key(chip8_key);
                        }
                    }
                }
                _ => {
                    if let Some(&chip8_key) = self.emu.get_key_mapping(&key_str) {
                        if state {
                            self.emu.press_key(chip8_key);
                        } else {
                            self.emu.release_key(chip8_key);
                        }
                    }
                },
                
            }
        }
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
