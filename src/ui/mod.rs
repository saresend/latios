pub mod help;
pub mod preset_picker;
pub mod task_detail;
pub mod task_list;
pub mod workstream_detail;

use crate::app::App;
use crate::models::AppView;
use ratatui::Frame;

pub fn render(f: &mut Frame, app: &App) {
    match app.view_state.current_view {
        AppView::TaskList => task_list::render(f, app, f.area()),
        AppView::TaskDetail => task_detail::render(f, app, f.area()),
        AppView::WorkstreamDetail => workstream_detail::render(f, app, f.area()),
        AppView::PresetPicker => preset_picker::render(f, app, f.area()),
        AppView::Help => help::render(f, app, f.area()),
    }
}
