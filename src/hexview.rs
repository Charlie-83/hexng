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
  pos: i16,
  cursor: (u16, u16),
  area: Rect,
  packet_areas: Vec<u16>,
  folded: HashSet<usize>,
}

impl HexView {
  pub fn draw(&mut self, mut area: Rect, buf: &mut Buffer, data: &Vec<PngBlock>) {
    if self.area != area {
      self.area = area;
      self.cursor.0 = min(self.cursor.0, area.width - 1);
      self.cursor.1 = min(self.cursor.1, area.height - 1);
    }
    let mut offset: u16;
    if self.pos < 0 {
      self.top_block -= 1;
      self.pos = data[self.top_block].rows(area.width) as i16;
    }
    offset = self.pos as u16;
    self.packet_areas.clear();
    for (i, block) in data[self.top_block as usize..].iter().enumerate() {
      let rows = block.draw(
        area,
        buf,
        offset,
        self.folded.contains(&(self.top_block + i)),
      );
      if rows == 0 {
        self.top_block += 1;
        self.pos = 0;
      } else {
        self.packet_areas.push(rows);
        if area.height - rows <= 2 {
          break;
        } else {
          area.y += rows + 1;
          area.height -= rows + 1;
        }
      }
      offset = 0;
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
    self.cursor.1 += 1;
    if self.cursor.1 >= self.area.height {
      self.pos += 1;
      self.cursor.1 -= 1;
    }
  }

  pub fn down_half(&mut self) {
    for _ in 0..self.area.height / 2 {
      self.down()
    }
  }

  pub fn up(&mut self) {
    if self.cursor.1 == 0 {
      self.pos -= 1;
      if self.top_block == 0 && self.pos == -1 {
        self.pos = 0;
      }
    } else {
      self.cursor.1 -= 1;
    }
  }

  pub fn up_half(&mut self) {
    for _ in 0..self.area.height / 2 {
      self.up()
    }
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
    while cursor_y > self.packet_areas[i] {
      cursor_y -= self.packet_areas[i] + 1;
      i += 1;
    }
    if self.folded.contains(&i) {
      self.folded.remove(&i);
    } else {
      self.folded.insert(i);
    }
  }
}
