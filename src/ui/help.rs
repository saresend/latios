use crate::app::App;
use ratatui::{
    Frame,
    layout::Rect,
    text::Text,
    widgets::{Block, Borders, Paragraph},
};

pub fn render(f: &mut Frame, _app: &App, area: Rect) {
    let help_text = r#"
Latios - Productivity TUI

Key Bindings:

Task List View (Left Pane):
  j/k or Down/Up   Navigate tasks
  Space/Enter      Toggle task completion
  a                Add new task
  e                Edit task details
  c/y              Copy task to clipboard
  d                Delete task
  x                Export to markdown
  TAB              Switch to workstreams pane
  ?                Show this help
  q                Quit

Workstream List View (Right Pane):
  j/k or Down/Up   Navigate workstreams
  Enter            Launch selected workstream
  a                Add new workstream (opens preset picker)
  e                Edit workstream details
  d                Delete workstream
  TAB              Switch to tasks pane

Task Detail View:
  j/k/Tab          Navigate sections
  h/l or Left/Right Navigate within lists
  i/Enter          Edit selected field
  a                Add to list (tags/file refs/metadata)
  d                Delete selected item
  ESC/q            Back to task list

Insert Mode:
  ESC              Save/Cancel (context dependent)
  Enter            Confirm/Newline (context dependent)
  Left/Right       Move cursor
  Backspace        Delete character

Press any key to return...
    "#;

    let help = Paragraph::new(Text::raw(help_text))
        .block(Block::default().borders(Borders::ALL).title("Help"));
    f.render_widget(help, area);
}
