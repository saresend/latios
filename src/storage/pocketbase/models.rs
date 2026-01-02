use serde::{Deserialize, Serialize};
use crate::models::{Task, Project, FileReference};

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
    pub project_local_id: String,
    pub tags: Vec<String>,
    pub file_references: serde_json::Value,
    pub created_at: String,
    pub updated_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
}

/// PocketBase project record
#[derive(Debug, Serialize, Deserialize)]
pub struct PBProject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub local_id: String,
    pub name: String,
    pub description: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<&Task> for PBTask {
    fn from(task: &Task) -> Self {
        let file_refs = serde_json::to_value(&task.file_references)
            .unwrap_or(serde_json::Value::Array(vec![]));

        Self {
            id: task.remote_id.clone(),
            local_id: task.id.clone(),
            title: task.title.clone(),
            description: task.description.clone(),
            completed: task.completed,
            project_local_id: task.project_id.clone(),
            tags: task.tags.clone(),
            file_references: file_refs,
            created_at: task.created_at.clone(),
            updated_at: task.updated_at.clone(),
            completed_at: task.completed_at.clone(),
        }
    }
}

impl PBTask {
    pub fn into_task(self, project_id: String) -> Task {
        let file_references: Vec<FileReference> = serde_json::from_value(self.file_references)
            .unwrap_or_default();

        Task {
            id: self.local_id,
            remote_id: self.id,
            title: self.title,
            description: self.description,
            completed: self.completed,
            project_id,
            tags: self.tags,
            file_references,
            created_at: self.created_at,
            updated_at: self.updated_at,
            completed_at: self.completed_at,
        }
    }
}

impl From<&Project> for PBProject {
    fn from(project: &Project) -> Self {
        Self {
            id: project.remote_id.clone(),
            local_id: project.id.clone(),
            name: project.name.clone(),
            description: project.description.clone(),
            created_at: project.created_at.clone(),
            updated_at: project.updated_at.clone(),
        }
    }
}

impl PBProject {
    pub fn into_project(self) -> Project {
        Project {
            id: self.local_id,
            remote_id: self.id,
            name: self.name,
            description: self.description,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
