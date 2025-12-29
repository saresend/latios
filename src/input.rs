use crossterm::event::{self, Event, KeyCode, KeyEvent};
use crate::app::App;
use crate::models::{AppView, FocusPane, InputMode, DetailEditField};

pub fn handle_input(app: &mut App) -> anyhow::Result<()> {
    if event::poll(std::time::Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            match app.view_state.input_mode {
                InputMode::Normal => handle_normal_mode(app, key),
                InputMode::Insert => handle_insert_mode(app, key),
            }
        }
    }
    Ok(())
}

fn handle_normal_mode(app: &mut App, key: KeyEvent) {
    match app.view_state.current_view {
        AppView::TaskList => handle_task_list_normal(app, key),
        AppView::TaskDetail => handle_task_detail_normal(app, key),
        AppView::ProjectDetail => handle_project_detail_normal(app, key),
        AppView::Help => handle_help_view(app, key),
        _ => {}
    }
}

fn handle_task_list_normal(app: &mut App, key: KeyEvent) {
    if key.code == KeyCode::Tab {
        app.switch_pane();
        return;
    }

    // Global keys
    match key.code {
        KeyCode::Char('q') => {
            app.should_quit = true;
            return;
        }
        KeyCode::Char('?') => {
            app.view_state.current_view = AppView::Help;
            return;
        }
        _ => {}
    }

    match app.view_state.focused_pane {
        FocusPane::Tasks => handle_tasks_pane_normal(app, key),
        FocusPane::Projects => handle_projects_pane_normal(app, key),
    }
}

fn handle_tasks_pane_normal(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('j') | KeyCode::Down => app.next_task(),
        KeyCode::Char('k') | KeyCode::Up => app.previous_task(),
        KeyCode::Char(' ') | KeyCode::Enter => app.toggle_selected_task(),
        KeyCode::Char('d') => app.delete_selected_task(),
        KeyCode::Char('a') => app.start_add_task(),
        KeyCode::Char('e') => app.start_edit_task(),
        KeyCode::Char('c') | KeyCode::Char('y') => copy_task_to_clipboard(app),
        KeyCode::Char('x') => export_tasks(app),
        _ => {}
    }
}

fn handle_projects_pane_normal(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('j') | KeyCode::Down => app.next_project(),
        KeyCode::Char('k') | KeyCode::Up => app.previous_project(),
        KeyCode::Char('a') => app.start_add_project(),
        KeyCode::Char('e') | KeyCode::Enter => app.start_edit_project(),
        KeyCode::Char('d') => app.delete_selected_project(),
        _ => {}
    }
}

fn handle_task_detail_normal(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => app.exit_detail_view(),
        KeyCode::Char('j') | KeyCode::Down => {
            // Navigate between fields, or within lists
            if app.detail_field_selection == 2 {
                app.next_tag();
            } else if app.detail_field_selection == 3 {
                app.next_file_ref();
            } else {
                app.next_detail_field();
            }
        }
        KeyCode::Char('k') | KeyCode::Up => {
            // Navigate between fields, or within lists
            if app.detail_field_selection == 2 {
                app.previous_tag();
            } else if app.detail_field_selection == 3 {
                app.previous_file_ref();
            } else {
                app.previous_detail_field();
            }
        }
        KeyCode::Tab => app.next_detail_field(),
        KeyCode::Char('i') | KeyCode::Enter => start_editing_current_field(app),
        KeyCode::Char('a') => add_to_current_list(app),
        KeyCode::Char('d') => delete_from_current_list(app),
        KeyCode::Char('h') | KeyCode::Left => {
            // Navigate within lists
            if app.detail_field_selection == 2 {
                app.previous_tag();
            } else if app.detail_field_selection == 3 {
                app.previous_file_ref();
            }
        }
        KeyCode::Char('l') | KeyCode::Right => {
            // Navigate within lists
            if app.detail_field_selection == 2 {
                app.next_tag();
            } else if app.detail_field_selection == 3 {
                app.next_file_ref();
            }
        }
        _ => {}
    }
}

fn handle_project_detail_normal(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => app.exit_project_detail_view(),
        KeyCode::Char('j') | KeyCode::Down => app.next_detail_field(),
        KeyCode::Char('k') | KeyCode::Up => app.previous_detail_field(),
        KeyCode::Tab => app.next_detail_field(),
        KeyCode::Char('i') | KeyCode::Enter => start_editing_current_project_field(app),
        _ => {}
    }
}

fn start_editing_current_project_field(app: &mut App) {
    match app.detail_field_selection {
        0 => app.start_edit_project_name(),
        1 => app.start_edit_project_description(),
        _ => {}
    }
}

fn start_editing_current_field(app: &mut App) {
    match app.detail_field_selection {
        0 => app.start_edit_title(),
        1 => app.start_edit_description(),
        _ => {}
    }
}

fn add_to_current_list(app: &mut App) {
    match app.detail_field_selection {
        2 => app.start_add_tag(),
        3 => app.start_add_file_ref(),
        _ => {}
    }
}

fn delete_from_current_list(app: &mut App) {
    match app.detail_field_selection {
        2 => app.delete_selected_tag(),
        3 => app.delete_selected_file_ref(),
        _ => {}
    }
}

fn handle_help_view(app: &mut App, _key: KeyEvent) {
    // Any key returns to task list
    app.view_state.current_view = AppView::TaskList;
}

fn handle_insert_mode(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => cancel_current_edit(app),
        KeyCode::Enter => handle_enter_in_edit_mode(app),
        KeyCode::Up => {
            if let Some(edit_state) = &mut app.view_state.description_edit_state {
                edit_state.move_cursor_up();
            }
        }
        KeyCode::Down => {
            if let Some(edit_state) = &mut app.view_state.description_edit_state {
                edit_state.move_cursor_down();
            }
        }
        KeyCode::Char(c) => {
            if let Some(edit_state) = &mut app.view_state.description_edit_state {
                edit_state.insert_char(c);
            } else {
                app.view_state.input_buffer.insert(app.view_state.cursor_position, c);
                app.view_state.cursor_position += 1;
            }
        }
        KeyCode::Backspace => {
            if let Some(edit_state) = &mut app.view_state.description_edit_state {
                edit_state.backspace();
            } else if app.view_state.cursor_position > 0 {
                app.view_state.input_buffer.remove(app.view_state.cursor_position - 1);
                app.view_state.cursor_position -= 1;
            }
        }
        KeyCode::Left => {
            if let Some(edit_state) = &mut app.view_state.description_edit_state {
                edit_state.move_cursor_left();
            } else if app.view_state.cursor_position > 0 {
                app.view_state.cursor_position -= 1;
            }
        }
        KeyCode::Right => {
            if let Some(edit_state) = &mut app.view_state.description_edit_state {
                edit_state.move_cursor_right();
            } else if app.view_state.cursor_position < app.view_state.input_buffer.len() {
                app.view_state.cursor_position += 1;
            }
        }
        _ => {}
    }
}

fn handle_enter_in_edit_mode(app: &mut App) {
    match &app.view_state.detail_editing_field {
        Some(DetailEditField::Title) => app.save_title_edit(),
        Some(DetailEditField::AddingTag) => app.save_new_tag(),
        Some(DetailEditField::AddingFileRef) => app.advance_file_ref_step(),
        Some(DetailEditField::Description) | Some(DetailEditField::ProjectDescription) => {
            // Insert newline in the description editor
            if let Some(edit_state) = &mut app.view_state.description_edit_state {
                edit_state.insert_newline();
            }
        }
        Some(DetailEditField::ProjectName) => app.save_project_name_edit(),
        None => {
            // Route based on focused pane
            match app.view_state.focused_pane {
                FocusPane::Tasks => app.confirm_add_task(),
                FocusPane::Projects => app.confirm_add_project(),
            }
        }
    }
}

fn cancel_current_edit(app: &mut App) {
    // For description editing, ESC saves the changes
    if app.view_state.detail_editing_field == Some(DetailEditField::Description) {
        app.save_description_edit();
    } else if app.view_state.detail_editing_field == Some(DetailEditField::ProjectDescription) {
        app.save_project_description_edit();
    } else if app.view_state.detail_editing_field.is_some() {
        // For other edit fields, ESC cancels
        app.view_state.detail_editing_field = None;
        app.view_state.input_mode = InputMode::Normal;
        app.view_state.input_buffer.clear();
    } else {
        app.cancel_input();
    }
}

fn export_tasks(app: &mut App) {
    use crate::storage::export::export_to_markdown;

    let timestamp = chrono::Utc::now().format("%Y%m%d-%H%M%S");
    let filename = format!("latios-export-{}.md", timestamp);

    match export_to_markdown(&app.data, &filename, app.current_project_id.as_deref()) {
        Ok(_) => {
            app.set_status(format!("Exported to {}", filename));
        }
        Err(e) => {
            app.set_status(format!("Export failed: {}", e));
        }
    }
}

fn copy_task_to_clipboard(app: &mut App) {
    use arboard::Clipboard;

    if let Some(task_id) = app.get_selected_task_id() {
        if let Some(task) = app.data.get_task(&task_id) {
            let context = format_task_for_clipboard(task, &app.data);

            match Clipboard::new().and_then(|mut cb| cb.set_text(context)) {
                Ok(_) => app.set_status("Task copied to clipboard!".to_string()),
                Err(e) => app.set_status(format!("Copy failed: {}", e)),
            }
        }
    }
}

fn format_task_for_clipboard(task: &crate::models::Task, data: &crate::models::AppData) -> String {
    let mut output = String::new();

    // Title
    output.push_str(&format!("# Task: {}\n\n", task.title));

    // Metadata
    output.push_str(&format!("**Status:** {}\n",
        if task.completed { "Completed" } else { "Pending" }));
    output.push_str(&format!("**Created:** {}\n", task.created_at));

    // Project
    if let Some(pid) = &task.project_id {
        if let Some(project) = data.get_project(pid) {
            output.push_str(&format!("**Project:** {}\n", project.name));
        }
    }

    // Tags
    if !task.tags.is_empty() {
        output.push_str(&format!("**Tags:** {}\n", task.tags.join(", ")));
    }

    output.push_str("\n");

    // Description
    if !task.description.is_empty() {
        output.push_str("## Description\n\n");
        output.push_str(&task.description);
        output.push_str("\n\n");
    }

    // File references
    if !task.file_references.is_empty() {
        output.push_str("## File References\n\n");
        for file_ref in &task.file_references {
            if let Some(line) = file_ref.line_number {
                output.push_str(&format!("- `{}:{}`", file_ref.path, line));
            } else {
                output.push_str(&format!("- `{}`", file_ref.path));
            }
            if let Some(desc) = &file_ref.description {
                output.push_str(&format!(" - {}", desc));
            }
            output.push_str("\n");
        }
    }

    output
}
