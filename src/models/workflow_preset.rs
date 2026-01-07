/// Hardcoded workflow presets.
/// Each preset defines a command that will be executed via `bash -c "<command>"`.
/// Users select a preset when creating a new workstream.

#[derive(Debug, Clone)]
pub struct WorkflowPreset {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub command: &'static str,
}

/// Get all available workflow presets
pub fn get_all_presets() -> Vec<WorkflowPreset> {
    vec![
        WorkflowPreset {
            id: "default",
            name: "Default Workspace",
            description: "Opens a new wezterm window with neovim",
            command: "wezterm start -- nvim",
        },
        WorkflowPreset {
            id: "opencode",
            name: "OpenCode Session",
            description: "Opens wezterm with opencode CLI",
            command: "wezterm start -- opencode",
        },
        WorkflowPreset {
            id: "dev-server",
            name: "Development Server",
            description: "Opens wezterm and runs a dev server (placeholder)",
            command: "wezterm start -- bash -c 'echo \"Dev server placeholder - customize this command\"'",
        },
        WorkflowPreset {
            id: "split-nvim-term",
            name: "Neovim + Terminal Split",
            description: "Opens wezterm with neovim in a split layout",
            command: "wezterm start -- bash -c 'nvim'",
        },
        WorkflowPreset {
            id: "custom",
            name: "Custom Command",
            description: "Placeholder for custom user command",
            command: "wezterm start -- bash",
        },
    ]
}

/// Get a preset by ID
pub fn get_preset_by_id(id: &str) -> Option<WorkflowPreset> {
    get_all_presets().into_iter().find(|p| p.id == id)
}

/// Get the default preset
pub fn get_default_preset() -> WorkflowPreset {
    get_preset_by_id("default").expect("Default preset should always exist")
}
