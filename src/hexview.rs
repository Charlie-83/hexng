use ratatui::{
  prelude::{Buffer, Rect},
  widgets::{Block, Paragraph, Widget, Wrap},
};

pub struct HexView {
    pub pos: usize,
}

impl HexView {
  pub fn draw(&self, area: Rect, buf: &mut Buffer, data: &[u8]) {
    let block = Block::bordered();
    let chars = (area.width * area.height) as usize;
    let hex_string: String = data[self.pos..chars + self.pos]
      .iter()
      .map(|b| format!("{:02x}", b))
      .collect::<Vec<_>>()
      .join(" ");

    Paragraph::new(hex_string)
      .block(block)
      .wrap(Wrap { trim: true })
      .render(area, buf);
  }

  pub fn down(&mut self) {
      self.pos += 1;
  }

  pub fn up(&mut self) {
      self.pos = self.pos.saturating_sub(1);
  }
}
