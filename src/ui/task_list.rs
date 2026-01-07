use crate::app::App;
use crate::models::{FocusPane, InputMode, WorkstreamState, get_preset_by_id};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    // Main horizontal split: tasks (60%) and workstreams (40%)
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(area);

    render_tasks_pane(f, app, main_chunks[0]);
    render_workstreams_pane(f, app, main_chunks[1]);
}

fn render_tasks_pane(f: &mut Frame, app: &App, area: Rect) {
    // Vertical layout for tasks pane
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Task list
            Constraint::Length(3), // Status bar
        ])
        .split(area);

    render_task_header(f, app, chunks[0]);
    render_task_list(f, app, chunks[1]);
    render_task_status_bar(f, app, chunks[2]);
}

fn render_workstreams_pane(f: &mut Frame, app: &App, area: Rect) {
    // Vertical layout for workstreams pane
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Workstream list
            Constraint::Length(3), // Status bar
        ])
        .split(area);

    render_workstream_header(f, app, chunks[0]);
    render_workstream_list(f, app, chunks[1]);
    render_workstream_status_bar(f, app, chunks[2]);
}

fn render_task_header(f: &mut Frame, app: &App, area: Rect) {
    let is_focused = app.view_state.focused_pane == FocusPane::Tasks;
    let border_color = if is_focused {
        Color::Cyan
    } else {
        Color::White
    };

    let task_count = app.data.tasks.len();
    let title_spans = vec![
        Span::styled(
            "Tasks ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("({})", task_count),
            Style::default().fg(Color::DarkGray),
        ),
    ];

    let header = Paragraph::new(Line::from(title_spans)).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color)),
    );

    f.render_widget(header, area);
}

fn render_task_list(f: &mut Frame, app: &App, area: Rect) {
    let tasks = app.data.get_all_tasks();
    let is_focused = app.view_state.focused_pane == FocusPane::Tasks;

    if app.view_state.input_mode == InputMode::Insert && is_focused {
        // Show input prompt
        let input_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)])
            .split(area);

        // Render existing tasks
        let items: Vec<ListItem> = tasks
            .iter()
            .enumerate()
            .map(|(i, task)| {
                let checkbox = if task.completed { "[x]" } else { "[ ]" };
                let style = if i == app.selected_task_index {
                    Style::default().bg(Color::Cyan).fg(Color::Black)
                } else if task.completed {
                    Style::default()
                        .fg(Color::DarkGray)
                        .add_modifier(Modifier::CROSSED_OUT)
                } else {
                    Style::default()
                };

                let content = format!("{} {}", checkbox, task.title);
                ListItem::new(content).style(style)
            })
            .collect();

        let border_color = if is_focused {
            Color::Cyan
        } else {
            Color::White
        };
        let list = List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Tasks")
                .border_style(Style::default().fg(border_color)),
        );
        f.render_widget(list, input_area[0]);

        // Render input box
        let input = Paragraph::new(app.view_state.input_buffer.as_str())
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title("New Task"));
        f.render_widget(input, input_area[1]);

        // Show cursor
        f.set_cursor_position((
            input_area[1].x + app.view_state.cursor_position as u16 + 1,
            input_area[1].y + 1,
        ));
    } else {
        // Normal mode - just show tasks
        let items: Vec<ListItem> = tasks
            .iter()
            .enumerate()
            .map(|(i, task)| {
                let checkbox = if task.completed { "[x]" } else { "[ ]" };
                let style = if i == app.selected_task_index && is_focused {
                    Style::default()
                        .bg(Color::Cyan)
                        .fg(Color::Black)
                        .add_modifier(Modifier::BOLD)
                } else if task.completed {
                    Style::default()
                        .fg(Color::DarkGray)
                        .add_modifier(Modifier::CROSSED_OUT)
                } else {
                    Style::default()
                };

                let content = format!("{} {}", checkbox, task.title);
                ListItem::new(content).style(style)
            })
            .collect();

        let border_color = if is_focused {
            Color::Cyan
        } else {
            Color::White
        };
        let list = List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Tasks")
                .border_style(Style::default().fg(border_color)),
        );
        f.render_widget(list, area);
    }
}

fn render_task_status_bar(f: &mut Frame, app: &App, area: Rect) {
    let is_focused = app.view_state.focused_pane == FocusPane::Tasks;
    let mode_text = match app.view_state.input_mode {
        InputMode::Normal => "NORMAL",
        InputMode::Insert => "INSERT",
    };

    let help_text = if app.view_state.input_mode == InputMode::Insert && is_focused {
        "ESC cancel | ENTER confirm"
    } else if is_focused {
        "TAB switch | a add | e edit | d delete | q quit"
    } else {
        "TAB switch to tasks"
    };

    // Sync status indicator
    let sync_indicator = if app.sync_enabled {
        Span::styled(" [SYNC] ", Style::default().fg(Color::Green))
    } else {
        Span::styled(" [LOCAL] ", Style::default().fg(Color::DarkGray))
    };

    let status = if let Some(msg) = &app.status_message {
        Paragraph::new(vec![Line::from(vec![
            Span::styled(
                format!(" {} ", mode_text),
                Style::default().bg(Color::Blue).fg(Color::White),
            ),
            sync_indicator,
            Span::styled(msg, Style::default().fg(Color::Green)),
        ])])
        .block(Block::default().borders(Borders::ALL))
    } else {
        Paragraph::new(vec![Line::from(vec![
            Span::styled(
                format!(" {} ", mode_text),
                Style::default().bg(Color::Blue).fg(Color::White),
            ),
            sync_indicator,
            Span::raw(help_text),
        ])])
        .block(Block::default().borders(Borders::ALL))
    };

    f.render_widget(status, area);
}

fn render_workstream_header(f: &mut Frame, app: &App, area: Rect) {
    let is_focused = app.view_state.focused_pane == FocusPane::Workstreams;
    let border_color = if is_focused {
        Color::Magenta
    } else {
        Color::White
    };

    let workstream_count = app.data.workstreams.len();
    let title_spans = vec![
        Span::styled(
            "Workstreams ",
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("({})", workstream_count),
            Style::default().fg(Color::DarkGray),
        ),
    ];

    let header = Paragraph::new(Line::from(title_spans)).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color)),
    );

    f.render_widget(header, area);
}

fn render_workstream_list(f: &mut Frame, app: &App, area: Rect) {
    let workstreams = app.data.get_workstreams_sorted();
    let is_focused = app.view_state.focused_pane == FocusPane::Workstreams;

    let items: Vec<ListItem> = workstreams
        .iter()
        .enumerate()
        .map(|(i, ws)| {
            // State indicator
            let state_indicator = match ws.state {
                WorkstreamState::Idle => Span::styled("[ ]", Style::default().fg(Color::DarkGray)),
                WorkstreamState::Running => Span::styled("[>]", Style::default().fg(Color::Green)),
                WorkstreamState::NeedsInput => {
                    Span::styled("[?]", Style::default().fg(Color::Yellow))
                }
            };

            // Get preset name
            let preset_name = get_preset_by_id(&ws.preset_id)
                .map(|p| p.name)
                .unwrap_or("Unknown");

            let is_selected = i == app.selected_workstream_index;
            let base_style = if is_selected && is_focused {
                Style::default()
                    .bg(Color::Magenta)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            let content = Line::from(vec![
                state_indicator,
                Span::raw(" "),
                Span::styled(&ws.name, base_style),
                Span::styled(
                    format!(" ({})", preset_name),
                    Style::default().fg(Color::DarkGray),
                ),
            ]);

            ListItem::new(content).style(base_style)
        })
        .collect();

    let border_color = if is_focused {
        Color::Magenta
    } else {
        Color::White
    };
    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Workstreams")
            .border_style(Style::default().fg(border_color)),
    );
    f.render_widget(list, area);
}

fn render_workstream_status_bar(f: &mut Frame, app: &App, area: Rect) {
    let is_focused = app.view_state.focused_pane == FocusPane::Workstreams;
    let mode_text = match app.view_state.input_mode {
        InputMode::Normal => "NORMAL",
        InputMode::Insert => "INSERT",
    };

    let help_text = if is_focused {
        "ENTER launch | a add | e edit | d delete"
    } else {
        "TAB switch to workstreams"
    };

    let status = Paragraph::new(vec![Line::from(vec![
        Span::styled(
            format!(" {} ", mode_text),
            Style::default().bg(Color::Blue).fg(Color::White),
        ),
        Span::raw(" "),
        Span::raw(help_text),
    ])])
    .block(Block::default().borders(Borders::ALL));

    f.render_widget(status, area);
}
