use ratatui::{
  prelude::{Buffer, Rect},
  widgets::{Block, Paragraph, Widget, Wrap},
  Frame,
};

#[derive(Default)]
pub struct HexView {
  pos: usize,
  row_bytes: u16,
}

impl HexView {
  pub fn draw(&mut self, frame: &mut Frame, data: &[u8]) {
    let area = frame.area();
    self.row_bytes = ( area.width + 1 ) / 3;

    let chars = (self.row_bytes * area.height) as usize;
    let hex_string: String = data[self.pos..chars + self.pos]
      .iter()
      .map(|b| format!("{:02x}", b))
      .collect::<Vec<_>>()
      .join(" ");

    frame.render_widget(
      Paragraph::new(hex_string)
        .wrap(Wrap { trim: true }),
      area,
    )
  }

  pub fn down(&mut self) {
    self.pos += self.row_bytes as usize;
  }

  pub fn up(&mut self) {
    self.pos = self.pos.saturating_sub(self.row_bytes as usize);
  }
}
