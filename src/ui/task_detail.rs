use crate::app::App;
use crate::models::{DetailEditField, InputMode};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
};

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
            Constraint::Length(3), // Title
            Constraint::Min(5),    // Description
            Constraint::Length(3), // Tags
            Constraint::Length(4), // File references
            Constraint::Length(3), // Metadata
            Constraint::Length(3), // Workstreams
            Constraint::Length(3), // Status bar
        ])
        .split(area);

    render_title_section(f, app, task, chunks[0]);
    render_description_section(f, app, task, chunks[1]);
    render_tags_section(f, app, task, chunks[2]);
    render_file_refs_section(f, app, task, chunks[3]);
    render_metadata_section(f, app, task, chunks[4]);
    render_workstreams_section(f, app, task, chunks[5]);
    render_status_bar(f, app, chunks[6]);
}

fn render_title_section(f: &mut Frame, app: &App, task: &crate::models::Task, area: Rect) {
    let is_selected = app.detail_field_selection == 0;
    let is_editing = app.view_state.detail_editing_field == Some(DetailEditField::Title);

    if is_editing {
        let input = Paragraph::new(app.view_state.input_buffer.as_str())
            .style(Style::default().fg(Color::Yellow))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Title (editing)"),
            );
        f.render_widget(input, area);

        if app.view_state.input_mode == InputMode::Insert {
            f.set_cursor_position((
                area.x + app.view_state.cursor_position as u16 + 1,
                area.y + 1,
            ));
        }
    } else {
        let style = if is_selected {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };

        let checkbox = if task.completed { "[x]" } else { "[ ]" };
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
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Description (editing - Enter for newline, ESC to save)"),
                );

            f.render_widget(description, area);

            if app.view_state.input_mode == InputMode::Insert {
                let (line, col) = edit_state.cursor_position();
                f.set_cursor_position((area.x + col as u16 + 1, area.y + line as u16 + 1));
            }
        }
    } else {
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

        let description = Paragraph::new(desc_text).wrap(Wrap { trim: false }).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Description")
                .border_style(border_style),
        );

        f.render_widget(description, area);
    }
}

fn render_tags_section(f: &mut Frame, app: &App, task: &crate::models::Task, area: Rect) {
    let is_selected = app.detail_field_selection == 2;
    let is_adding = app.view_state.detail_editing_field == Some(DetailEditField::AddingTag);

    if is_adding {
        let prompt = format!("Add tag: {}", app.view_state.input_buffer);
        let input = Paragraph::new(prompt)
            .style(Style::default().fg(Color::Yellow))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Tags (adding)"),
            );
        f.render_widget(input, area);

        if app.view_state.input_mode == InputMode::Insert {
            f.set_cursor_position((
                area.x + app.view_state.cursor_position as u16 + 10,
                area.y + 1,
            ));
        }
    } else {
        let border_style = if is_selected {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };

        let items: Vec<ListItem> = if task.tags.is_empty() {
            vec![ListItem::new("(No tags - press 'a' to add)")]
        } else {
            task.tags
                .iter()
                .enumerate()
                .map(|(i, tag)| {
                    let content = format!("[{}]", tag);
                    let style = if is_selected && i == app.selected_tag_index {
                        Style::default()
                            .bg(Color::DarkGray)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                    };
                    ListItem::new(content).style(style)
                })
                .collect()
        };

        let title = if is_selected && !task.tags.is_empty() {
            "Tags (j/k navigate, d delete, a add)"
        } else {
            "Tags"
        };

        let tags = List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(border_style),
        );

        f.render_widget(tags, area);
    }
}

fn render_file_refs_section(f: &mut Frame, app: &App, task: &crate::models::Task, area: Rect) {
    let is_selected = app.detail_field_selection == 3;
    let is_adding = app.view_state.detail_editing_field == Some(DetailEditField::AddingFileRef);

    if is_adding {
        let (prompt, hint) = match app.file_ref_input_step {
            0 => ("File path: ", "Enter the file path"),
            1 => (
                "Line number (optional): ",
                "Enter line number or leave blank",
            ),
            2 => (
                "Description (optional): ",
                "Enter description or leave blank",
            ),
            _ => ("", ""),
        };

        let input_text = format!("{}{}", prompt, app.view_state.input_buffer);
        let title = format!(
            "File References (adding - step {}/3: {})",
            app.file_ref_input_step + 1,
            hint
        );

        let input = Paragraph::new(input_text)
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title(title));
        f.render_widget(input, area);

        if app.view_state.input_mode == InputMode::Insert {
            f.set_cursor_position((
                area.x + app.view_state.cursor_position as u16 + prompt.len() as u16 + 1,
                area.y + 1,
            ));
        }
    } else {
        let border_style = if is_selected {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };

        let items: Vec<ListItem> = if task.file_references.is_empty() {
            vec![ListItem::new("(No file references - press 'a' to add)")]
        } else {
            task.file_references
                .iter()
                .enumerate()
                .map(|(i, file_ref)| {
                    let line_part = file_ref
                        .line_number
                        .map(|l| format!(":{}", l))
                        .unwrap_or_default();
                    let desc_part = file_ref
                        .description
                        .as_ref()
                        .map(|d| format!(" - {}", d))
                        .unwrap_or_default();
                    let text = format!("{}{}{}", file_ref.path, line_part, desc_part);
                    let style = if is_selected && i == app.selected_file_ref_index {
                        Style::default()
                            .bg(Color::DarkGray)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                    };
                    ListItem::new(text).style(style)
                })
                .collect()
        };

        let title = if is_selected && !task.file_references.is_empty() {
            "File References (j/k navigate, d delete, a add)"
        } else {
            "File References"
        };

        let list = List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(border_style),
        );

        f.render_widget(list, area);
    }
}

fn render_metadata_section(f: &mut Frame, app: &App, task: &crate::models::Task, area: Rect) {
    let is_selected = app.detail_field_selection == 4;
    let is_adding = app.view_state.detail_editing_field == Some(DetailEditField::AddingMetadata);

    if is_adding {
        let (prompt, hint) = match app.metadata_input_step {
            0 => ("Key: ", "Enter metadata key"),
            1 => ("Value: ", "Enter metadata value"),
            _ => ("", ""),
        };

        let input_text = format!("{}{}", prompt, app.view_state.input_buffer);
        let title = format!(
            "Metadata (adding - step {}/2: {})",
            app.metadata_input_step + 1,
            hint
        );

        let input = Paragraph::new(input_text)
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title(title));
        f.render_widget(input, area);

        if app.view_state.input_mode == InputMode::Insert {
            f.set_cursor_position((
                area.x + app.view_state.cursor_position as u16 + prompt.len() as u16 + 1,
                area.y + 1,
            ));
        }
    } else {
        let border_style = if is_selected {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };

        let items: Vec<ListItem> = if task.metadata.is_empty() {
            vec![ListItem::new("(No metadata - press 'a' to add)")]
        } else {
            task.metadata
                .iter()
                .enumerate()
                .map(|(i, (k, v))| {
                    let text = format!("{}: {}", k, v);
                    let style = if is_selected && i == app.selected_metadata_index {
                        Style::default()
                            .bg(Color::DarkGray)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                    };
                    ListItem::new(text).style(style)
                })
                .collect()
        };

        let title = if is_selected && !task.metadata.is_empty() {
            "Metadata (j/k navigate, d delete, a add)"
        } else {
            "Metadata"
        };

        let list = List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(border_style),
        );

        f.render_widget(list, area);
    }
}

fn render_workstreams_section(f: &mut Frame, app: &App, task: &crate::models::Task, area: Rect) {
    let is_selected = app.detail_field_selection == 5;
    let border_style = if is_selected {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    let items: Vec<ListItem> = if task.workstream_ids.is_empty() {
        vec![ListItem::new("(No linked workstreams)")]
    } else {
        task.workstream_ids
            .iter()
            .enumerate()
            .map(|(i, ws_id)| {
                let ws_name = app
                    .data
                    .get_workstream(ws_id)
                    .map(|w| w.name.as_str())
                    .unwrap_or("Unknown");
                let style = if is_selected && i == app.selected_workstream_link_index {
                    Style::default()
                        .bg(Color::DarkGray)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                ListItem::new(format!("[{}]", ws_name)).style(style)
            })
            .collect()
    };

    let title = if is_selected && !task.workstream_ids.is_empty() {
        "Workstreams (j/k navigate, d unlink)"
    } else {
        "Workstreams"
    };

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(title)
            .border_style(border_style),
    );

    f.render_widget(list, area);
}

fn render_status_bar(f: &mut Frame, app: &App, area: Rect) {
    let (mode_text, help_text) = if app.view_state.input_mode == InputMode::Insert {
        match &app.view_state.detail_editing_field {
            Some(DetailEditField::Title) => ("INSERT", "Enter save | ESC cancel"),
            Some(DetailEditField::Description) => ("INSERT", "Enter newline | ESC save"),
            Some(DetailEditField::AddingTag) => ("INSERT", "Enter save tag | ESC cancel"),
            Some(DetailEditField::AddingFileRef) => ("INSERT", "Enter next step | ESC cancel"),
            Some(DetailEditField::AddingMetadata) => ("INSERT", "Enter next step | ESC cancel"),
            Some(DetailEditField::WorkstreamName) => ("INSERT", "Enter save | ESC cancel"),
            None => ("INSERT", "Enter confirm | ESC cancel"),
        }
    } else {
        (
            "NORMAL",
            "ESC/q back | j/k navigate | Tab next | i/Enter edit | a add",
        )
    };

    let mode_color = if app.view_state.input_mode == InputMode::Insert {
        Color::Green
    } else {
        Color::Blue
    };

    let status = Paragraph::new(vec![Line::from(vec![
        Span::styled(
            format!(" {} ", mode_text),
            Style::default().bg(mode_color).fg(Color::White),
        ),
        Span::raw(" "),
        Span::raw(help_text),
    ])])
    .block(Block::default().borders(Borders::ALL));

    f.render_widget(status, area);
}
