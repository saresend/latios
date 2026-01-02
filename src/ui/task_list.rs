use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use crate::app::App;
use crate::models::{FocusPane, InputMode};

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    // Main horizontal split: projects (60%) and tasks (40%)
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(70),
        ])
        .split(area);

    render_projects_pane(f, app, main_chunks[0]);
    render_tasks_pane(f, app, main_chunks[1]);
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
    let is_focused = app.view_state.focused_pane == FocusPane::Tasks;
    let border_color = if is_focused { Color::Cyan } else { Color::White };

    // Build title with optional filter badge
    let title_spans = if let Some(pid) = &app.current_project_id {
        if let Some(project) = app.data.projects.get(pid) {
            vec![
                Span::styled("Tasks ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::raw("["),
                Span::styled(&project.name, Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::raw("]"),
            ]
        } else {
            vec![Span::styled("Tasks - All", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))]
        }
    } else {
        vec![Span::styled("Tasks - All", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))]
    };

    let header = Paragraph::new(Line::from(title_spans))
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(border_color)));

    f.render_widget(header, area);
}

fn render_task_list(f: &mut Frame, app: &App, area: Rect) {
    let mut tasks = app.data.get_tasks_by_project(app.current_project_id.as_deref());
    tasks.sort_by(|a, b| a.created_at.cmp(&b.created_at));
    let is_focused = app.view_state.focused_pane == FocusPane::Tasks;

    if app.view_state.input_mode == InputMode::Insert && is_focused {
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

        let border_color = if is_focused { Color::Cyan } else { Color::White };
        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Tasks").border_style(Style::default().fg(border_color)));
        f.render_widget(list, input_area[0]);

        // Render input box
        let input = Paragraph::new(app.view_state.input_buffer.as_str())
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title("New Task"));
        f.render_widget(input, input_area[1]);

        // Show cursor
        f.set_cursor_position((input_area[1].x + app.view_state.cursor_position as u16 + 1, input_area[1].y + 1));
    } else {
        // Normal mode - just show tasks
        let items: Vec<ListItem> = tasks
            .iter()
            .enumerate()
            .map(|(i, task)| {
                let checkbox = if task.completed { "[✓]" } else { "[ ]" };
                let style = if i == app.selected_task_index {
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

        let border_color = if is_focused { Color::Cyan } else { Color::White };
        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Tasks").border_style(Style::default().fg(border_color)));
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
        Paragraph::new(vec![
            Line::from(vec![
                Span::styled(
                    format!(" {} ", mode_text),
                    Style::default().bg(Color::Blue).fg(Color::White),
                ),
                sync_indicator,
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
                sync_indicator,
                Span::raw(help_text),
            ]),
        ])
        .block(Block::default().borders(Borders::ALL))
    };

    f.render_widget(status, area);
}

fn render_project_header(f: &mut Frame, app: &App, area: Rect) {
    let is_focused = app.view_state.focused_pane == FocusPane::Projects;
    let border_color = if is_focused { Color::Cyan } else { Color::White };

    let header = Paragraph::new("Projects")
        .style(Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(border_color)));

    f.render_widget(header, area);
}

fn render_project_list(f: &mut Frame, app: &App, area: Rect) {
    let projects = app.data.get_projects_sorted();
    let is_focused = app.view_state.focused_pane == FocusPane::Projects;

    // Count all tasks for "All Projects"
    let all_tasks_count = app.data.tasks.len();

    // Check if "All Projects" is the active filter
    let is_all_active = app.current_project_id.is_none();

    if app.view_state.input_mode == InputMode::Insert && is_focused {
        // Show input prompt for adding new project
        let input_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(area);

        // Build items list with "All Projects" first
        let mut items: Vec<ListItem> = Vec::new();

        // "All Projects" item (index 0)
        let is_all_selected = app.selected_project_index_in_pane == 0;
        let all_prefix = if is_all_active { ">" } else { " " };
        let all_style = if is_all_selected {
            Style::default().bg(Color::Magenta).fg(Color::Black).add_modifier(Modifier::BOLD)
        } else if is_all_active {
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };
        items.push(ListItem::new(format!("{} All Projects ({})", all_prefix, all_tasks_count)).style(all_style));

        // Project items (index 1+)
        for (i, project) in projects.iter().enumerate() {
            let task_count = app.data.tasks.values()
                .filter(|t| t.project_id == project.id)
                .count();
            let is_selected = app.selected_project_index_in_pane == i + 1;
            let is_active = app.current_project_id.as_deref() == Some(&project.id);
            let prefix = if is_active { ">" } else { " " };

            let style = if is_selected {
                Style::default().bg(Color::Magenta).fg(Color::Black).add_modifier(Modifier::BOLD)
            } else if is_active {
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            items.push(ListItem::new(format!("{} {} ({})", prefix, project.name, task_count)).style(style));
        }

        let border_color = if is_focused { Color::Magenta } else { Color::White };
        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Projects").border_style(Style::default().fg(border_color)));
        f.render_widget(list, input_area[0]);

        // Render input box
        let input = Paragraph::new(app.view_state.input_buffer.as_str())
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title("New Project"));
        f.render_widget(input, input_area[1]);

        // Show cursor
        f.set_cursor_position((input_area[1].x + app.view_state.cursor_position as u16 + 1, input_area[1].y + 1));
    } else {
        // Normal mode - show projects with "All Projects" first
        let mut items: Vec<ListItem> = Vec::new();

        // "All Projects" item (index 0)
        let is_all_selected = app.selected_project_index_in_pane == 0;
        let all_prefix = if is_all_active { ">" } else { " " };
        let all_style = if is_all_selected && is_focused {
            Style::default().bg(Color::Magenta).fg(Color::Black).add_modifier(Modifier::BOLD)
        } else if is_all_active {
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };
        items.push(ListItem::new(format!("{} All Projects ({})", all_prefix, all_tasks_count)).style(all_style));

        // Project items (index 1+)
        for (i, project) in projects.iter().enumerate() {
            let task_count = app.data.tasks.values()
                .filter(|t| t.project_id == project.id)
                .count();
            let is_selected = app.selected_project_index_in_pane == i + 1;
            let is_active = app.current_project_id.as_deref() == Some(&project.id);
            let prefix = if is_active { ">" } else { " " };

            let style = if is_selected && is_focused {
                Style::default().bg(Color::Magenta).fg(Color::Black).add_modifier(Modifier::BOLD)
            } else if is_active {
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            items.push(ListItem::new(format!("{} {} ({})", prefix, project.name, task_count)).style(style));
        }

        let border_color = if is_focused { Color::Magenta } else { Color::White };
        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Projects").border_style(Style::default().fg(border_color)));
        f.render_widget(list, area);
    }
}

fn render_project_status_bar(f: &mut Frame, app: &App, area: Rect) {
    let is_focused = app.view_state.focused_pane == FocusPane::Projects;
    let mode_text = match app.view_state.input_mode {
        InputMode::Normal => "NORMAL",
        InputMode::Insert => "INSERT",
    };

    let help_text = if app.view_state.input_mode == InputMode::Insert && is_focused {
        "ESC cancel | ENTER confirm"
    } else if is_focused {
        "ENTER filter | a add | e edit | d delete"
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
