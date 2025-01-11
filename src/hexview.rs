use ratatui::prelude::{Buffer, Rect};

use crate::pcapng::PngBlock;

#[derive(Default)]
pub struct HexView {
  top_block: usize,
  pos: u16,
}

impl HexView {
  pub fn draw(&mut self, mut area: Rect, buf: &mut Buffer, data: &Vec<PngBlock>) {
    let mut offset = self.pos;
    for block in &data[self.top_block as usize..] {
      let rows = block.draw(area, buf, offset);
      if rows == 0 {
        self.top_block += 1;
        self.pos = 0;
      } else {
        if area.height - rows <= 1 {
          break;
        } else {
          area.y += rows + 1;
          area.height -= rows + 1;
        }
      }
      offset = 0;
    }
  }

  pub fn down(&mut self) {
    self.pos += 1;
  }

  pub fn up(&mut self) {
    self.pos = self.pos.saturating_sub(1);
    self.top_block = self.top_block.saturating_sub(1);
  }
}
