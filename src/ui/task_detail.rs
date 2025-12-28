use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};
use crate::app::App;

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let task = match app.get_editing_task() {
        Some(t) => t,
        None => {
            let error = Paragraph::new("No task selected")
                .block(Block::default().borders(Borders::ALL).title("Error"));
            f.render_widget(error, area);
            return;
        }
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),   // Title
            Constraint::Min(5),      // Description
            Constraint::Length(3),   // Tags
            Constraint::Min(4),      // File references
            Constraint::Length(3),   // Status bar
        ])
        .split(area);

    render_title_section(f, app, task, chunks[0]);
    render_description_section(f, app, task, chunks[1]);
    render_tags_section(f, app, task, chunks[2]);
    render_file_refs_section(f, app, task, chunks[3]);
    render_status_bar(f, app, chunks[4]);
}

fn render_title_section(f: &mut Frame, app: &App, task: &crate::models::Task, area: Rect) {
    let is_selected = app.detail_field_selection == 0;
    let style = if is_selected {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    let checkbox = if task.completed { "[✓]" } else { "[ ]" };
    let title_text = format!("{} {}", checkbox, task.title);

    let title = Paragraph::new(title_text)
        .style(style)
        .block(Block::default().borders(Borders::ALL).title("Title"));

    f.render_widget(title, area);
}

fn render_description_section(f: &mut Frame, app: &App, task: &crate::models::Task, area: Rect) {
    let is_selected = app.detail_field_selection == 1;
    let border_style = if is_selected {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    let desc_text = if task.description.is_empty() {
        "(No description)".to_string()
    } else {
        task.description.clone()
    };

    let description = Paragraph::new(desc_text)
        .wrap(Wrap { trim: false })
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Description")
            .border_style(border_style));

    f.render_widget(description, area);
}

fn render_tags_section(f: &mut Frame, app: &App, task: &crate::models::Task, area: Rect) {
    let is_selected = app.detail_field_selection == 2;
    let border_style = if is_selected {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    let tags_text = if task.tags.is_empty() {
        "(No tags - press 'a' to add)".to_string()
    } else {
        task.tags.iter()
            .map(|t| format!("[{}]", t))
            .collect::<Vec<_>>()
            .join(" ")
    };

    let tags = Paragraph::new(tags_text)
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Tags")
            .border_style(border_style));

    f.render_widget(tags, area);
}

fn render_file_refs_section(f: &mut Frame, app: &App, task: &crate::models::Task, area: Rect) {
    let is_selected = app.detail_field_selection == 3;
    let border_style = if is_selected {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    let items: Vec<ListItem> = if task.file_references.is_empty() {
        vec![ListItem::new("(No file references - press 'a' to add)")]
    } else {
        task.file_references.iter().map(|file_ref| {
            let line_part = file_ref.line_number
                .map(|l| format!(":{}", l))
                .unwrap_or_default();
            let desc_part = file_ref.description
                .as_ref()
                .map(|d| format!(" - {}", d))
                .unwrap_or_default();
            let text = format!("• {}{}{}", file_ref.path, line_part, desc_part);
            ListItem::new(text)
        }).collect()
    };

    let list = List::new(items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title("File References")
            .border_style(border_style));

    f.render_widget(list, area);
}


fn render_status_bar(f: &mut Frame, app: &App, area: Rect) {
    let help_text = "ESC/q back | j/k navigate | Tab next section | i edit | a add | d delete";

    let status = Paragraph::new(vec![
        Line::from(vec![
            Span::styled(
                " NORMAL ",
                Style::default().bg(Color::Blue).fg(Color::White),
            ),
            Span::raw(" "),
            Span::raw(help_text),
        ]),
    ])
    .block(Block::default().borders(Borders::ALL));

    f.render_widget(status, area);
}
