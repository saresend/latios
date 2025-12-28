use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use crate::app::{App, FocusPane, InputMode};

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    // Main horizontal split: tasks (60%) and projects (40%)
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(60),  // Tasks pane
            Constraint::Percentage(40),  // Projects pane
        ])
        .split(area);

    render_tasks_pane(f, app, main_chunks[0]);
    render_projects_pane(f, app, main_chunks[1]);
}

fn render_tasks_pane(f: &mut Frame, app: &App, area: Rect) {
    // Vertical layout for tasks pane
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(0),     // Task list
            Constraint::Length(3),  // Status bar
        ])
        .split(area);

    render_task_header(f, app, chunks[0]);
    render_task_list(f, app, chunks[1]);
    render_task_status_bar(f, app, chunks[2]);
}

fn render_projects_pane(f: &mut Frame, app: &App, area: Rect) {
    // Vertical layout for projects pane
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(0),     // Project list
            Constraint::Length(3),  // Status bar
        ])
        .split(area);

    render_project_header(f, app, chunks[0]);
    render_project_list(f, app, chunks[1]);
    render_project_status_bar(f, app, chunks[2]);
}

fn render_task_header(f: &mut Frame, app: &App, area: Rect) {
    let is_focused = app.focused_pane == FocusPane::Tasks;
    let title = if let Some(pid) = &app.current_project_id {
        if let Some(project) = app.data.projects.get(pid) {
            format!("Tasks - Project: {}", project.name)
        } else {
            "Tasks - All".to_string()
        }
    } else {
        "Tasks - All".to_string()
    };

    let border_color = if is_focused { Color::Cyan } else { Color::White };
    let header = Paragraph::new(title)
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(border_color)));

    f.render_widget(header, area);
}

fn render_task_list(f: &mut Frame, app: &App, area: Rect) {
    let mut tasks = app.data.get_tasks_by_project(app.current_project_id.as_deref());
    tasks.sort_by(|a, b| a.created_at.cmp(&b.created_at));
    let is_focused = app.focused_pane == FocusPane::Tasks;

    if app.input_mode == InputMode::Insert && is_focused {
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

fn render_task_status_bar(f: &mut Frame, app: &App, area: Rect) {
    let is_focused = app.focused_pane == FocusPane::Tasks;
    let mode_text = match app.input_mode {
        InputMode::Normal => "NORMAL",
        InputMode::Insert => "INSERT",
    };

    let help_text = if app.input_mode == InputMode::Insert && is_focused {
        "ESC cancel | ENTER confirm"
    } else if is_focused {
        "TAB switch | a add | e edit | d delete | q quit"
    } else {
        "TAB switch to tasks"
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

fn render_project_header(f: &mut Frame, app: &App, area: Rect) {
    let is_focused = app.focused_pane == FocusPane::Projects;
    let border_color = if is_focused { Color::Cyan } else { Color::White };

    let header = Paragraph::new("Projects")
        .style(Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(border_color)));

    f.render_widget(header, area);
}

fn render_project_list(f: &mut Frame, app: &App, area: Rect) {
    let projects = app.data.get_projects_sorted();
    let is_focused = app.focused_pane == FocusPane::Projects;

    if app.input_mode == InputMode::Insert && is_focused {
        // Show input prompt for adding new project
        let input_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(area);

        // Render existing projects
        let items: Vec<ListItem> = projects
            .iter()
            .enumerate()
            .map(|(i, project)| {
                let style = if i == app.selected_project_index_in_pane {
                    Style::default().bg(Color::DarkGray)
                } else {
                    Style::default()
                };
                let content = format!("• {}", project.name);
                ListItem::new(content).style(style)
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Projects"));
        f.render_widget(list, input_area[0]);

        // Render input box
        let input = Paragraph::new(app.input_buffer.as_str())
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title("New Project"));
        f.render_widget(input, input_area[1]);

        // Show cursor
        f.set_cursor_position((input_area[1].x + app.cursor_position as u16 + 1, input_area[1].y + 1));
    } else {
        // Normal mode - just show projects
        let items: Vec<ListItem> = if projects.is_empty() {
            vec![ListItem::new("(No projects - press 'a' to add)").style(Style::default().fg(Color::DarkGray))]
        } else {
            projects
                .iter()
                .enumerate()
                .map(|(i, project)| {
                    let style = if i == app.selected_project_index_in_pane {
                        Style::default()
                            .bg(Color::DarkGray)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                    };
                    let content = format!("• {}", project.name);
                    ListItem::new(content).style(style)
                })
                .collect()
        };

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Projects"));
        f.render_widget(list, area);
    }
}

fn render_project_status_bar(f: &mut Frame, app: &App, area: Rect) {
    let is_focused = app.focused_pane == FocusPane::Projects;
    let mode_text = match app.input_mode {
        InputMode::Normal => "NORMAL",
        InputMode::Insert => "INSERT",
    };

    let help_text = if app.input_mode == InputMode::Insert && is_focused {
        "ESC cancel | ENTER confirm"
    } else if is_focused {
        "TAB switch | a add | e edit | d delete"
    } else {
        "TAB switch to projects"
    };

    let status = Paragraph::new(vec![
        Line::from(vec![
            Span::styled(
                format!(" {} ", mode_text),
                Style::default().bg(Color::Blue).fg(Color::White),
            ),
            Span::raw(" "),
            Span::raw(help_text),
        ]),
    ])
    .block(Block::default().borders(Borders::ALL));

    f.render_widget(status, area);
}
