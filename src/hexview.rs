use ratatui::{
  prelude::{Buffer, Rect},
  widgets::{Paragraph, Widget, Wrap},
};

#[derive(Default)]
pub struct HexView {
  pos: usize,
  row_bytes: u16,
}

impl HexView {
  pub fn draw(&mut self, area: Rect, buf: &mut Buffer, data: &[u8]) {
    self.row_bytes = (area.width + 1) / 3;

    let chars = (self.row_bytes * area.height) as usize;
    let hex_string: String = data[self.pos..chars + self.pos]
      .iter()
      .map(|b| format!("{:02x}", b))
      .collect::<Vec<_>>()
      .join(" ");

    Paragraph::new(hex_string)
      .wrap(Wrap { trim: true })
      .render(area, buf)
  }

  pub fn down(&mut self) {
    self.pos += self.row_bytes as usize;
  }

  pub fn up(&mut self) {
    self.pos = self.pos.saturating_sub(self.row_bytes as usize);
  }
}
