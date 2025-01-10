use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};
use std::io::{self, Read};

use crate::hexview::HexView;

pub struct App {
  data: Vec<u8>,
  hexview: HexView,
  exit: bool,
}

impl App {
  pub fn new(path: std::path::PathBuf) -> std::io::Result<App> {
    let f = std::fs::File::open(&path).expect("Failed to open file");
    let mut reader = std::io::BufReader::new(f);
    let mut data: Vec<u8> = vec![];
    reader.read_to_end(&mut data)?;
    let application = App {
      data,
      hexview: HexView::default(),
      exit: false,
    };
    Ok(application)
  }

  pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
    while !self.exit {
      terminal.draw(|frame| self.draw(frame))?;
      self.handle_events()?;
    }
    Ok(())
  }

  fn handle_events(&mut self) -> io::Result<()> {
    match event::read()? {
      Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
        self.handle_key_event(key_event)
      }
      _ => {}
    }
    Ok(())
  }

  fn handle_key_event(&mut self, key_event: KeyEvent) {
    match key_event.code {
      KeyCode::Char('q') => self.exit = true,
      KeyCode::Char('j') => self.hexview.down(),
      KeyCode::Char('k') => self.hexview.up(),
      _ => (),
    }
  }

  fn draw(&mut self, frame: &mut Frame) {
    self.hexview.draw(frame, &self.data);
  }
}
