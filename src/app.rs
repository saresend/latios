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

    // Detail view state
    pub editing_task_id: Option<String>,
    pub detail_field_selection: usize,
    pub detail_scroll: usize,
    pub multiline_buffer: Vec<String>,

    // File path for data persistence
    pub data_file_path: String,

    // Status message
    pub status_message: Option<String>,
    pub status_timestamp: Option<std::time::Instant>,
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
            editing_task_id: None,
            detail_field_selection: 0,
            detail_scroll: 0,
            multiline_buffer: Vec::new(),
            data_file_path,
            status_message: None,
            status_timestamp: None,
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
        self.status_timestamp = Some(std::time::Instant::now());
    }

    pub fn clear_status(&mut self) {
        self.status_message = None;
        self.status_timestamp = None;
    }

    pub fn check_status_timeout(&mut self) {
        if let Some(timestamp) = self.status_timestamp {
            if timestamp.elapsed() >= std::time::Duration::from_millis(500) {
                self.clear_status();
            }
        }
    }

    // Detail view methods
    pub fn start_edit_task(&mut self) {
        if let Some(task_id) = self.get_selected_task_id() {
            self.editing_task_id = Some(task_id);
            self.current_view = AppView::TaskDetail;
            self.detail_field_selection = 0;
            self.input_mode = InputMode::Normal;
        }
    }

    pub fn exit_detail_view(&mut self) {
        self.editing_task_id = None;
        self.current_view = AppView::TaskList;
        self.input_mode = InputMode::Normal;
        self.input_buffer.clear();
        self.multiline_buffer.clear();
    }

    pub fn next_detail_field(&mut self) {
        // Max 3 sections: title, description, tags, file_refs
        self.detail_field_selection = (self.detail_field_selection + 1).min(3);
    }

    pub fn previous_detail_field(&mut self) {
        if self.detail_field_selection > 0 {
            self.detail_field_selection -= 1;
        }
    }

    pub fn get_editing_task(&self) -> Option<&crate::models::Task> {
        self.editing_task_id
            .as_ref()
            .and_then(|id| self.data.get_task(id))
    }

    pub fn get_editing_task_mut(&mut self) -> Option<&mut crate::models::Task> {
        self.editing_task_id
            .as_ref()
            .and_then(|id| self.data.get_task_mut(id))
    }
}
