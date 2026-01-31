use ratatui::DefaultTerminal;
use ratatui::prelude::*;
use ratatui::widgets::*;
use ratatui::widgets::{List, WidgetRef};
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

fn main() -> Result<()> {
    color_eyre::install();
    let mut terminal = ratatui::init();
    let mut app = LatiosApp::default();
    app.run(&mut terminal)?;
    ratatui::restore();
    Ok(())
}

#[derive(Debug, PartialEq)]
pub struct Workstream {
    title: String,
    needs_attention: bool,
    spec_file: PathBuf,
}

#[derive(Debug, PartialEq, Default)]
pub struct LatiosApp {
    workstreams: Vec<Workstream>,
    exit: bool,
}

impl LatiosApp {
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

impl Widget for &LatiosApp {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let title = Line::from(" Latios - Agentic Workstream management".bold());
        let block = Block::bordered().title(title.centered());

        block.render(area, buf);
    }
}
