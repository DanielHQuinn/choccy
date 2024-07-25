use super::{App, AppState, EmulateState};
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::text::{Span, Text};
use ratatui::widgets::canvas::{Canvas, Rectangle};
use ratatui::widgets::Paragraph;
use ratatui::widgets::{Block, Borders, Wrap};
use ratatui::Frame;
use ratatui::{layout::Rect, text::Line};

const SCALE_FACTOR: f64 = 2.0;

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

fn render_home(f: &mut Frame<'_>, area: Rect) {
    let home_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());
    let info = Paragraph::new(Text::styled(
        "Choocy is a TUI app for CHIP 8.",
        Style::default().fg(Color::Blue),
    ))
    .block(home_block)
    .alignment(Alignment::Center);

    f.render_widget(info, area);
}

#[allow(clippy::cast_precision_loss)]
fn render_screen(f: &mut Frame<'_>, app: &App, area: Rect) {
    let (width, height) = app.emu.screen_size();
    let screen = app.emu.screen();
    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Emulator Screen"),
        )
        .paint(|ctx| {
            for y in 0..height {
                for x in 0..width {
                    if screen[y * width + x] {
                        ctx.draw(&Rectangle {
                            x: (x as f64) * SCALE_FACTOR,
                            y: (y as f64) * SCALE_FACTOR,
                            width: SCALE_FACTOR,
                            height: SCALE_FACTOR,
                            color: Color::White,
                        });
                    }
                }
            }
        })
        .x_bounds([0.0, (width as f64) * SCALE_FACTOR])
        .y_bounds([0.0, (height as f64) * SCALE_FACTOR]);

    f.render_widget(canvas, area);
}

fn render_emulator(f: &mut Frame<'_>, app: &App, area: Rect) {
    // main block
    match app.emu_state {
        EmulateState::Off => render_home(f, area),
        EmulateState::Running => render_screen(f, app, area),
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

            let area = centered_rect(60, 50, f.size());
            f.render_widget(pause_block, area);

            // TODO tell emulator to pause
        }
        // I assume we should map libraries errors here, and also if the emulator itself has an error
        // this should be a current_screen
        EmulateState::Error => {
            let error_block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default());

            let error = Paragraph::new(Text::styled("IDK mate", Style::default().fg(Color::Red)))
                .block(error_block);

            f.render_widget(error, area);
        }
    }
}

pub fn ui(f: &mut Frame<'_>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // header
            Constraint::Min(1),    // main content
            Constraint::Length(3), // footer
        ])
        .split(f.size());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled("Choocy", Style::default().fg(Color::Green)))
        .block(title_block)
        .alignment(Alignment::Center);

    f.render_widget(title, chunks[0]);

    match app.app_state {
        AppState::Home => render_home(f, chunks[1]),
        AppState::Rom => todo!(),
        AppState::Emulate => render_emulator(f, app, chunks[1]),
        AppState::Remap => todo!(),
        AppState::Pause => todo!(), // only reachable from Emulate
        AppState::Quit => todo!(),
    }

    // footer
    let current_navigation_text = vec![
        // The first half of the text
        match app.app_state {
            AppState::Home => Span::styled("Home", Style::default().fg(Color::Green)),
            AppState::Rom => Span::styled("Rom", Style::default().fg(Color::Yellow)),
            _ => todo!(),
        }

        .clone(),
        // A white divider bar to separate the two sections
        Span::styled(" | ", Style::default().fg(Color::White)),
        // The final section of the text, with hints on whether the emulator is running or not
        {
            match app.emu_state {
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
        match app.app_state {
            AppState::Home => {
                // TODO: should we add a load, save, or configure option here?
                Span::styled("(q) to quit / (r) to run", Style::default().fg(Color::Red))
            }
            AppState::Emulate => todo!(),
            _ => todo!(),
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
