use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
  buffer::Buffer,
  layout::Rect,
  widgets::{Block, Paragraph, Widget},
  DefaultTerminal,
};
use std::io::{self, Read};

use crate::{
  help::{draw_help, HELP_LINES},
  info::get_detail_string,
  pcapng::PngBlock,
};
use crate::{hexview::HexView, pcapng::parse};

pub struct App {
  data: Vec<Box<dyn PngBlock>>,
  hexview: HexView,
  exit: bool,
  path: std::path::PathBuf,
  help: bool,
}

impl App {
  pub fn new(path: std::path::PathBuf) -> std::io::Result<App> {
    let f = std::fs::File::open(&path).expect("Failed to open file");
    let mut reader = std::io::BufReader::new(f);
    let mut raw: Vec<u8> = vec![];
    reader.read_to_end(&mut raw)?;
    let application = App {
      data: parse(&raw),
      hexview: HexView::default(),
      exit: false,
      path,
      help: false,
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
      KeyCode::Char('h') => self.hexview.left(),
      KeyCode::Char('l') => self.hexview.right(),
      KeyCode::Char('G') => self.hexview.bottom(),
      KeyCode::Char('g') => self.hexview.top(),
      KeyCode::Char('d') => {
        if key_event.modifiers.contains(KeyModifiers::CONTROL) {
          self.hexview.down_half()
        }
      }
      KeyCode::Char('u') => {
        if key_event.modifiers.contains(KeyModifiers::CONTROL) {
          self.hexview.up_half()
        }
      }
      KeyCode::Char('f') => self.hexview.fold(),
      KeyCode::Char('a') => self.hexview.toggle_ascii(),
      KeyCode::Char('?') => self.toggle_help(),
      _ => (),
    }
  }

  fn draw(&mut self, area: Rect, buf: &mut Buffer) {
    Paragraph::new(
      self
        .path
        .file_name()
        .expect("Error") // TODO: Error handling
        .to_str()
        .unwrap_or("Invalid unicode in path")
        .to_owned()
        + std::format!(" | {} Packets", self.data.len()).as_str(),
    )
    .block(Block::bordered())
    .render(
      Rect {
        height: 3,
        width: area.width - 11,
        ..area
      },
      buf,
    );

    Paragraph::new(" Help: ?").block(Block::bordered()).render(
      Rect {
        height: 3,
        width: 11,
        x: area.width - 11,
        ..area
      },
      buf,
    );

    let hex_area = Rect {
      y: area.y + 6,
      height: area.height - 6,
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

    let (id, cursor_y) = self.hexview.id_under_cursor();
    Paragraph::new(get_detail_string(
      &self.data[id as usize],
      hex_area.width - 2,
      (self.hexview.cursor.0, cursor_y),
    ))
    .block(Block::bordered())
    .render(
      Rect {
        height: 3,
        width: area.width,
        y: 3,
        x: area.x,
      },
      buf,
    );

    if self.help {
      draw_help(
        Rect {
          x: (area.width - 40) / 2,
          y: (area.height - HELP_LINES) / 2,
          width: 40,
          height: HELP_LINES + 2,
        },
        buf,
      );
    }
  }

  fn toggle_help(&mut self) {
    self.help = !self.help;
  }
}
