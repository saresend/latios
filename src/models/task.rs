use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remote_id: Option<String>,
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub project_id: String,
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
    pub fn new(title: String, project_id: String) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            remote_id: None,
            title,
            description: String::new(),
            completed: false,
            project_id,
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

