use crate::models::AppData;
use super::client::PocketBaseClient;
use super::config::PocketBaseConfig;
use super::models::{PBTask, PBProject};
use std::collections::HashMap;

pub struct SyncResult {
    pub success: bool,
    pub tasks_synced: usize,
    pub projects_synced: usize,
    pub error: Option<String>,
}

impl SyncResult {
    pub fn skipped() -> Self {
        Self {
            success: true,
            tasks_synced: 0,
            projects_synced: 0,
            error: None,
        }
    }

    pub fn offline() -> Self {
        Self {
            success: false,
            tasks_synced: 0,
            projects_synced: 0,
            error: Some("Server unavailable, continuing offline".to_string()),
        }
    }

    pub fn error(e: anyhow::Error) -> Self {
        Self {
            success: false,
            tasks_synced: 0,
            projects_synced: 0,
            error: Some(e.to_string()),
        }
    }
}

/// Pull data from server on startup.
/// Strategy: Merge server data with local, populating remote_ids.
/// If item exists locally by local_id, update remote_id.
/// If item exists only on server, add locally.
pub fn sync_from_server(
    config: &PocketBaseConfig,
    local_data: &mut AppData,
) -> SyncResult {
    if !config.enabled {
        return SyncResult::skipped();
    }

    let client = PocketBaseClient::new(config);

    // Health check first
    if !client.health_check() {
        return SyncResult::offline();
    }

    // Fetch all server data
    let server_projects = match client.list_projects() {
        Ok(p) => p,
        Err(e) => return SyncResult::error(e),
    };

    let server_tasks = match client.list_tasks() {
        Ok(t) => t,
        Err(e) => return SyncResult::error(e),
    };

    let mut projects_synced = 0;
    let mut tasks_synced = 0;

    // Build a map of server project local_id -> remote_id
    let mut server_project_map: HashMap<String, String> = HashMap::new();
    for pb_project in &server_projects {
        if let Some(ref remote_id) = pb_project.id {
            server_project_map.insert(pb_project.local_id.clone(), remote_id.clone());
        }
    }

    // Sync projects: update local items with remote_id from server
    for pb_project in server_projects {
        if let Some(local_project) = local_data.projects.get_mut(&pb_project.local_id) {
            // Local project exists - update remote_id
            local_project.remote_id = pb_project.id.clone();
            projects_synced += 1;
        } else {
            // Project exists only on server - add locally
            let project = pb_project.into_project();
            local_data.projects.insert(project.id.clone(), project);
            projects_synced += 1;
        }
    }

    // Sync tasks: update local items with remote_id from server
    for pb_task in server_tasks {
        if let Some(local_task) = local_data.tasks.get_mut(&pb_task.local_id) {
            // Local task exists - update remote_id
            local_task.remote_id = pb_task.id.clone();
            tasks_synced += 1;
        } else {
            // Task exists only on server - add locally
            // Need to resolve project_local_id to project_id
            let project_id = pb_task.project_local_id.clone();
            let task = pb_task.into_task(project_id);
            local_data.tasks.insert(task.id.clone(), task);
            tasks_synced += 1;
        }
    }

    SyncResult {
        success: true,
        tasks_synced,
        projects_synced,
        error: None,
    }
}

/// Push data to server on quit.
/// Strategy: Local wins - push all local data to server.
/// For items with remote_id: update on server.
/// For items without remote_id: create on server.
pub fn sync_to_server(
    config: &PocketBaseConfig,
    local_data: &mut AppData,
) -> SyncResult {
    if !config.enabled {
        return SyncResult::skipped();
    }

    let client = PocketBaseClient::new(config);

    if !client.health_check() {
        return SyncResult::offline();
    }

    let mut projects_synced = 0;
    let mut tasks_synced = 0;
    let mut errors: Vec<String> = Vec::new();

    // First, get all server items to detect deletions
    let server_projects = client.list_projects().unwrap_or_default();
    let server_tasks = client.list_tasks().unwrap_or_default();

    // Build sets of local IDs
    let local_project_ids: std::collections::HashSet<_> =
        local_data.projects.keys().cloned().collect();
    let local_task_ids: std::collections::HashSet<_> =
        local_data.tasks.keys().cloned().collect();

    // Delete server items that don't exist locally
    for pb_project in &server_projects {
        if !local_project_ids.contains(&pb_project.local_id) {
            if let Some(ref remote_id) = pb_project.id {
                if let Err(e) = client.delete_project(remote_id) {
                    errors.push(format!("Failed to delete project: {}", e));
                }
            }
        }
    }

    for pb_task in &server_tasks {
        if !local_task_ids.contains(&pb_task.local_id) {
            if let Some(ref remote_id) = pb_task.id {
                if let Err(e) = client.delete_task(remote_id) {
                    errors.push(format!("Failed to delete task: {}", e));
                }
            }
        }
    }

    // Sync projects first (tasks depend on projects)
    for project in local_data.projects.values_mut() {
        let pb_project = PBProject::from(&*project);

        let result = if let Some(ref remote_id) = project.remote_id {
            // Update existing
            client.update_project(remote_id, &pb_project)
        } else {
            // Create new
            client.create_project(&pb_project)
        };

        match result {
            Ok(response) => {
                project.remote_id = response.id;
                projects_synced += 1;
            }
            Err(e) => {
                errors.push(format!("Failed to sync project '{}': {}", project.name, e));
            }
        }
    }

    // Sync tasks
    for task in local_data.tasks.values_mut() {
        let pb_task = PBTask::from(&*task);

        let result = if let Some(ref remote_id) = task.remote_id {
            // Update existing
            client.update_task(remote_id, &pb_task)
        } else {
            // Create new
            client.create_task(&pb_task)
        };

        match result {
            Ok(response) => {
                task.remote_id = response.id;
                tasks_synced += 1;
            }
            Err(e) => {
                errors.push(format!("Failed to sync task '{}': {}", task.title, e));
            }
        }
    }

    SyncResult {
        success: errors.is_empty(),
        tasks_synced,
        projects_synced,
        error: if errors.is_empty() { None } else { Some(errors.join("; ")) },
    }
}
