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
    symbols::border,
    text::{Line, Text},
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
        let base_block = Block::bordered().title(title);
        base_block.render(area, buf);
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct LatiosApp {
    workstreams: Vec<Workstream>,
    exit: bool,
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
        let title = Span::styled(
            " Latios - Workstream management",
            Style::default().fg(Color::LightBlue).bg(Color::DarkGray),
        );
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([Constraint::Max(3), Constraint::Min(40)]);

        let sections = layout.split(area);
        let workstream_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(self.workstreams.iter().map(|x| Constraint::Max(5)));
        title.render(sections[0], buf);
        let stream_item = workstream_layout.split(sections[1]);
        let title = Line::from("Workstreams: ".bold());

        for (i, workstream) in self.workstreams.iter().enumerate() {
            workstream.render(stream_item[i], buf);
        }
    }
}
