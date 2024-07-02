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
    quit: bool,
}

#[derive(Debug, Default)]
pub enum EmulateState {
    #[default]
    Off,
    Running,
    Paused,
    // Stopped,
    Error,
}

#[derive(Debug, Default)]
pub struct EmulateOpts {
    pub sound: bool,
    pub debug: bool,
    // pub speed: u8,
    // pub rom: Option<Rom>,
}

#[derive(Debug, Default, Clone, Copy)]
pub enum CurrentScreen {
    #[default]
    Home,
    // Rom,  // maybe we want this, to load a roam from a path. Not sure
    Emulate, // Emulate the device
    Pause,
}

// impl App {
//     pub fn new(
//     ) -> Self {
//         Self {
//             current_screen: CurrentScreen::Home,
//             currently_editing: None,
//             counter: 0,
//             exit: false,
//         }
//     }
//
//
//     fn exit(&mut self) {
//         self.exit = true;
//     }
//
//     fn decrement_counter(&mut self) {
//         self.counter -= 1;
//     }
//
//     fn increment_counter(&mut self) -> Result<()> {
//         self.counter += 1;
//         if self.counter > 2 {
//             bail!("counter overflow");
//         }
//
//         Ok(())
//     }
// }
//
//
// #[cfg(test)]
// mod tests {
//     use ratatui::style::Style;
//
//     use super::*;
//
//     #[test]
//     fn render() {
//         let app = App::default();
//         let mut buf = Buffer::empty(Rect::new(0, 0, 50, 4));
//
//         app.render(buf.area, &mut buf);
//
//         let mut expected = Buffer::with_lines(vec![
//             "┏━━━━━━━━━━━━━ Counter App Tutorial ━━━━━━━━━━━━━┓",
//             "┃                    Value: 0                    ┃",
//             "┃                                                ┃",
//             "┗━ Decrement <Left> Increment <Right> Quit <Q> ━━┛",
//         ]);
//         let title_style = Style::new().bold();
//         let counter_style = Style::new().yellow();
//         let key_style = Style::new().blue().bold();
//         expected.set_style(Rect::new(14, 0, 22, 1), title_style);
//         expected.set_style(Rect::new(28, 1, 1, 1), counter_style);
//         expected.set_style(Rect::new(13, 3, 6, 1), key_style);
//         expected.set_style(Rect::new(30, 3, 7, 1), key_style);
//         expected.set_style(Rect::new(43, 3, 4, 1), key_style);
//
//         // note ratatui also has an assert_buffer_eq! macro that can be used to
//         // compare buffers and display the differences in a more readable way
//         assert_eq!(buf, expected);
//     }
//
//     #[test]
//     fn handle_key_event() {
//         let mut app = App::default();
//         app.handle_key_event(KeyCode::Right.into()).unwrap();
//         assert_eq!(app.counter, 1);
//
//         app.handle_key_event(KeyCode::Left.into()).unwrap();
//         assert_eq!(app.counter, 0);
//
//         let mut app = App::default();
//         app.handle_key_event(KeyCode::Char('q').into()).unwrap();
//         assert!(app.exit);
//     }
//
//     #[test]
//     #[should_panic(expected = "attempt to subtract with overflow")]
//     fn handle_key_event_panic() {
//         let mut app = App::default();
//         let _ = app.handle_key_event(KeyCode::Left.into());
//     }
//
//     #[test]
//     fn handle_key_event_overflow() {
//         let mut app = App::default();
//         assert!(app.handle_key_event(KeyCode::Right.into()).is_ok());
//         assert!(app.handle_key_event(KeyCode::Right.into()).is_ok());
//         assert_eq!(
//             app.handle_key_event(KeyCode::Right.into())
//                 .unwrap_err()
//                 .to_string(),
//             "counter overflow"
//         );
//     }
// }
