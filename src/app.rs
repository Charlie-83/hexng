use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
  buffer::Buffer,
  layout::Rect,
  widgets::{Block, Paragraph, Widget},
  DefaultTerminal,
};
use std::{
  alloc::Layout,
  io::{self, Read},
};

use crate::hexview::HexView;

pub struct App {
  data: Vec<u8>,
  hexview: HexView,
  exit: bool,
  path: std::path::PathBuf,
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
      path,
    };
    Ok(application)
  }

  pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
    while !self.exit {
      terminal.draw(|frame| self.draw(frame.area(), frame.buffer_mut()))?;
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

  fn draw(&mut self, area: Rect, buf: &mut Buffer) {
    Paragraph::new(self.path.to_str().unwrap_or("Invalid unicode in path"))
      .block(Block::bordered())
      .render(Rect { height: 3, ..area }, buf);
    let hex_area = Rect {
      y: area.y + 3,
      height: area.height - 3,
      ..area
    };
    Block::bordered().render(hex_area, buf);
    self.hexview.draw(
      Rect {
        x: hex_area.x + 1,
        y: hex_area.y + 1,
        width: hex_area.width - 2,
        height: hex_area.height - 2,
      },
      buf,
      &self.data,
    );
  }
}
