use super::{Task, Workstream};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppData {
    pub tasks: HashMap<String, Task>,
    pub workstreams: HashMap<String, Workstream>,
    pub version: String,
}

impl Default for AppData {
    fn default() -> Self {
        Self {
            tasks: HashMap::new(),
            workstreams: HashMap::new(),
            version: "2.0.0".to_string(),
        }
    }
}

impl AppData {
    /// Get all tasks sorted by created_at
    pub fn get_all_tasks(&self) -> Vec<&Task> {
        let mut tasks: Vec<_> = self.tasks.values().collect();
        tasks.sort_by(|a, b| a.created_at.cmp(&b.created_at));
        tasks
    }

    /// Get tasks linked to a specific workstream
    pub fn get_tasks_by_workstream(&self, workstream_id: &str) -> Vec<&Task> {
        self.tasks
            .values()
            .filter(|t| t.workstream_ids.contains(&workstream_id.to_string()))
            .collect()
    }

    pub fn find_task_by_remote_id(&self, remote_id: &str) -> Option<&Task> {
        self.tasks
            .values()
            .find(|t| t.remote_id.as_deref() == Some(remote_id))
    }

    pub fn find_task_by_remote_id_mut(&mut self, remote_id: &str) -> Option<&mut Task> {
        self.tasks
            .values_mut()
            .find(|t| t.remote_id.as_deref() == Some(remote_id))
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.insert(task.id.clone(), task);
    }

    pub fn remove_task(&mut self, task_id: &str) -> Option<Task> {
        self.tasks.remove(task_id)
    }

    pub fn get_task(&self, task_id: &str) -> Option<&Task> {
        self.tasks.get(task_id)
    }

    pub fn get_task_mut(&mut self, task_id: &str) -> Option<&mut Task> {
        self.tasks.get_mut(task_id)
    }

    // Workstream methods
    pub fn add_workstream(&mut self, workstream: Workstream) {
        self.workstreams.insert(workstream.id.clone(), workstream);
    }

    pub fn remove_workstream(&mut self, workstream_id: &str) -> Option<Workstream> {
        // Also remove this workstream from any tasks that reference it
        for task in self.tasks.values_mut() {
            task.workstream_ids.retain(|id| id != workstream_id);
        }
        self.workstreams.remove(workstream_id)
    }

    pub fn get_workstream(&self, workstream_id: &str) -> Option<&Workstream> {
        self.workstreams.get(workstream_id)
    }

    pub fn get_workstream_mut(&mut self, workstream_id: &str) -> Option<&mut Workstream> {
        self.workstreams.get_mut(workstream_id)
    }

    pub fn get_workstreams_sorted(&self) -> Vec<&Workstream> {
        let mut workstreams: Vec<_> = self.workstreams.values().collect();
        workstreams.sort_by(|a, b| a.created_at.cmp(&b.created_at));
        workstreams
    }

    /// Link a task to a workstream
    pub fn link_task_to_workstream(&mut self, task_id: &str, workstream_id: &str) {
        if let Some(task) = self.tasks.get_mut(task_id) {
            task.add_workstream(workstream_id.to_string());
        }
    }

    /// Unlink a task from a workstream
    pub fn unlink_task_from_workstream(&mut self, task_id: &str, workstream_id: &str) {
        if let Some(task) = self.tasks.get_mut(task_id) {
            task.remove_workstream(workstream_id);
        }
    }
}
