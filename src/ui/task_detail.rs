use ratatui::{
    layout::Rect,
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::app::App;

pub fn render(f: &mut Frame, _app: &App, area: Rect) {
    let placeholder = Paragraph::new(Text::raw("Task Detail View - Coming Soon"))
        .block(Block::default().borders(Borders::ALL).title("Task Details"));
    f.render_widget(placeholder, area);
}
