use crate::app::App;
use crate::models::{DetailEditField, InputMode, WorkstreamState, get_preset_by_id};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let workstream = match app.get_editing_workstream() {
        Some(w) => w,
        None => {
            let error = Paragraph::new("No workstream selected")
                .block(Block::default().borders(Borders::ALL).title("Error"));
            f.render_widget(error, area);
            return;
        }
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Name
            Constraint::Length(3), // Preset
            Constraint::Length(3), // State
            Constraint::Length(3), // Metadata (created, updated, last accessed)
            Constraint::Min(1),    // Spacer
            Constraint::Length(3), // Status bar
        ])
        .split(area);

    render_name_section(f, app, workstream, chunks[0]);
    render_preset_section(f, workstream, chunks[1]);
    render_state_section(f, workstream, chunks[2]);
    render_metadata_section(f, workstream, chunks[3]);
    render_status_bar(f, app, chunks[5]);
}

fn render_name_section(
    f: &mut Frame,
    app: &App,
    workstream: &crate::models::Workstream,
    area: Rect,
) {
    let is_selected = app.detail_field_selection == 0;
    let is_editing = app.view_state.detail_editing_field == Some(DetailEditField::WorkstreamName);

    if is_editing {
        // Show input box when editing
        let input = Paragraph::new(app.view_state.input_buffer.as_str())
            .style(Style::default().fg(Color::Yellow))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Name (editing)"),
            );
        f.render_widget(input, area);

        // Set cursor position
        if app.view_state.input_mode == InputMode::Insert {
            f.set_cursor_position((
                area.x + app.view_state.cursor_position as u16 + 1,
                area.y + 1,
            ));
        }
    } else {
        // Show read-only view
        let style = if is_selected {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };

        let name = Paragraph::new(workstream.name.clone())
            .style(style)
            .block(Block::default().borders(Borders::ALL).title("Name"));

        f.render_widget(name, area);
    }
}

fn render_preset_section(f: &mut Frame, workstream: &crate::models::Workstream, area: Rect) {
    let preset_info = get_preset_by_id(&workstream.preset_id)
        .map(|p| format!("{} - {}", p.name, p.description))
        .unwrap_or_else(|| format!("Unknown preset: {}", workstream.preset_id));

    let preset = Paragraph::new(preset_info)
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL).title("Preset"));

    f.render_widget(preset, area);
}

fn render_state_section(f: &mut Frame, workstream: &crate::models::Workstream, area: Rect) {
    let (state_text, state_color) = match workstream.state {
        WorkstreamState::Idle => ("Idle", Color::DarkGray),
        WorkstreamState::Running => ("Running", Color::Green),
        WorkstreamState::NeedsInput => ("Needs Input", Color::Yellow),
    };

    let state = Paragraph::new(state_text)
        .style(
            Style::default()
                .fg(state_color)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().borders(Borders::ALL).title("State"));

    f.render_widget(state, area);
}

fn render_metadata_section(f: &mut Frame, workstream: &crate::models::Workstream, area: Rect) {
    let last_accessed = workstream
        .last_accessed
        .as_ref()
        .map(|s| s.as_str())
        .unwrap_or("Never");

    let text = format!(
        "Created: {} | Updated: {} | Last Accessed: {}",
        workstream.created_at, workstream.updated_at, last_accessed
    );

    let metadata = Paragraph::new(text)
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default().borders(Borders::ALL).title("Metadata"));

    f.render_widget(metadata, area);
}

fn render_status_bar(f: &mut Frame, app: &App, area: Rect) {
    let (mode_text, help_text) = if app.view_state.input_mode == InputMode::Insert {
        match &app.view_state.detail_editing_field {
            Some(DetailEditField::WorkstreamName) => ("INSERT", "Enter save | ESC cancel"),
            _ => ("INSERT", "Enter save | ESC cancel"),
        }
    } else {
        ("NORMAL", "ESC/q back | i/Enter edit name | ENTER launch")
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
