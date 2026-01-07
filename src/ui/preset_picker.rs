use crate::app::App;
use crate::models::get_all_presets;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Preset list
            Constraint::Length(3), // Status bar
        ])
        .split(area);

    render_header(f, chunks[0]);
    render_preset_list(f, app, chunks[1]);
    render_status_bar(f, chunks[2]);
}

fn render_header(f: &mut Frame, area: Rect) {
    let header = Paragraph::new("Select a Workflow Preset")
        .style(
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(header, area);
}

fn render_preset_list(f: &mut Frame, app: &App, area: Rect) {
    let presets = get_all_presets();

    let items: Vec<ListItem> = presets
        .iter()
        .enumerate()
        .map(|(i, preset)| {
            let is_selected = i == app.view_state.selected_preset_index;
            let style = if is_selected {
                Style::default()
                    .bg(Color::Magenta)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            let content = Line::from(vec![
                Span::styled(preset.name, style),
                Span::styled(
                    format!(" - {}", preset.description),
                    Style::default().fg(Color::DarkGray),
                ),
            ]);

            ListItem::new(content).style(style)
        })
        .collect();

    let list = List::new(items).block(Block::default().borders(Borders::ALL).title("Presets"));

    f.render_widget(list, area);
}

fn render_status_bar(f: &mut Frame, area: Rect) {
    let status = Paragraph::new(vec![Line::from(vec![
        Span::styled(
            " SELECT ",
            Style::default().bg(Color::Blue).fg(Color::White),
        ),
        Span::raw(" j/k navigate | ENTER select | ESC cancel"),
    ])])
    .block(Block::default().borders(Borders::ALL));

    f.render_widget(status, area);
}
