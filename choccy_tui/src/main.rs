// use choccy_chip::prelude::*;
// use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use std::io::Result;

mod app;
mod tui;

fn main() -> Result<()> {
    let mut terminal = tui::init()?;
    let app_result = app::App::default().run(&mut terminal);
    tui::restore()?;
    Ok(())
}
