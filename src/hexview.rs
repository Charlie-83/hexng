use std::{
  cmp::min,
  collections::{HashMap, HashSet},
};

use ratatui::{
  prelude::{Buffer, Rect},
  style::{Style, Stylize},
  widgets::Block,
  widgets::Widget,
};

use crate::pcapng::{self, PngBlock};

#[derive(Default)]
pub struct HexView {
  pos: u32, // Number of lines of the top block that are hidden
  pub cursor: (u16, u16),
  area: Rect,
  block_areas: Vec<(u32, u16)>,
  folded: HashSet<u32>,
  row_counts: HashMap<u32, u16>,
  ascii: bool,
}

impl HexView {
  pub fn draw(&mut self, mut area: Rect, buf: &mut Buffer, data: &Vec<Box<dyn PngBlock>>) {
    if self.area != area {
      // Area changed
      if self.area.width != area.width {
        self.row_counts.clear()
      }
      self.area = area;
      self.cursor.0 = min(self.cursor.0, area.width - 1);
      self.cursor.1 = min(self.cursor.1, area.height - 1);
      for block in data {
        self.row_counts.insert(block.id(), block.rows(area.width));
      }
    }
    self.block_areas.clear();
    let mut current_pos: u32 = 0;
    for (_, block) in data.iter().enumerate() {
      let rows;
      if self.folded.contains(&block.id()) {
        rows = 1;
      } else {
        rows = self.row_counts[&block.id()];
      }
      if current_pos + (rows as u32) <= self.pos {
        if current_pos + (rows as u32) == self.pos {
          area.y += 1;
          area.height -= 1;
          current_pos += rows as u32;
        } else {
          current_pos += rows as u32 + 1;
        }
        continue;
      }

      let mut hidden = 0;
      if current_pos < self.pos {
        hidden = self.pos - current_pos;
      }
      current_pos += rows as u32 + 1;

      let rows_drawn = pcapng::draw_block(
        block,
        area,
        buf,
        hidden as u16,
        self.folded.contains(&block.id()),
        self.ascii,
      );
      self.block_areas.push((block.id(), rows_drawn));
      if area.height <= 2 + rows_drawn {
        // Block has filled the remaining area
        break;
      } else {
        area.y += rows_drawn + 1;
        area.height -= rows_drawn + 1;
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
      self.pos += 1;
    } else {
      self.cursor.1 += 1;
    }
  }

  pub fn down_half(&mut self) {
    self.pos += (self.area.height / 2) as u32;
  }

  pub fn up(&mut self) {
    if self.cursor.1 == 0 {
      self.pos = self.pos.saturating_sub(1);
    } else {
      self.cursor.1 -= 1;
    }
  }

  pub fn up_half(&mut self) {
    self.pos = self.pos.saturating_sub((self.area.height / 2) as u32);
  }

  pub fn left(&mut self) {
    self.cursor.0 = self.cursor.0.saturating_sub(1);
  }

  pub fn right(&mut self) {
    self.cursor.0 = min(self.cursor.0 + 1, self.area.width - 1);
  }

  pub fn bottom(&mut self) {
    let last_id = self.row_counts.keys().max().unwrap();
    self.pos = self.get_block_pos(last_id);
  }

  pub fn top(&mut self) {
    self.pos = 0;
  }

  pub fn id_under_cursor(&self) -> (u32, u16) {
    let mut cursor_y = self.cursor.1;
    for (id, area) in &self.block_areas {
      if &cursor_y > area {
        cursor_y -= area + 1;
      } else {
        return (*id, cursor_y);
      }
    }
    assert!(false);
    (0, 0)
  }

  pub fn fold(&mut self) {
    let (id, _) = self.id_under_cursor();
    if self.folded.contains(&id) {
      self.folded.remove(&id);
    } else {
      self.folded.insert(id);
    }
  }

  fn get_block_pos(&self, id: &u32) -> u32 {
    let mut pos = 0;
    for i in 0..*id {
      pos += self.row_counts[&i] as u32 + 1;
    }
    return pos;
  }

  pub fn toggle_ascii(&mut self) {
    self.ascii = !self.ascii;
  }
}
