use ratatui::DefaultTerminal;
use ratatui::prelude::*;
use ratatui::widgets::*;
use ratatui::widgets::{List, WidgetRef};
use std::io::ErrorKind;
use std::path::PathBuf;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, read};
use ratatui::{
    Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    text::Line,
    widgets::{Block, Paragraph, Widget},
};

use anyhow::Result;
use serde::{Deserialize, Serialize};

fn main() -> Result<()> {
    color_eyre::install();
    let mut app = LatiosApp::default();
    app.load()?;

    let mut terminal = ratatui::init();
    app.run(&mut terminal)?;
    ratatui::restore();
    Ok(())
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Workstream {
    title: String,
    needs_attention: bool,
    spec_file: PathBuf,
    highlight: bool,
}

impl Widget for &Workstream {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let title = Line::from(self.title.clone());
        let subtitle = Line::from(self.spec_file.file_name().unwrap().to_str().unwrap());
        let block_style = match self.highlight {
            true => Style::default().fg(Color::LightBlue),
            false => Style::default(),
        };
        let base_block = Block::bordered().title(title).style(block_style);
        subtitle.render(base_block.inner(area), buf);
        base_block.render(area, buf);
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct NewWorkstream {
    title: String,
    spec_file: String,
}

impl NewWorkstream {
    fn get_default_spec_location(&self) -> String {
        format!("~/.latios/specs/{}.md", self.title)
    }
}

impl Widget for &NewWorkstream {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Max(5), Constraint::Max(5)]);
        let sections = layout.split(area);

        let title_block = Block::bordered().title("Workstream Name");

        self.title
            .clone()
            .render(title_block.inner(sections[0]), buf);
        title_block.render(sections[0], buf);

        let spec_location_block = Block::bordered().title("Spec file path");

        let spec_file = if self.spec_file.is_empty() {
            self.get_default_spec_location()
        } else {
            self.spec_file.clone()
        };
        spec_file.render(spec_location_block.inner(sections[1]), buf);
        spec_location_block.render(sections[1], buf);
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct LatiosApp {
    workstreams: Vec<Workstream>,
    exit: bool,
    curr_selected: usize,
    is_creating_workstream: bool,
}

impl LatiosApp {
    pub fn load(&mut self) -> Result<()> {
        let path = workstreams_path()?;
        let contents = match std::fs::read_to_string(&path) {
            Ok(contents) => contents,
            Err(err) if err.kind() == ErrorKind::NotFound => {
                self.workstreams.clear();
                return Ok(());
            }
            Err(err) => return Err(err.into()),
        };

        if contents.trim().is_empty() {
            self.workstreams.clear();
            return Ok(());
        }

        self.workstreams = serde_json::from_str(&contents)?;
        Ok(())
    }

    pub fn change_selected(&mut self, delta: isize) {
        if self.workstreams.is_empty() {
            // noop if empty workstreamset
            return;
        }
        self.workstreams[self.curr_selected].highlight = false;
        let amt = self.curr_selected as isize + delta;
        self.curr_selected =
            std::cmp::max(0, std::cmp::min((self.workstreams.len() - 1) as isize, amt)) as usize;
        self.workstreams[self.curr_selected].highlight = true;
    }
    pub fn open_new_workstream_view(&mut self) {
        self.is_creating_workstream = true;
    }

    pub fn handle_escape(&mut self) {
        if self.is_creating_workstream {
            self.is_creating_workstream = false;
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Char('q') => self.exit = true,
                    KeyCode::Char('j') => self.change_selected(1),
                    KeyCode::Char('k') => self.change_selected(-1),
                    KeyCode::Char('a') => self.open_new_workstream_view(),
                    KeyCode::Esc => self.handle_escape(),
                    _ => {}
                }
            }
            _ => {}
        }
        Ok(())
    }
}

fn workstreams_path() -> Result<PathBuf> {
    let home_dir = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Home directory not found"))?;
    Ok(home_dir.join(".latios/workstreams.json"))
}

impl Widget for &LatiosApp {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let mut constraint_set = vec![Constraint::Max(3), Constraint::Min(40)];
        if self.is_creating_workstream {
            constraint_set.push(Constraint::Max(10));
        }

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraint_set);

        let sections = layout.split(area);
        let title = Span::styled(
            " Latios - Workstream management",
            Style::default().fg(Color::LightBlue).bg(Color::DarkGray),
        );

        let workstream_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(self.workstreams.iter().map(|x| Constraint::Max(3)));
        title.render(sections[0], buf);
        let stream_item = workstream_layout.split(sections[1]);

        for (i, workstream) in self.workstreams.iter().enumerate() {
            workstream.render(stream_item[i], buf);
        }

        if self.is_creating_workstream {
            let new_workstream_widget = NewWorkstream::default();
            new_workstream_widget.render(sections[2], buf);
        }
    }
}
