use crate::models::{FileReference, Task};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// PocketBase list response wrapper
#[derive(Debug, Deserialize)]
pub struct ListResponse<T> {
    pub page: u32,
    #[serde(rename = "perPage")]
    pub per_page: u32,
    #[serde(rename = "totalPages")]
    pub total_pages: u32,
    #[serde(rename = "totalItems")]
    pub total_items: u32,
    pub items: Vec<T>,
}

/// PocketBase task record
#[derive(Debug, Serialize, Deserialize)]
pub struct PBTask {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub local_id: String,
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub tags: Vec<String>,
    pub file_references: serde_json::Value,
    pub metadata: serde_json::Value,
    pub workstream_ids: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
}

impl From<&Task> for PBTask {
    fn from(task: &Task) -> Self {
        let file_refs =
            serde_json::to_value(&task.file_references).unwrap_or(serde_json::Value::Array(vec![]));
        let metadata = serde_json::to_value(&task.metadata)
            .unwrap_or(serde_json::Value::Object(serde_json::Map::new()));

        Self {
            id: task.remote_id.clone(),
            local_id: task.id.clone(),
            title: task.title.clone(),
            description: task.description.clone(),
            completed: task.completed,
            tags: task.tags.clone(),
            file_references: file_refs,
            metadata,
            workstream_ids: task.workstream_ids.clone(),
            created_at: task.created_at.clone(),
            updated_at: task.updated_at.clone(),
            completed_at: task.completed_at.clone(),
        }
    }
}

impl PBTask {
    pub fn into_task(self) -> Task {
        let file_references: Vec<FileReference> =
            serde_json::from_value(self.file_references).unwrap_or_default();
        let metadata: HashMap<String, String> =
            serde_json::from_value(self.metadata).unwrap_or_default();

        Task {
            id: self.local_id,
            remote_id: self.id,
            title: self.title,
            description: self.description,
            completed: self.completed,
            metadata,
            workstream_ids: self.workstream_ids,
            tags: self.tags,
            file_references,
            created_at: self.created_at,
            updated_at: self.updated_at,
            completed_at: self.completed_at,
        }
    }
}
