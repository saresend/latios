use ratatui::DefaultTerminal;
use ratatui::prelude::*;
use ratatui::widgets::*;
use ratatui::widgets::{List, WidgetRef};
use reqwest::Client;
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

mod server;

use server::{ServerConfig, run};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install();
    let mut app = LatiosApp::default();
    app.load()?;

    let server_handle = tokio::spawn(async move { run(ServerConfig::default()).await });

    let mut terminal = ratatui::init();
    app.run(&mut terminal).await?;
    ratatui::restore();
    server_handle.abort();
    Ok(())
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Workstream {
    title: String,
    needs_attention: bool,
    spec_file: PathBuf,
    highlight: bool,
}

impl Workstream {
    fn new(title: String, spec_file: String) -> Self {
        let spec_file = PathBuf::from(spec_file);
        Self {
            title,
            needs_attention: false,
            spec_file,
            highlight: false,
        }
    }
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
        Paragraph::new(subtitle).block(base_block).render(area, buf);
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct NewWorkstream {
    title: String,
    spec_file: String,
    title_focused: bool,
}

impl NewWorkstream {
    fn get_default_spec_location(&self) -> String {
        format!("~/.latios/specs/{}.md", self.title)
    }

    fn toggle_focus(&mut self) {
        self.title_focused = !self.title_focused;
    }

    fn handle_char(&mut self, c: char) {
        if self.title_focused {
            self.title.push(c)
        } else {
            self.spec_file.push(c);
        }
    }

    fn handle_backspace(&mut self) {
        if self.title_focused {
            self.title.pop();
        } else {
            self.spec_file.pop();
        }
    }

    fn get_spec(&self) -> String {
        if self.spec_file.is_empty() {
            self.get_default_spec_location()
        } else {
            self.spec_file.clone()
        }
    }

    fn get_workstream(self) -> Workstream {
        let spec = self.get_spec();
        let title = self.title;
        Workstream::new(title, spec)
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct TextInput {
    title: String,
    text: String,
    cursor_pos: usize,
    style: Style,
}

impl TextInput {
    fn new(title: impl ToString, text: impl ToString, style: Style) -> Self {
        Self {
            title: title.to_string(),
            text: text.to_string(),
            cursor_pos: 0,
            style,
        }
    }
}

impl Widget for &TextInput {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let text_input = Paragraph::new(self.text.clone())
            .style(self.style)
            .block(Block::bordered().title(self.title.clone()));
        text_input.render(area, buf);
    }
}

impl Widget for &NewWorkstream {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Max(4), Constraint::Max(4)]);
        let sections = layout.split(area);

        let highlight_style = Style::default().fg(Color::Cyan);
        let base_style = Style::default().fg(Color::Gray);

        let workstream_name_input = TextInput::new(
            "Workstream Name",
            self.title.clone(),
            if self.title_focused {
                highlight_style
            } else {
                base_style
            },
        );
        workstream_name_input.render(sections[0], buf);

        let spec_file = self.get_spec();
        let spec_file_input = TextInput::new(
            "Spec Location",
            spec_file,
            if self.title_focused {
                base_style
            } else {
                highlight_style
            },
        );
        spec_file_input.render(sections[1], buf);
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct LatiosApp {
    workstreams: Vec<Workstream>,
    exit: bool,
    curr_selected: usize,
    new_workstream: Option<NewWorkstream>,
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
        self.new_workstream = Some(NewWorkstream::default())
    }

    pub fn handle_escape(&mut self) {
        if self.new_workstream.is_some() {
            self.new_workstream = None;
        }
    }

    pub fn handle_tab(&mut self) {
        if let Some(new_workstream) = self.new_workstream.as_mut() {
            new_workstream.toggle_focus()
        }
    }

    pub fn handle_backspace(&mut self) {
        if let Some(new_workstream) = self.new_workstream.as_mut() {
            new_workstream.handle_backspace();
        }
    }
    pub async fn handle_enter(&mut self) -> Result<()> {
        if let Some(new_workstream) = self.new_workstream.take() {
            let new_workstream = new_workstream.get_workstream();

            let http_client = Client::new();
            let session_input = server::SessionInput::new(
                new_workstream.title.clone(),
                new_workstream
                    .spec_file
                    .clone()
                    .to_str()
                    .unwrap()
                    .to_string(),
            );
            let result = http_client
                .post("http://localhost:8080/new")
                .json(&session_input)
                .send()
                .await?;
            self.workstreams.push(new_workstream)
        }
        Ok(())
    }

    pub fn handle_alphanum(&mut self, c: char) {
        if let Some(new_workstream) = self.new_workstream.as_mut() {
            new_workstream.handle_char(c);
        } else {
            match c {
                'q' => self.exit = true,
                'j' => self.change_selected(1),
                'k' => self.change_selected(-1),
                'a' => self.open_new_workstream_view(),
                _ => {}
            }
        }
    }

    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events().await?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
    async fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Char(x) => self.handle_alphanum(x),
                    KeyCode::Esc => self.handle_escape(),
                    KeyCode::Tab => self.handle_tab(),
                    KeyCode::Backspace => self.handle_backspace(),
                    KeyCode::Enter => {
                        let _ = self.handle_enter().await;
                    }
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
        if self.new_workstream.is_some() {
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

        if let Some(workstream) = self.new_workstream.as_ref() {
            workstream.render(sections[2], buf)
        }
    }
}
