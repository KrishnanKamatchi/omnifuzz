use ratatui::{prelude::*, widgets::*};
use crossterm::{event, terminal, ExecutableCommand};
use std::io::{stdout, Result};
use crate::finder::{search_files, SearchResult};
use crate::config::AppConfig;
use crate::preview::preview_file;
use crate::actions::open_file;

pub fn run_ui(config: AppConfig) -> Result<()> {
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(terminal::EnterAlternateScreen)?;
    stdout.execute(crossterm::cursor::Hide)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut input = String::new();
    let mut selected = 0;
    let mut results: Vec<SearchResult> = vec![];

    loop {
        results = search_files(&config, &input);

        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(5),
                    Constraint::Length(10),
                ])
                .split(size);

            let input_widget = Paragraph::new(input.clone())
                .style(Style::default().fg(Color::Yellow))
                .block(Block::default().borders(Borders::ALL).title("Search"));
            f.render_widget(input_widget, chunks[0]);

            let items: Vec<ListItem> = results.iter().map(|r| ListItem::new(r.path.clone())).collect();
            let mut state = ListState::default();
            state.select(Some(selected));

            let list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("Results"))
                .highlight_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
                .highlight_symbol("▶ ");

            f.render_stateful_widget(list, chunks[1], &mut state);

            let preview = results.get(selected).and_then(|res| preview_file(&res.path)).unwrap_or_else(|| "[No Preview Available]".into());

            let preview_widget = Paragraph::new(preview)
                .style(Style::default().fg(Color::Gray))
                .block(Block::default().borders(Borders::ALL).title("Preview"));

            f.render_widget(preview_widget, chunks[2]);
        })?;

        if crossterm::event::poll(std::time::Duration::from_millis(100))? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                match key.code {
                    crossterm::event::KeyCode::Char('q') => break,
                    crossterm::event::KeyCode::Enter => {
                        if let Some(result) = results.get(selected) {
                            open_file(&result.path);
                        }
                    },
                    crossterm::event::KeyCode::Char(c) => input.push(c),
                    crossterm::event::KeyCode::Backspace => { input.pop(); },
                    crossterm::event::KeyCode::Down => {
                        if selected + 1 < results.len() {
                            selected += 1;
                        }
                    },
                    crossterm::event::KeyCode::Up => {
                        if selected > 0 {
                            selected -= 1;
                        }
                    },
                    _ => {}
                }
            }
        }
    }

    terminal::disable_raw_mode()?;
    std::io::stdout().execute(terminal::LeaveAlternateScreen)?;
    std::io::stdout().execute(crossterm::cursor::Show)?;
    Ok(())
}