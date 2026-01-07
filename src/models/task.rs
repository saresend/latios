use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remote_id: Option<String>,
    pub title: String,
    pub description: String,
    pub completed: bool,
    #[serde(default)]
    pub metadata: HashMap<String, String>,
    #[serde(default)]
    pub workstream_ids: Vec<String>,
    pub tags: Vec<String>,
    pub file_references: Vec<FileReference>,
    pub created_at: String,
    pub updated_at: String,
    pub completed_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileReference {
    pub path: String,
    pub line_number: Option<usize>,
    pub description: Option<String>,
}

impl Task {
    pub fn new(title: String) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            remote_id: None,
            title,
            description: String::new(),
            completed: false,
            metadata: HashMap::new(),
            workstream_ids: Vec::new(),
            tags: Vec::new(),
            file_references: Vec::new(),
            created_at: now.clone(),
            updated_at: now,
            completed_at: None,
        }
    }

    pub fn toggle_complete(&mut self) {
        self.completed = !self.completed;
        self.updated_at = chrono::Utc::now().to_rfc3339();
        if self.completed {
            self.completed_at = Some(chrono::Utc::now().to_rfc3339());
        } else {
            self.completed_at = None;
        }
    }

    pub fn update_timestamp(&mut self) {
        self.updated_at = chrono::Utc::now().to_rfc3339();
    }

    pub fn add_workstream(&mut self, workstream_id: String) {
        if !self.workstream_ids.contains(&workstream_id) {
            self.workstream_ids.push(workstream_id);
            self.update_timestamp();
        }
    }

    pub fn remove_workstream(&mut self, workstream_id: &str) {
        self.workstream_ids.retain(|id| id != workstream_id);
        self.update_timestamp();
    }

    pub fn set_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
        self.update_timestamp();
    }

    pub fn remove_metadata(&mut self, key: &str) {
        self.metadata.remove(key);
        self.update_timestamp();
    }
}

impl FileReference {
    pub fn new(path: String) -> Self {
        Self {
            path,
            line_number: None,
            description: None,
        }
    }
}
