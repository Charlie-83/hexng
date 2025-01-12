use std::{cmp::min, collections::HashSet};

use ratatui::{
  prelude::{Buffer, Rect},
  style::{Style, Stylize},
  widgets::Block,
  widgets::Widget,
};

use crate::pcapng::PngBlock;

#[derive(Default)]
pub struct HexView {
  top_block: usize,
  hidden: i16, // Number of lines of the top block that are hidden
  cursor: (u16, u16),
  area: Rect,
  block_areas: Vec<u16>,
  folded: HashSet<usize>,
}

impl HexView {
  pub fn draw(&mut self, mut area: Rect, buf: &mut Buffer, data: &Vec<PngBlock>) {
    if self.area != area {
      // Area changed
      self.area = area;
      self.cursor.0 = min(self.cursor.0, area.width - 1);
      self.cursor.1 = min(self.cursor.1, area.height - 1);
    }
    while self.hidden < 0 {
      if self.top_block != 0 {
        self.top_block -= 1;
        self.hidden = data[self.top_block].rows(area.width) as i16;
      } else {
        self.hidden = 0;
      }
    }
    self.block_areas.clear();
    for (i, block) in data[self.top_block as usize..].iter().enumerate() {
      let rows = block.draw(
        area,
        buf,
        self.hidden as u16,
        self.folded.contains(&(self.top_block + i)),
      );
      if rows == 0 {
        // Whole block above area
        self.top_block += 1;
        self.hidden -= block.rows(area.width) as i16;
        assert!(self.hidden >= 0)
      } else {
        self.block_areas.push(rows);
        if area.height - rows <= 2 {
          // Block has filled the remaining area
          break;
        } else {
          area.y += rows + 1;
          area.height -= rows + 1;
        }
      }
    }

    Block::default()
      .style(Style::new().black().on_white())
      .render(
        Rect {
          x: self.area.x + self.cursor.0,
          y: self.area.y + self.cursor.1,
          width: 1,
          height: 1,
        },
        buf,
      );
  }

  pub fn down(&mut self) {
    if self.cursor.1 >= self.area.height {
      self.hidden += 1;
    } else {
      self.cursor.1 += 1;
    }
  }

  pub fn down_half(&mut self) {
    self.hidden += (self.area.height / 2) as i16;
  }

  pub fn up(&mut self) {
    if self.cursor.1 == 0 {
      self.hidden -= 1;
    } else {
      self.cursor.1 -= 1;
    }
  }

  pub fn up_half(&mut self) {
    self.hidden -= (self.area.height / 2) as i16;
  }

  pub fn left(&mut self) {
    self.cursor.0 = self.cursor.0.saturating_sub(1);
  }

  pub fn right(&mut self) {
    self.cursor.0 = min(self.cursor.0 + 1, self.area.width - 1);
  }

  pub fn fold(&mut self) {
    let mut cursor_y = self.cursor.1;
    let mut i = self.top_block;
    while cursor_y > self.block_areas[i] {
      cursor_y -= self.block_areas[i] + 1;
      i += 1;
    }
    if self.folded.contains(&i) {
      self.folded.remove(&i);
    } else {
      self.folded.insert(i);
    }
  }
}
