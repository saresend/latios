use crate::models::AppData;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppView {
    TaskList,
    TaskDetail,
    ProjectDetail,
    ProjectList,
    Help,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusPane {
    Tasks,
    Projects,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    Insert,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DetailEditField {
    Title,
    Description,
    AddingTag,
    AddingFileRef,
    ProjectName,
    ProjectDescription,
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

    // Split view state
    pub focused_pane: FocusPane,
    pub selected_project_index_in_pane: usize,
    pub project_list_scroll: usize,

    // Input buffers for editing
    pub input_mode: InputMode,
    pub input_buffer: String,
    pub cursor_position: usize,

    // Detail view state
    pub editing_task_id: Option<String>,
    pub editing_project_id: Option<String>,
    pub detail_field_selection: usize,
    pub detail_scroll: usize,
    pub multiline_buffer: Vec<String>,
    pub current_editing_line_index: usize,
    pub detail_editing_field: Option<DetailEditField>,
    pub selected_tag_index: usize,
    pub selected_file_ref_index: usize,

    // File reference input buffers
    pub file_ref_path_buffer: String,
    pub file_ref_line_buffer: String,
    pub file_ref_desc_buffer: String,
    pub file_ref_input_step: usize,

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
            focused_pane: FocusPane::Tasks,
            selected_project_index_in_pane: 0,
            project_list_scroll: 0,
            input_mode: InputMode::Normal,
            input_buffer: String::new(),
            cursor_position: 0,
            editing_task_id: None,
            editing_project_id: None,
            detail_field_selection: 0,
            detail_scroll: 0,
            multiline_buffer: Vec::new(),
            current_editing_line_index: 0,
            detail_editing_field: None,
            selected_tag_index: 0,
            selected_file_ref_index: 0,
            file_ref_path_buffer: String::new(),
            file_ref_line_buffer: String::new(),
            file_ref_desc_buffer: String::new(),
            file_ref_input_step: 0,
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

    // Edit field methods
    pub fn start_edit_title(&mut self) {
        if let Some(task) = self.get_editing_task() {
            self.input_buffer = task.title.clone();
            self.cursor_position = self.input_buffer.len();
            self.detail_editing_field = Some(DetailEditField::Title);
            self.input_mode = InputMode::Insert;
        }
    }

    pub fn save_title_edit(&mut self) {
        let new_title = self.input_buffer.clone();
        if let Some(task) = self.get_editing_task_mut() {
            task.title = new_title;
            task.update_timestamp();
        }
        self.input_buffer.clear();
        self.detail_editing_field = None;
        self.input_mode = InputMode::Normal;
    }

    pub fn start_edit_description(&mut self) {
        if let Some(task) = self.get_editing_task() {
            self.multiline_buffer = task.description
                .lines()
                .map(|s| s.to_string())
                .collect();
            if self.multiline_buffer.is_empty() {
                self.multiline_buffer.push(String::new());
            }
            // Start editing the last line
            self.current_editing_line_index = self.multiline_buffer.len().saturating_sub(1);
            self.input_buffer = self.multiline_buffer[self.current_editing_line_index].clone();
            self.cursor_position = self.input_buffer.len();
            self.detail_editing_field = Some(DetailEditField::Description);
            self.input_mode = InputMode::Insert;
        }
    }

    pub fn save_description_edit(&mut self) {
        // Save the current line being edited
        if self.current_editing_line_index < self.multiline_buffer.len() {
            self.multiline_buffer[self.current_editing_line_index] = self.input_buffer.clone();
        }

        let new_description = self.multiline_buffer.join("\n");
        if let Some(task) = self.get_editing_task_mut() {
            task.description = new_description;
            task.update_timestamp();
        }
        self.multiline_buffer.clear();
        self.input_buffer.clear();
        self.current_editing_line_index = 0;
        self.detail_editing_field = None;
        self.input_mode = InputMode::Normal;
    }

    pub fn start_add_tag(&mut self) {
        self.input_buffer.clear();
        self.cursor_position = 0;
        self.detail_editing_field = Some(DetailEditField::AddingTag);
        self.input_mode = InputMode::Insert;
    }

    pub fn save_new_tag(&mut self) {
        if !self.input_buffer.is_empty() {
            let new_tag = self.input_buffer.clone();
            if let Some(task) = self.get_editing_task_mut() {
                task.tags.push(new_tag);
                task.update_timestamp();
            }
        }
        self.input_buffer.clear();
        self.detail_editing_field = None;
        self.input_mode = InputMode::Normal;
    }

    pub fn next_tag(&mut self) {
        if let Some(task) = self.get_editing_task() {
            if !task.tags.is_empty() {
                self.selected_tag_index = (self.selected_tag_index + 1).min(task.tags.len() - 1);
            }
        }
    }

    pub fn previous_tag(&mut self) {
        if self.selected_tag_index > 0 {
            self.selected_tag_index -= 1;
        }
    }

    pub fn delete_selected_tag(&mut self) {
        let index = self.selected_tag_index;
        if let Some(task) = self.get_editing_task_mut() {
            if index < task.tags.len() {
                task.tags.remove(index);
                task.update_timestamp();
            }
        }
        // Adjust selection after mutable borrow ends
        if let Some(task) = self.get_editing_task() {
            if self.selected_tag_index >= task.tags.len() && !task.tags.is_empty() {
                self.selected_tag_index = task.tags.len() - 1;
            } else if task.tags.is_empty() {
                self.selected_tag_index = 0;
            }
        }
    }

    pub fn start_add_file_ref(&mut self) {
        self.file_ref_path_buffer.clear();
        self.file_ref_line_buffer.clear();
        self.file_ref_desc_buffer.clear();
        self.file_ref_input_step = 0;
        self.input_buffer.clear();
        self.cursor_position = 0;
        self.detail_editing_field = Some(DetailEditField::AddingFileRef);
        self.input_mode = InputMode::Insert;
    }

    pub fn advance_file_ref_step(&mut self) {
        match self.file_ref_input_step {
            0 => self.file_ref_path_buffer = self.input_buffer.clone(),
            1 => self.file_ref_line_buffer = self.input_buffer.clone(),
            2 => self.file_ref_desc_buffer = self.input_buffer.clone(),
            _ => {}
        }

        self.file_ref_input_step += 1;
        self.input_buffer.clear();
        self.cursor_position = 0;

        if self.file_ref_input_step > 2 {
            self.save_file_ref();
        }
    }

    pub fn save_file_ref(&mut self) {
        if !self.file_ref_path_buffer.is_empty() {
            // Clone buffers before mutable borrow
            let path = self.file_ref_path_buffer.clone();
            let line_num = self.file_ref_line_buffer.parse::<usize>().ok();
            let desc = if self.file_ref_desc_buffer.is_empty() {
                None
            } else {
                Some(self.file_ref_desc_buffer.clone())
            };

            if let Some(task) = self.get_editing_task_mut() {
                task.file_references.push(crate::models::FileReference {
                    path,
                    line_number: line_num,
                    description: desc,
                });
                task.update_timestamp();
            }
        }

        self.file_ref_path_buffer.clear();
        self.file_ref_line_buffer.clear();
        self.file_ref_desc_buffer.clear();
        self.file_ref_input_step = 0;
        self.detail_editing_field = None;
        self.input_mode = InputMode::Normal;
    }

    pub fn next_file_ref(&mut self) {
        if let Some(task) = self.get_editing_task() {
            if !task.file_references.is_empty() {
                self.selected_file_ref_index = (self.selected_file_ref_index + 1).min(task.file_references.len() - 1);
            }
        }
    }

    pub fn previous_file_ref(&mut self) {
        if self.selected_file_ref_index > 0 {
            self.selected_file_ref_index -= 1;
        }
    }

    pub fn delete_selected_file_ref(&mut self) {
        let index = self.selected_file_ref_index;
        if let Some(task) = self.get_editing_task_mut() {
            if index < task.file_references.len() {
                task.file_references.remove(index);
                task.update_timestamp();
            }
        }
        // Adjust selection after mutable borrow ends
        if let Some(task) = self.get_editing_task() {
            if self.selected_file_ref_index >= task.file_references.len() && !task.file_references.is_empty() {
                self.selected_file_ref_index = task.file_references.len() - 1;
            } else if task.file_references.is_empty() {
                self.selected_file_ref_index = 0;
            }
        }
    }

    // Pane switching
    pub fn switch_pane(&mut self) {
        self.focused_pane = match self.focused_pane {
            FocusPane::Tasks => FocusPane::Projects,
            FocusPane::Projects => FocusPane::Tasks,
        };
    }

    // Project navigation
    pub fn next_project(&mut self) {
        let project_count = self.data.get_projects_sorted().len();
        if project_count > 0 {
            self.selected_project_index_in_pane = (self.selected_project_index_in_pane + 1).min(project_count - 1);
        }
    }

    pub fn previous_project(&mut self) {
        if self.selected_project_index_in_pane > 0 {
            self.selected_project_index_in_pane -= 1;
        }
    }

    pub fn get_selected_project_id(&self) -> Option<String> {
        let projects = self.data.get_projects_sorted();
        projects.get(self.selected_project_index_in_pane).map(|p| p.id.clone())
    }

    // Project operations
    pub fn delete_selected_project(&mut self) {
        if let Some(project_id) = self.get_selected_project_id() {
            self.data.remove_project(&project_id);
            // Adjust selection if needed
            let project_count = self.data.get_projects_sorted().len();
            if self.selected_project_index_in_pane >= project_count && project_count > 0 {
                self.selected_project_index_in_pane = project_count - 1;
            }
        }
    }

    pub fn start_add_project(&mut self) {
        self.input_mode = InputMode::Insert;
        self.input_buffer.clear();
        self.cursor_position = 0;
    }

    pub fn confirm_add_project(&mut self) {
        if !self.input_buffer.is_empty() {
            let project = crate::models::Project::new(self.input_buffer.clone());
            self.data.add_project(project);
            self.input_buffer.clear();
            self.input_mode = InputMode::Normal;
        }
    }

    pub fn start_edit_project(&mut self) {
        if let Some(project_id) = self.get_selected_project_id() {
            self.editing_project_id = Some(project_id);
            self.current_view = AppView::ProjectDetail;
            self.detail_field_selection = 0;
            self.input_mode = InputMode::Normal;
        }
    }

    pub fn exit_project_detail_view(&mut self) {
        self.editing_project_id = None;
        self.current_view = AppView::TaskList;
        self.input_mode = InputMode::Normal;
        self.input_buffer.clear();
        self.multiline_buffer.clear();
    }

    pub fn get_editing_project(&self) -> Option<&crate::models::Project> {
        self.editing_project_id
            .as_ref()
            .and_then(|id| self.data.get_project(id))
    }

    pub fn get_editing_project_mut(&mut self) -> Option<&mut crate::models::Project> {
        self.editing_project_id
            .as_ref()
            .and_then(|id| self.data.get_project_mut(id))
    }

    // Project edit field methods
    pub fn start_edit_project_name(&mut self) {
        if let Some(project) = self.get_editing_project() {
            self.input_buffer = project.name.clone();
            self.cursor_position = self.input_buffer.len();
            self.detail_editing_field = Some(DetailEditField::ProjectName);
            self.input_mode = InputMode::Insert;
        }
    }

    pub fn save_project_name_edit(&mut self) {
        let new_name = self.input_buffer.clone();
        if let Some(project) = self.get_editing_project_mut() {
            project.name = new_name;
            project.update_timestamp();
        }
        self.input_buffer.clear();
        self.detail_editing_field = None;
        self.input_mode = InputMode::Normal;
    }

    pub fn start_edit_project_description(&mut self) {
        if let Some(project) = self.get_editing_project() {
            self.multiline_buffer = project.description
                .lines()
                .map(|s| s.to_string())
                .collect();
            if self.multiline_buffer.is_empty() {
                self.multiline_buffer.push(String::new());
            }
            // Start editing the last line
            self.current_editing_line_index = self.multiline_buffer.len().saturating_sub(1);
            self.input_buffer = self.multiline_buffer[self.current_editing_line_index].clone();
            self.cursor_position = self.input_buffer.len();
            self.detail_editing_field = Some(DetailEditField::ProjectDescription);
            self.input_mode = InputMode::Insert;
        }
    }

    pub fn save_project_description_edit(&mut self) {
        // Save the current line being edited
        if self.current_editing_line_index < self.multiline_buffer.len() {
            self.multiline_buffer[self.current_editing_line_index] = self.input_buffer.clone();
        }

        let new_description = self.multiline_buffer.join("\n");
        if let Some(project) = self.get_editing_project_mut() {
            project.description = new_description;
            project.update_timestamp();
        }
        self.multiline_buffer.clear();
        self.input_buffer.clear();
        self.current_editing_line_index = 0;
        self.detail_editing_field = None;
        self.input_mode = InputMode::Normal;
    }

    // Multiline editing navigation
    pub fn move_to_previous_line(&mut self) {
        // Save current line first
        if self.current_editing_line_index < self.multiline_buffer.len() {
            self.multiline_buffer[self.current_editing_line_index] = self.input_buffer.clone();
        }

        // Move to previous line if possible
        if self.current_editing_line_index > 0 {
            self.current_editing_line_index -= 1;
            self.input_buffer = self.multiline_buffer[self.current_editing_line_index].clone();
            self.cursor_position = self.input_buffer.len();
        }
    }

    pub fn move_to_next_line(&mut self) {
        // Save current line first
        if self.current_editing_line_index < self.multiline_buffer.len() {
            self.multiline_buffer[self.current_editing_line_index] = self.input_buffer.clone();
        }

        // Move to next line if possible
        if self.current_editing_line_index + 1 < self.multiline_buffer.len() {
            self.current_editing_line_index += 1;
            self.input_buffer = self.multiline_buffer[self.current_editing_line_index].clone();
            self.cursor_position = self.input_buffer.len();
        }
    }
}
