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
mod simulated_cluster;
mod command;
use simulated_cluster::{ClusterClient, get_client};
use command::{CommandState, CentralView};

#[cfg(test)]
mod tests {
    use super::*;

    // TEST: GIVEN main WHEN called THEN config loads without panic
    #[test]
    fn test_main_config_loads() {
        let cfg = config::Config::load();
        // Just check that simulated is a bool
        assert!(cfg.simulated == true || cfg.simulated == false);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = config::Config::load();
    println!("Loaded config: {:?}", cfg);

    let mut client: Option<Box<dyn ClusterClient>> = None;
    if cfg.simulated {
        println!("Running in simulated mode...");
        client = Some(get_client());
    } else {
        println!("Running with real cluster connection...");
        // later: real K8s client logic
    }

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

            // Detail panel (status + instructions)
            let status_text = if let Some(ref c) = client {
                c.status()
            } else {
                "No cluster client initialized".to_string()
            };
            let detail_panel = Paragraph::new(vec![
                Line::from(Span::styled(
                    status_text,
                    Style::default().fg(Color::Yellow),
                )),
                Line::from(Span::raw("Press 'q' to quit.")),
            ])
            .block(Block::default().title("[clust]").borders(Borders::ALL))
            .alignment(Alignment::Left);
            f.render_widget(detail_panel, chunks[0]);

            // Central view
            match command_state.view {
                CentralView::Pods => {
                    let pods = if let Some(ref c) = client {
                        c.get_pods()
                    } else {
                        vec!["No pods (no client)".to_string()]
                    };
                    let pods_lines: Vec<Line> = pods.iter().map(|p| Line::from(Span::raw(p))).collect();
                    let pods_paragraph = Paragraph::new(pods_lines)
                        .block(Block::default().title("Pods").borders(Borders::ALL))
                        .alignment(Alignment::Left);
                    f.render_widget(pods_paragraph, chunks[1]);
                }
                CentralView::Help => {
                    let help_lines = vec![
                        Line::from(Span::raw(":pods - show pods")),
                        Line::from(Span::raw(":help - show help")),
                    ];
                    let help_paragraph = Paragraph::new(help_lines)
                        .block(Block::default().title("Help").borders(Borders::ALL))
                        .alignment(Alignment::Left);
                    f.render_widget(help_paragraph, chunks[1]);
                }
            }

            // Command entry box
            let command_paragraph = Paragraph::new(command_state.input.clone())
                .block(Block::default().title("Command").borders(Borders::ALL))
                .alignment(Alignment::Left);
            f.render_widget(command_paragraph, chunks[2]);
        })?;

        // Handle key events
        let Event::Key(key) = event::read()? else { continue };
        match key.code {
            KeyCode::Char('q') => break,
            KeyCode::Enter => {
                command_state.handle_command();
            },
            KeyCode::Char(c) => {
                command_state.input.push(c);
            },
            KeyCode::Backspace => {
                command_state.input.pop();
            },
            _ => {}
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        crossterm::terminal::LeaveAlternateScreen
    )?;
    Ok(())
}

