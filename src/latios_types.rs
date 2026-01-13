use ratatui::widgets::WidgetRef;

pub(crate) trait AppState: WidgetRef {}

pub(crate) enum WorkstreamType {
    Claude,
    Codex,
    Learning,
    Handwritten,
}

pub struct Workstream {
    title: String,
    workstream_type: WorkstreamType,
}

impl Workstream {
    pub fn new(title: impl ToString, workstream_type: WorkstreamType) -> Self {
        Self {
            title: title.to_string(),
            workstream_type,
        }
    }
    pub(crate) fn get_line(&self) -> &str {
        &self.title
    }
}
