use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};
use crate::app::App;
use crate::models::{DetailEditField, InputMode};
use crate::markdown;

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let project = match app.get_editing_project() {
        Some(p) => p,
        None => {
            let error = Paragraph::new("No project selected")
                .block(Block::default().borders(Borders::ALL).title("Error"));
            f.render_widget(error, area);
            return;
        }
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),   // Name
            Constraint::Min(5),      // Description
            Constraint::Length(3),   // Metadata
            Constraint::Length(3),   // Status bar
        ])
        .split(area);

    render_name_section(f, app, project, chunks[0]);
    render_description_section(f, app, project, chunks[1]);
    render_metadata_section(f, project, chunks[2]);
    render_status_bar(f, app, chunks[3]);
}

fn render_name_section(f: &mut Frame, app: &App, project: &crate::models::Project, area: Rect) {
    let is_selected = app.detail_field_selection == 0;
    let is_editing = app.view_state.detail_editing_field == Some(DetailEditField::ProjectName);

    if is_editing {
        // Show input box when editing
        let input = Paragraph::new(app.view_state.input_buffer.as_str())
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title("Name (editing)"));
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

        let name = Paragraph::new(project.name.clone())
            .style(style)
            .block(Block::default().borders(Borders::ALL).title("Name"));

        f.render_widget(name, area);
    }
}

fn render_description_section(f: &mut Frame, app: &App, project: &crate::models::Project, area: Rect) {
    let is_selected = app.detail_field_selection == 1;
    let is_editing = app.view_state.detail_editing_field == Some(DetailEditField::ProjectDescription);

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

        let desc_lines = if project.description.is_empty() {
            vec![Line::from("(No description)").style(Style::default().fg(Color::DarkGray))]
        } else {
            let lines = markdown::parse_markdown(&project.description);
            if lines.is_empty() {
                // Fallback to plain text if markdown parsing produces no output
                vec![Line::from(project.description.clone())]
            } else {
                lines
            }
        };

        let description = Paragraph::new(desc_lines)
            .wrap(Wrap { trim: false })
            .block(Block::default()
                .borders(Borders::ALL)
                .title("Description")
                .border_style(border_style));

        f.render_widget(description, area);
    }
}

fn render_metadata_section(f: &mut Frame, project: &crate::models::Project, area: Rect) {
    let text = format!("Created: {} | Updated: {}", project.created_at, project.updated_at);
    let metadata = Paragraph::new(text)
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default().borders(Borders::ALL).title("Metadata"));

    f.render_widget(metadata, area);
}

fn render_status_bar(f: &mut Frame, app: &App, area: Rect) {
    let (mode_text, help_text) = if app.view_state.input_mode == InputMode::Insert {
        // Context-specific help for insert mode
        match &app.view_state.detail_editing_field {
            Some(DetailEditField::ProjectName) => {
                ("INSERT", "Enter save | ESC cancel")
            },
            Some(DetailEditField::ProjectDescription) => {
                ("INSERT", "Enter newline | ESC save")
            },
            _ => {
                ("INSERT", "Enter save | ESC cancel")
            }
        }
    } else {
        // Normal mode help
        ("NORMAL", "ESC/q back | j/k navigate | Tab next | i/Enter edit")
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
