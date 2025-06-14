
use ratatui::{prelude::*, widgets::*};
use crossterm::{ terminal, ExecutableCommand};
use std::io::{stdout, Result};
use crate::finder::SearchResult;
use crate::config::AppConfig;

pub fn run_ui(results: Vec<SearchResult>, _config: &AppConfig) -> Result<()> {
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(terminal::EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| {
            let size = f.size();

            let block = Block::default().borders(Borders::ALL).title("WarpFind");
            f.render_widget(block, size);

            let items: Vec<ListItem> = results.iter().map(|r| ListItem::new(r.path.clone())).collect();

            let list = List::new(items).block(Block::default().title("Results").borders(Borders::ALL));
            f.render_widget(list, size);
        })?;

        if crossterm::event::poll(std::time::Duration::from_millis(100))? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                if key.code == crossterm::event::KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    terminal::disable_raw_mode()?;
    std::io::stdout().execute(terminal::LeaveAlternateScreen)?;
    Ok(())
}
