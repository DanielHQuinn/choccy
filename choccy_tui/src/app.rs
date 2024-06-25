use ratatui::layout::Alignment;
use ratatui::symbols::border;
use ratatui::text::Text;
use ratatui::widgets::block::Position;
use ratatui::{style::Stylize, widgets::Paragraph};
use ratatui::widgets::{Block, Borders};
use std::io;
use ratatui::{buffer::Buffer, layout::Rect, text::Line, widgets::{block::Title, Widget}, Frame};

use crate::tui;

#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
}

impl App {
    /// Handle key events
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            // should only call once
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_event()?;
        }

        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn handle_event(&mut self) -> io::Result<()> {
        todo!()
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let title = Title::from("Choccy Chip TUI");
        let instructions = Title::from(Line::from(vec![
            "Decrease counter: ".into(),
            "<Left>".blue().bold(),
            " | Increase counter: ".into(),
            "<Right>".blue().bold(),
            " | Quit: ".into(),
            "<Q>".blue().bold(),
        ]));

        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .borders(Borders::ALL)
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.counter.to_string().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
