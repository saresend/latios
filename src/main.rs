use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io;

mod app;
mod models;
mod storage;
mod ui;
mod input;
mod markdown;

use app::App;

fn main() -> anyhow::Result<()> {
    // Determine data file path
    let data_path = get_data_file_path()?;

    // Initialize app and load data
    let mut app = App::new(data_path.clone());
    app.data = storage::json::load_data(&data_path)?;

    // Load PocketBase config and attempt sync from server
    let pb_config = storage::pocketbase::load_config().unwrap_or_default();
    app.sync_enabled = pb_config.enabled;

    if pb_config.enabled {
        let sync_result = storage::pocketbase::sync_from_server(&pb_config, &mut app.data);
        if let Some(err) = sync_result.error {
            app.startup_message = Some(format!("Sync: {}", err));
        }
    }

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run app
    let result = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    // Sync to server if enabled (before saving locally)
    if pb_config.enabled {
        let _ = storage::pocketbase::sync_to_server(&pb_config, &mut app.data);
    }

    // Always save data locally before exiting
    storage::json::save_data(&data_path, &app.data)?;

    result
}

fn run_app<B>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> anyhow::Result<()>
where
    B: ratatui::backend::Backend,
    <B as ratatui::backend::Backend>::Error: Send + Sync + 'static,
{
    loop {
        // Check status message timeout
        app.check_status_timeout();

        // Render UI
        terminal.draw(|f| ui::render(f, app))?;

        // Handle input
        input::handle_input(app)?;

        // Check exit condition
        if app.should_quit {
            break;
        }
    }
    Ok(())
}

fn get_data_file_path() -> anyhow::Result<String> {
    // Get home directory using dirs crate
    let home_dir = dirs::home_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;

    // Construct path: ~/.latios/tasks.json
    let data_path = home_dir
        .join(".latios")
        .join("tasks.json");

    Ok(data_path.to_string_lossy().to_string())
}
