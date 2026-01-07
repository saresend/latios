#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppView {
    TaskList,
    TaskDetail,
    WorkstreamDetail,
    PresetPicker,
    Help,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusPane {
    Tasks,
    Workstreams,
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
    AddingMetadata,
    WorkstreamName,
}

use crate::models::DescriptionEditState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ViewState {
    pub current_view: AppView,
    pub focused_pane: FocusPane,

    pub editing_task_id: Option<String>,
    pub editing_workstream_id: Option<String>,
    pub detail_editing_field: Option<DetailEditField>,

    pub input_mode: InputMode,
    pub input_buffer: String,
    pub cursor_position: usize,

    pub description_edit_state: Option<DescriptionEditState>,

    // Preset picker state
    pub selected_preset_index: usize,
}

impl Default for ViewState {
    fn default() -> Self {
        Self {
            current_view: AppView::TaskList,
            focused_pane: FocusPane::Tasks,
            editing_task_id: None,
            editing_workstream_id: None,
            detail_editing_field: None,
            input_mode: InputMode::Normal,
            input_buffer: String::new(),
            cursor_position: 0,
            description_edit_state: None,
            selected_preset_index: 0,
        }
    }
}
