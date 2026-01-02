use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::{Task, Project};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppData {
    pub tasks: HashMap<String, Task>,
    pub projects: HashMap<String, Project>,
    pub version: String,
}

impl Default for AppData {
    fn default() -> Self {
        Self {
            tasks: HashMap::new(),
            projects: HashMap::new(),
            version: "1.0.0".to_string(),
        }
    }
}

impl AppData {
    /// Get tasks filtered by project.
    /// If project_id is None, returns all tasks.
    /// If project_id is Some(pid), returns only tasks for that project.
    pub fn get_tasks_by_project(&self, project_id: Option<&str>) -> Vec<&Task> {
        self.tasks
            .values()
            .filter(|t| {
                match project_id {
                    None => true, // Show all tasks
                    Some(pid) => t.project_id == pid,
                }
            })
            .collect()
    }

    pub fn get_tasks_by_project_mut(&mut self, project_id: Option<&str>) -> Vec<&mut Task> {
        self.tasks
            .values_mut()
            .filter(|t| {
                match project_id {
                    None => true, // Show all tasks
                    Some(pid) => t.project_id == pid,
                }
            })
            .collect()
    }

    pub fn find_task_by_remote_id(&self, remote_id: &str) -> Option<&Task> {
        self.tasks.values().find(|t| t.remote_id.as_deref() == Some(remote_id))
    }

    pub fn find_task_by_remote_id_mut(&mut self, remote_id: &str) -> Option<&mut Task> {
        self.tasks.values_mut().find(|t| t.remote_id.as_deref() == Some(remote_id))
    }

    pub fn find_project_by_remote_id(&self, remote_id: &str) -> Option<&Project> {
        self.projects.values().find(|p| p.remote_id.as_deref() == Some(remote_id))
    }

    pub fn find_project_by_remote_id_mut(&mut self, remote_id: &str) -> Option<&mut Project> {
        self.projects.values_mut().find(|p| p.remote_id.as_deref() == Some(remote_id))
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

    pub fn add_project(&mut self, project: Project) {
        self.projects.insert(project.id.clone(), project);
    }

    pub fn remove_project(&mut self, project_id: &str) -> Option<Project> {
        self.projects.remove(project_id)
    }

    pub fn get_project(&self, project_id: &str) -> Option<&Project> {
        self.projects.get(project_id)
    }

    pub fn get_project_mut(&mut self, project_id: &str) -> Option<&mut Project> {
        self.projects.get_mut(project_id)
    }

    pub fn get_projects_sorted(&self) -> Vec<&Project> {
        let mut projects: Vec<_> = self.projects.values().collect();
        projects.sort_by(|a, b| a.created_at.cmp(&b.created_at));
        projects
    }
}
