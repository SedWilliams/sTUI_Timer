use ratatui::style::{Color, Modifier};
use ratatui_core::style::Style;

pub const BASIC_TEXT_STYLE: Style = Style::new()
    .fg(Color::White)
    .bg(Color::Black)
    .add_modifier(Modifier::BOLD);
