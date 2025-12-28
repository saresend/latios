use ratatui::{
    layout::Rect,
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::app::App;

pub fn render(f: &mut Frame, _app: &App, area: Rect) {
    let help_text = r#"
Latios - Productivity TUI

Key Bindings:

Normal Mode (Task List):
  j/k or ↓/↑    Navigate tasks
  Space/Enter   Toggle task completion
  a             Add new task
  e             Edit task details
  d             Delete task
  p             Switch projects
  x             Export to markdown
  ?             Show this help
  q             Quit

Insert Mode:
  ESC           Cancel input
  Enter         Confirm input

Press any key to return...
    "#;

    let help = Paragraph::new(Text::raw(help_text))
        .block(Block::default().borders(Borders::ALL).title("Help"));
    f.render_widget(help, area);
}
