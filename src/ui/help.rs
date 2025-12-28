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
  c/y           Copy task to clipboard
  d             Delete task
  x             Export to markdown
  ?             Show this help
  q             Quit

Task Detail View:
  j/k/Tab       Navigate sections
  h/l or ←/→    Navigate within lists (tags/file refs)
  i/Enter       Edit selected field
  a             Add to list (tags/file refs)
  d             Delete selected item (tags/file refs)
  ESC/q         Back to task list

Insert Mode:
  ESC           Save/Cancel (context dependent)
  Enter         Confirm/Newline (context dependent)
  Left/Right    Move cursor
  Backspace     Delete character

Press any key to return...
    "#;

    let help = Paragraph::new(Text::raw(help_text))
        .block(Block::default().borders(Borders::ALL).title("Help"));
    f.render_widget(help, area);
}
