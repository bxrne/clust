
use std::io;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::Alignment,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

mod config;
mod client;
mod command;

use client::get_client;
use command::{CommandState, CentralView};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load config
    let cfg = config::Config::load();
    println!("Loaded config: {:?}", cfg);

    // Get appropriate client (real or simulated)
    let client = get_client();

    // Command handling state
    let mut command_state = CommandState::new();

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // App loop
    loop {
        terminal.draw(|f| {
            use ratatui::layout::{Layout, Constraint, Direction};
            let size = f.area();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(4), // Top detail panel
                    Constraint::Min(0),    // Central view
                    Constraint::Length(3), // Command entry
                ])
                .split(size);

            // ---- Detail panel ----
            let detail_panel = Paragraph::new(vec![
                Line::from(Span::styled(
                    client.status(),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(ratatui::style::Modifier::BOLD),
                )),
                Line::from(Span::styled(
                    "Press 'q' to quit.",
                    Style::default()
                        .fg(Color::LightGreen)
                        .add_modifier(ratatui::style::Modifier::ITALIC),
                )),
            ])
            .block(
                Block::default()
                    .title(Span::styled(
                        "[clust]",
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(ratatui::style::Modifier::BOLD),
                    ))
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .alignment(Alignment::Left);
            f.render_widget(detail_panel, chunks[0]);

            // ---- Central view ----
            match command_state.view {
                CentralView::Pods => {
                    let pods = client.get_pods();
                    let pods_lines: Vec<Line> = pods
                        .iter()
                        .enumerate()
                        .map(|(i, p)| {
                            Line::from(Span::styled(
                                format!("{}: {}", i + 1, p),
                                Style::default()
                                    .fg(Color::LightMagenta)
                                    .add_modifier(ratatui::style::Modifier::BOLD),
                            ))
                        })
                        .collect();
                    let pods_paragraph = Paragraph::new(pods_lines)
                        .block(
                            Block::default()
                                .title(Span::styled(
                                    "Pods",
                                    Style::default()
                                        .fg(Color::Magenta)
                                        .add_modifier(ratatui::style::Modifier::BOLD),
                                ))
                                .borders(Borders::ALL)
                                .border_style(Style::default().fg(Color::Magenta)),
                        )
                        .alignment(Alignment::Left);
                    f.render_widget(pods_paragraph, chunks[1]);
                }
                CentralView::Ctxs => {
                    let ctxs = client.get_contexts();
                    let ctxs_lines: Vec<Line> = ctxs
                        .iter()
                        .enumerate()
                        .map(|(i, c)| {
                            Line::from(Span::styled(
                                format!("{}: {}", i + 1, c),
                                Style::default()
                                    .fg(Color::LightYellow)
                                    .add_modifier(ratatui::style::Modifier::BOLD),
                            ))
                        })
                        .collect();
                    let ctxs_paragraph = Paragraph::new(ctxs_lines)
                        .block(
                            Block::default()
                                .title(Span::styled(
                                    "Contexts",
                                    Style::default()
                                        .fg(Color::Yellow)
                                        .add_modifier(ratatui::style::Modifier::BOLD),
                                ))
                                .borders(Borders::ALL)
                                .border_style(Style::default().fg(Color::Yellow)),
                        )
                        .alignment(Alignment::Left);
                    f.render_widget(ctxs_paragraph, chunks[1]);
                }
                CentralView::Help => {
                    let help_lines = vec![
                        Line::from(Span::styled(
                            "Available commands:",
                            Style::default()
                                .fg(Color::White)
                                .add_modifier(ratatui::style::Modifier::UNDERLINED),
                        )),
                        Line::from(Span::styled(
                            ":q - quit application",
                            Style::default()
                                .fg(Color::LightBlue)
                                .add_modifier(ratatui::style::Modifier::BOLD),
                        )),
                        Line::from(Span::styled(
                            ":togglesim - toggle simulated mode",
                            Style::default()
                                .fg(Color::LightBlue)
                                .add_modifier(ratatui::style::Modifier::BOLD),
                        )),
                        Line::from(Span::styled(
                            ":pods - show pods",
                            Style::default()
                                .fg(Color::LightBlue)
                                .add_modifier(ratatui::style::Modifier::BOLD),
                        )),
                        Line::from(Span::styled(
                            ":ctx - show contexts",
                            Style::default()
                                .fg(Color::LightBlue)
                                .add_modifier(ratatui::style::Modifier::BOLD),
                        )),
                        Line::from(Span::styled(
                            ":help - show help",
                            Style::default()
                                .fg(Color::LightBlue)
                                .add_modifier(ratatui::style::Modifier::BOLD),
                        )),
                        Line::from(Span::styled(
                            "Type a command below and press Enter.",
                            Style::default()
                                .fg(Color::Gray)
                                .add_modifier(ratatui::style::Modifier::ITALIC),
                        )),
                    ];
                    let help_paragraph = Paragraph::new(help_lines)
                        .block(
                            Block::default()
                                .title(Span::styled(
                                    "Help",
                                    Style::default()
                                        .fg(Color::Blue)
                                        .add_modifier(ratatui::style::Modifier::BOLD),
                                ))
                                .borders(Borders::ALL)
                                .border_style(Style::default().fg(Color::Blue)),
                        )
                        .alignment(Alignment::Left);
                    f.render_widget(help_paragraph, chunks[1]);
                }
            }

            // ---- Command entry ----
            let command_paragraph = Paragraph::new(Span::styled(
                command_state.input.clone(),
                Style::default()
                    .fg(Color::White)
                    .add_modifier(ratatui::style::Modifier::BOLD),
            ))
            .block(
                Block::default()
                    .title(Span::styled(
                        "Command",
                        Style::default()
                            .fg(Color::Green)
                            .add_modifier(ratatui::style::Modifier::BOLD),
                    ))
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Green)),
            )
            .alignment(Alignment::Left);
            f.render_widget(command_paragraph, chunks[2]);
        })?;

        // ---- Key handling ----
        let Event::Key(key) = event::read()? else { continue };
        match key.code {
            KeyCode::Char('q') => break,
            KeyCode::Enter => {
                command_state.handle_command();
            }

            KeyCode::Char(c) => {
                command_state.input.push(c);
            }
            KeyCode::Backspace => {
                command_state.input.pop();
            }
            _ => {}
        }
    }
    // Restore Terminal
    //
    disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        crossterm::terminal::LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;
    Ok(())
}

