pub mod task_list;
pub mod task_detail;
pub mod project_detail;
pub mod project_list;
pub mod help;

use ratatui::Frame;
use crate::app::App;
use crate::models::AppView;

pub fn render(f: &mut Frame, app: &App) {
    match app.view_state.current_view {
        AppView::TaskList => task_list::render(f, app, f.area()),
        AppView::TaskDetail => task_detail::render(f, app, f.area()),
        AppView::ProjectDetail => project_detail::render(f, app, f.area()),
        AppView::ProjectList => project_list::render(f, app, f.area()),
        AppView::Help => help::render(f, app, f.area()),
    }
}
