use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};
use crate::app::App;
use crate::models::{DetailEditField, InputMode};

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
    let is_editing = app.view_state.detail_editing_field == Some(DetailEditField::Title);

    if is_editing {
        // Show input box when editing
        let input = Paragraph::new(app.view_state.input_buffer.as_str())
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title("Title (editing)"));
        f.render_widget(input, area);

        // Set cursor position
        if app.view_state.input_mode == InputMode::Insert {
            f.set_cursor_position((area.x + app.view_state.cursor_position as u16 + 1, area.y + 1));
        }
    } else {
        // Show read-only view
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
}

fn render_description_section(f: &mut Frame, app: &App, task: &crate::models::Task, area: Rect) {
    let is_selected = app.detail_field_selection == 1;
    let is_editing = app.view_state.detail_editing_field == Some(DetailEditField::Description);

    if is_editing {
        if let Some(edit_state) = &app.view_state.description_edit_state {
            let description = Paragraph::new(edit_state.text())
                .style(Style::default().fg(Color::Yellow))
                .wrap(Wrap { trim: false })
                .block(Block::default()
                    .borders(Borders::ALL)
                    .title("Description (editing - Enter for newline, ESC to save)"));

            f.render_widget(description, area);

            // Calculate cursor position
            if app.view_state.input_mode == InputMode::Insert {
                let (line, col) = edit_state.cursor_position();
                f.set_cursor_position((
                    area.x + col as u16 + 1,
                    area.y + line as u16 + 1,
                ));
            }
        }
    } else {
        // Show read-only view
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
}

fn render_tags_section(f: &mut Frame, app: &App, task: &crate::models::Task, area: Rect) {
    let is_selected = app.detail_field_selection == 2;
    let is_adding = app.view_state.detail_editing_field == Some(DetailEditField::AddingTag);

    if is_adding {
        // Show input prompt when adding a tag
        let prompt = format!("Add tag: {}", app.view_state.input_buffer);
        let input = Paragraph::new(prompt)
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title("Tags (adding)"));
        f.render_widget(input, area);

        // Set cursor position
        if app.view_state.input_mode == InputMode::Insert {
            f.set_cursor_position((area.x + app.view_state.cursor_position as u16 + 10, area.y + 1)); // +10 for "Add tag: "
        }
    } else {
        // Show as a list with selection
        let border_style = if is_selected {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };

        let items: Vec<ListItem> = if task.tags.is_empty() {
            vec![ListItem::new("(No tags - press 'a' to add)")]
        } else {
            task.tags.iter().enumerate().map(|(i, tag)| {
                let content = format!("[{}]", tag);
                let style = if is_selected && i == app.selected_tag_index {
                    Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                ListItem::new(content).style(style)
            }).collect()
        };

        let title = if is_selected && !task.tags.is_empty() {
            "Tags (h/l to navigate, d to delete, a to add)"
        } else {
            "Tags"
        };

        let tags = List::new(items)
            .block(Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(border_style));

        f.render_widget(tags, area);
    }
}

fn render_file_refs_section(f: &mut Frame, app: &App, task: &crate::models::Task, area: Rect) {
    let is_selected = app.detail_field_selection == 3;
    let is_adding = app.view_state.detail_editing_field == Some(DetailEditField::AddingFileRef);

    if is_adding {
        // Show multi-step input form
        let (prompt, hint) = match app.file_ref_input_step {
            0 => ("File path: ", "Enter the file path"),
            1 => ("Line number (optional): ", "Enter line number or leave blank"),
            2 => ("Description (optional): ", "Enter description or leave blank"),
            _ => ("", ""),
        };

        let input_text = format!("{}{}", prompt, app.view_state.input_buffer);
        let title = format!("File References (adding - step {}/3: {})", app.file_ref_input_step + 1, hint);

        let input = Paragraph::new(input_text)
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title(title));
        f.render_widget(input, area);

        // Set cursor position
        if app.view_state.input_mode == InputMode::Insert {
            f.set_cursor_position((area.x + app.view_state.cursor_position as u16 + prompt.len() as u16 + 1, area.y + 1));
        }
    } else {
        // Show as a list with selection
        let border_style = if is_selected {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };

        let items: Vec<ListItem> = if task.file_references.is_empty() {
            vec![ListItem::new("(No file references - press 'a' to add)")]
        } else {
            task.file_references.iter().enumerate().map(|(i, file_ref)| {
                let line_part = file_ref.line_number
                    .map(|l| format!(":{}", l))
                    .unwrap_or_default();
                let desc_part = file_ref.description
                    .as_ref()
                    .map(|d| format!(" - {}", d))
                    .unwrap_or_default();
                let text = format!("• {}{}{}", file_ref.path, line_part, desc_part);
                let style = if is_selected && i == app.selected_file_ref_index {
                    Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                ListItem::new(text).style(style)
            }).collect()
        };

        let title = if is_selected && !task.file_references.is_empty() {
            "File References (h/l to navigate, d to delete, a to add)"
        } else {
            "File References"
        };

        let list = List::new(items)
            .block(Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(border_style));

        f.render_widget(list, area);
    }
}


fn render_status_bar(f: &mut Frame, app: &App, area: Rect) {
    let (mode_text, help_text) = if app.view_state.input_mode == InputMode::Insert {
        // Context-specific help for insert mode
        match &app.view_state.detail_editing_field {
            Some(DetailEditField::Title) => {
                ("INSERT", "Enter save | ESC cancel")
            },
            Some(DetailEditField::Description) => {
                ("INSERT", "Enter newline | ESC save")
            },
            Some(DetailEditField::AddingTag) => {
                ("INSERT", "Enter save tag | ESC cancel")
            },
            Some(DetailEditField::AddingFileRef) => {
                ("INSERT", "Enter next step | ESC cancel")
            },
            Some(DetailEditField::ProjectName) => {
                ("INSERT", "Enter save | ESC cancel")
            },
            Some(DetailEditField::ProjectDescription) => {
                ("INSERT", "Enter newline | ESC save")
            },
            None => {
                ("INSERT", "Enter confirm | ESC cancel")
            }
        }
    } else {
        // Normal mode help
        ("NORMAL", "ESC/q back | j/k navigate | Tab next | i/Enter edit | a add")
    };

    let mode_color = if app.view_state.input_mode == InputMode::Insert {
        Color::Green
    } else {
        Color::Blue
    };

    let status = Paragraph::new(vec![
        Line::from(vec![
            Span::styled(
                format!(" {} ", mode_text),
                Style::default().bg(mode_color).fg(Color::White),
            ),
            Span::raw(" "),
            Span::raw(help_text),
        ]),
    ])
    .block(Block::default().borders(Borders::ALL));

    f.render_widget(status, area);
}
