use crate::models::{AppData, Task};
use anyhow::{Context, Result};
use std::fs;

pub fn export_to_markdown(
    data: &AppData,
    output_path: &str,
    project_id: Option<&str>,
) -> Result<()> {
    let mut markdown = String::new();

    // Header
    markdown.push_str("# Task Context Export\n\n");
    markdown.push_str(&format!("Generated: {}\n\n", chrono::Utc::now().to_rfc3339()));

    // Project information
    if let Some(pid) = project_id {
        if let Some(project) = data.projects.get(pid) {
            markdown.push_str(&format!("## Project: {}\n\n", project.name));
            if !project.description.is_empty() {
                markdown.push_str(&format!("{}\n\n", project.description));
            }
        }
    } else {
        markdown.push_str("## All Tasks\n\n");
    }

    // Get tasks (filtered by project if specified)
    let mut tasks: Vec<&Task> = data.get_tasks_by_project(project_id);
    tasks.sort_by(|a, b| b.updated_at.cmp(&a.updated_at)); // Most recent first

    // Export each task
    for task in tasks {
        export_task_to_markdown(&mut markdown, task, data);
    }

    fs::write(output_path, markdown)
        .context(format!("Failed to write markdown export: {}", output_path))?;

    Ok(())
}

fn export_task_to_markdown(markdown: &mut String, task: &Task, data: &AppData) {
    // Task header
    let status = if task.completed { "✓" } else { " " };
    markdown.push_str(&format!("## [{}] {}\n\n", status, task.title));

    // Metadata
    markdown.push_str(&format!("**ID:** `{}`\n", task.id));
    markdown.push_str(&format!("**Status:** {}\n", if task.completed { "Completed" } else { "Pending" }));
    markdown.push_str(&format!("**Created:** {}\n", task.created_at));
    markdown.push_str(&format!("**Updated:** {}\n", task.updated_at));

    if let Some(completed) = &task.completed_at {
        markdown.push_str(&format!("**Completed:** {}\n", completed));
    }

    // Project
    if let Some(pid) = &task.project_id {
        if let Some(project) = data.projects.get(pid) {
            markdown.push_str(&format!("**Project:** {}\n", project.name));
        }
    }

    // Tags
    if !task.tags.is_empty() {
        markdown.push_str(&format!("**Tags:** {}\n", task.tags.join(", ")));
    }

    markdown.push_str("\n");

    // Description
    if !task.description.is_empty() {
        markdown.push_str("### Description\n\n");
        markdown.push_str(&task.description);
        markdown.push_str("\n\n");
    }

    // File references
    if !task.file_references.is_empty() {
        markdown.push_str("### File References\n\n");
        for file_ref in &task.file_references {
            if let Some(line) = file_ref.line_number {
                markdown.push_str(&format!("- `{}:{}`", file_ref.path, line));
            } else {
                markdown.push_str(&format!("- `{}`", file_ref.path));
            }
            if let Some(desc) = &file_ref.description {
                markdown.push_str(&format!(" - {}", desc));
            }
            markdown.push_str("\n");
        }
        markdown.push_str("\n");
    }

    // Code snippets
    if !task.code_snippets.is_empty() {
        markdown.push_str("### Code Snippets\n\n");
        for (i, snippet) in task.code_snippets.iter().enumerate() {
            if let Some(desc) = &snippet.description {
                markdown.push_str(&format!("**Snippet {}:** {}\n\n", i + 1, desc));
            }

            let lang = snippet.language.as_deref().unwrap_or("");
            markdown.push_str(&format!("```{}\n", lang));
            markdown.push_str(&snippet.code);
            markdown.push_str("\n```\n\n");
        }
    }

    markdown.push_str("---\n\n");
}
