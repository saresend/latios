use crate::models::AppData;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppView {
    TaskList,
    TaskDetail,
    ProjectList,
    Help,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    Insert,
}

pub struct App {
    pub data: AppData,
    pub current_view: AppView,
    pub should_quit: bool,

    // Task list state
    pub selected_task_index: usize,
    pub task_list_scroll: usize,

    // Project filter
    pub current_project_id: Option<String>,
    pub selected_project_index: usize,

    // Input buffers for editing
    pub input_mode: InputMode,
    pub input_buffer: String,
    pub cursor_position: usize,

    // File path for data persistence
    pub data_file_path: String,

    // Status message
    pub status_message: Option<String>,
}

impl App {
    pub fn new(data_file_path: String) -> Self {
        Self {
            data: AppData::default(),
            current_view: AppView::TaskList,
            should_quit: false,
            selected_task_index: 0,
            task_list_scroll: 0,
            current_project_id: None,
            selected_project_index: 0,
            input_mode: InputMode::Normal,
            input_buffer: String::new(),
            cursor_position: 0,
            data_file_path,
            status_message: None,
        }
    }

    // Task navigation
    pub fn next_task(&mut self) {
        let task_count = self.get_visible_tasks().len();
        if task_count > 0 {
            self.selected_task_index = (self.selected_task_index + 1).min(task_count - 1);
        }
    }

    pub fn previous_task(&mut self) {
        if self.selected_task_index > 0 {
            self.selected_task_index -= 1;
        }
    }

    // Get tasks visible in current view
    fn get_visible_tasks(&self) -> Vec<&crate::models::Task> {
        let mut tasks = self.data.get_tasks_by_project(
            self.current_project_id.as_deref()
        );
        tasks.sort_by(|a, b| a.created_at.cmp(&b.created_at));
        tasks
    }

    // Get the ID of the currently selected task
    pub fn get_selected_task_id(&self) -> Option<String> {
        let tasks = self.get_visible_tasks();
        tasks.get(self.selected_task_index).map(|t| t.id.clone())
    }

    // Task operations
    pub fn toggle_selected_task(&mut self) {
        if let Some(task_id) = self.get_selected_task_id() {
            if let Some(task) = self.data.get_task_mut(&task_id) {
                task.toggle_complete();
            }
        }
    }

    pub fn delete_selected_task(&mut self) {
        if let Some(task_id) = self.get_selected_task_id() {
            self.data.remove_task(&task_id);
            // Adjust selection if needed
            let task_count = self.get_visible_tasks().len();
            if self.selected_task_index >= task_count && task_count > 0 {
                self.selected_task_index = task_count - 1;
            }
        }
    }

    pub fn start_add_task(&mut self) {
        self.input_mode = InputMode::Insert;
        self.input_buffer.clear();
        self.cursor_position = 0;
    }

    pub fn confirm_add_task(&mut self) {
        if !self.input_buffer.is_empty() {
            let mut task = crate::models::Task::new(self.input_buffer.clone());
            task.project_id = self.current_project_id.clone();
            self.data.add_task(task);
            self.input_buffer.clear();
            self.input_mode = InputMode::Normal;
        }
    }

    pub fn cancel_input(&mut self) {
        self.input_mode = InputMode::Normal;
        self.input_buffer.clear();
        self.cursor_position = 0;
    }

    pub fn set_status(&mut self, message: String) {
        self.status_message = Some(message);
    }

    pub fn clear_status(&mut self) {
        self.status_message = None;
    }
}
