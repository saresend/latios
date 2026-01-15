use crossterm::event::{Event, KeyCode};
use ratatui::DefaultTerminal;
use ratatui::prelude::*;
use ratatui::widgets::FrameExt;
use ratatui::widgets::{List, WidgetRef};

mod latios_types;

use anyhow::Result;
use latios_types::*;

struct App {
    curr_app_state: Box<dyn AppState>,
    is_terminal: bool,
}

impl App {
    fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        color_eyre::install().unwrap();
        while !self.is_terminal {
            terminal.draw(|frame| {
                self.curr_app_state
                    .render_ref(frame.area(), frame.buffer_mut())
            });
            let keypress = match crossterm::event::read()? {
                Event::Key(key) => Some(key.code),
                _ => None,
            };
            if let Some(keypress) = keypress {
                self.consume_event(keypress);
            }
        }
        Ok(())
    }

    fn consume_event(&mut self, keypress: KeyCode) {
        match keypress {
            KeyCode::Char('q') => self.is_terminal = true,
            _ => {}
        }
    }
}

impl WidgetRef for App {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        self.curr_app_state.render_ref(area, buf);
    }
}

struct Home {
    workstreams: Vec<Workstream>,
    curr_workstream: usize,
}

impl WidgetRef for Home {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(10), Constraint::Min(50)])
            .split(area);

        "Current Workstreams".render(layout[0], buf);
        let list = List::new(self.workstreams.iter().map(|wstream| wstream.get_line()));
        Widget::render(list, layout[1], buf);
    }
}
impl AppState for Home {}

fn main() {
    let sample_workstreams = vec![
        Workstream::new("Invoke claude", WorkstreamType::Claude),
        Workstream::new("Invoke Codex", WorkstreamType::Codex),
    ];

    let terminal = ratatui::init();
    let mut app = App {
        curr_app_state: Box::new(Home {
            workstreams: sample_workstreams,
            curr_workstream: 0,
        }),
        is_terminal: false,
    };
    let result = app.run(terminal);
    ratatui::restore();
}
