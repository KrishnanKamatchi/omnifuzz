use ratatui::style::{Color, Modifier, Style};

pub fn highlight_style() -> Style {
    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
}