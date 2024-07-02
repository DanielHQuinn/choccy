use super::{App, CurrentScreen, EmulateState};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::text::{Span, Text};
use ratatui::widgets::Paragraph;
use ratatui::widgets::{Block, Borders, Wrap};
use ratatui::Frame;
use ratatui::{layout::Rect, text::Line};

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}

#[allow(clippy::too_many_lines)]
pub fn ui(f: &mut Frame<'_>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(f.size());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled("Choocy", Style::default().fg(Color::Green)))
        .block(title_block);

    f.render_widget(title, chunks[0]);

    // main block
    match app.state {
        EmulateState::Off => {
            let info_block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default());

            let info = Paragraph::new(Text::styled(
                "Choocy is a TUI app for CHIP 8.",
                Style::default().fg(Color::Blue),
            ))
            .block(info_block);

            f.render_widget(info, chunks[1]);
        }
        EmulateState::Running => {
            let running_block = Block::default()
                .title("Emulator Running")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Green));

            let area = centered_rect(60, 25, f.size());
            f.render_widget(running_block, area);

            // now we need to figure out the logic needed to render the emulator screen
        }
        EmulateState::Paused => {
            let popup = Block::default()
                .title("Pause")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::DarkGray));

            let keybind_text =
                Text::styled("Press (r) to resume", Style::default().fg(Color::Green));

            let pause_block = Paragraph::new(keybind_text)
                .block(popup)
                .wrap(Wrap { trim: false });

            let area = centered_rect(60, 25, f.size());
            f.render_widget(pause_block, area);

            // TODO tell emulator to pause
        }
        EmulateState::Error => {
            let error_block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default());

            let error = Paragraph::new(Text::styled(
                "IDK mate",
                Style::default().fg(Color::Red),
            ))
            .block(error_block);

            f.render_widget(error, chunks[1]);
        }
    }

    let current_navigation_text = vec![
        // The first half of the text
        match app.current_screen {
            CurrentScreen::Home => Span::styled("Normal Mode", Style::default().fg(Color::Green)),
            CurrentScreen::Emulate => {
                Span::styled("Emulate Mode", Style::default().fg(Color::Yellow))
            }
            CurrentScreen::Pause => Span::styled("Pause", Style::default().fg(Color::LightRed)),
        }
        .clone(),
        // A white divider bar to separate the two sections
        Span::styled(" | ", Style::default().fg(Color::White)),
        // The final section of the text, with hints on whether the emulator is running or not
        {
            match app.state {
                EmulateState::Off => {
                    Span::styled("Not Running", Style::default().fg(Color::DarkGray))
                }
                EmulateState::Running => Span::styled("Running", Style::default().fg(Color::Green)),
                EmulateState::Paused => {
                    Span::styled("Paused", Style::default().fg(Color::LightRed))
                }
                EmulateState::Error => Span::styled("Error", Style::default().fg(Color::Red)),
            }
        },
    ];

    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Home => {
                // TODO: should we add a load, save, or configure option here?
                Span::styled("(q) to quit / (r) to run", Style::default().fg(Color::Red))
            }
            CurrentScreen::Emulate => todo!(),
            CurrentScreen::Pause => Span::styled("(q) to quit", Style::default().fg(Color::Red)),
        }
    };

    let key_notes_footer =
        Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);
    f.render_widget(mode_footer, footer_chunks[0]);
    f.render_widget(key_notes_footer, footer_chunks[1]);
}
