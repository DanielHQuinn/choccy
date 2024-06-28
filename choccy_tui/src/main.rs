use color_eyre::Result;
mod app;
mod errors;
mod tui;

fn main() -> Result<()> {
    errors::install_hooks()?; // error handling
    let mut terminal = tui::init()?;
    app::App::default().run(&mut terminal)?;
    tui::restore()?;
    Ok(())
}
