use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkstreamState {
    Idle,
    Running,
    NeedsInput,
}

impl Default for WorkstreamState {
    fn default() -> Self {
        Self::Idle
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workstream {
    pub id: String,
    pub name: String,
    pub preset_id: String,
    pub state: WorkstreamState,
    pub created_at: String,
    pub updated_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_accessed: Option<String>,
}

impl Workstream {
    pub fn new(name: String, preset_id: String) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            preset_id,
            state: WorkstreamState::Idle,
            created_at: now.clone(),
            updated_at: now,
            last_accessed: None,
        }
    }

    pub fn update_timestamp(&mut self) {
        self.updated_at = chrono::Utc::now().to_rfc3339();
    }

    pub fn mark_accessed(&mut self) {
        self.last_accessed = Some(chrono::Utc::now().to_rfc3339());
        self.update_timestamp();
    }

    pub fn set_state(&mut self, state: WorkstreamState) {
        self.state = state;
        self.update_timestamp();
    }
}
