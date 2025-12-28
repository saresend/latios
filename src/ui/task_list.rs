use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use crate::app::{App, InputMode};

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(0),     // Task list
            Constraint::Length(3),  // Status bar
        ])
        .split(area);

    render_header(f, app, chunks[0]);
    render_task_list(f, app, chunks[1]);
    render_status_bar(f, app, chunks[2]);
}

fn render_header(f: &mut Frame, app: &App, area: Rect) {
    let title = if let Some(pid) = &app.current_project_id {
        if let Some(project) = app.data.projects.get(pid) {
            format!("Tasks - Project: {}", project.name)
        } else {
            "Tasks - All".to_string()
        }
    } else {
        "Tasks - All".to_string()
    };

    let header = Paragraph::new(title)
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(header, area);
}

fn render_task_list(f: &mut Frame, app: &App, area: Rect) {
    let mut tasks = app.data.get_tasks_by_project(app.current_project_id.as_deref());
    tasks.sort_by(|a, b| a.created_at.cmp(&b.created_at));

    if app.input_mode == InputMode::Insert {
        // Show input prompt
        let input_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(area);

        // Render existing tasks
        let items: Vec<ListItem> = tasks
            .iter()
            .enumerate()
            .map(|(i, task)| {
                let checkbox = if task.completed { "[✓]" } else { "[ ]" };
                let style = if i == app.selected_task_index {
                    Style::default().bg(Color::DarkGray)
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

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Tasks"));
        f.render_widget(list, input_area[0]);

        // Render input box
        let input = Paragraph::new(app.input_buffer.as_str())
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title("New Task"));
        f.render_widget(input, input_area[1]);

        // Show cursor
        f.set_cursor_position((input_area[1].x + app.cursor_position as u16 + 1, input_area[1].y + 1));
    } else {
        // Normal mode - just show tasks
        let items: Vec<ListItem> = tasks
            .iter()
            .enumerate()
            .map(|(i, task)| {
                let checkbox = if task.completed { "[✓]" } else { "[ ]" };
                let style = if i == app.selected_task_index {
                    Style::default()
                        .bg(Color::DarkGray)
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

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Tasks"));
        f.render_widget(list, area);
    }
}

fn render_status_bar(f: &mut Frame, app: &App, area: Rect) {
    let mode_text = match app.input_mode {
        InputMode::Normal => "NORMAL",
        InputMode::Insert => "INSERT",
    };

    let help_text = if app.input_mode == InputMode::Insert {
        "ESC cancel | ENTER confirm"
    } else {
        "? help | a add | e edit | c copy | d delete | x export | q quit"
    };

    let status = if let Some(msg) = &app.status_message {
        Paragraph::new(vec![
            Line::from(vec![
                Span::styled(
                    format!(" {} ", mode_text),
                    Style::default().bg(Color::Blue).fg(Color::White),
                ),
                Span::raw(" "),
                Span::styled(msg, Style::default().fg(Color::Green)),
            ]),
        ])
        .block(Block::default().borders(Borders::ALL))
    } else {
        Paragraph::new(vec![
            Line::from(vec![
                Span::styled(
                    format!(" {} ", mode_text),
                    Style::default().bg(Color::Blue).fg(Color::White),
                ),
                Span::raw(" "),
                Span::raw(help_text),
            ]),
        ])
        .block(Block::default().borders(Borders::ALL))
    };

    f.render_widget(status, area);
}
