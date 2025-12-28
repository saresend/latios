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
    pub fn get_tasks_by_project(&self, project_id: Option<&str>) -> Vec<&Task> {
        self.tasks
            .values()
            .filter(|t| {
                match (project_id, &t.project_id) {
                    (None, None) => true,
                    (Some(pid), Some(tpid)) => pid == tpid,
                    _ => false,
                }
            })
            .collect()
    }

    pub fn get_tasks_by_project_mut(&mut self, project_id: Option<&str>) -> Vec<&mut Task> {
        self.tasks
            .values_mut()
            .filter(|t| {
                match (project_id, &t.project_id) {
                    (None, None) => true,
                    (Some(pid), Some(tpid)) => pid == tpid,
                    _ => false,
                }
            })
            .collect()
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
}
