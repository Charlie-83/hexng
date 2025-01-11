use ratatui::{
  buffer::Buffer,
  layout::Rect,
  widgets::{Paragraph, Widget, Wrap},
};

pub struct PngBlock {
  pub raw: Vec<u8>,
  pub block_type: u32,
  pub length: u32,
  pub options: Vec<u8>,
}

pub fn parse(data: &Vec<u8>) -> Vec<PngBlock> {
  let mut out: Vec<PngBlock> = vec![];
  let mut pos: usize = 0;
  while pos < data.len() {
    let block_type = u32::from_le_bytes(data[pos..pos + 4].try_into().unwrap());
    let length = u32::from_le_bytes(data[pos + 4..pos + 8].try_into().unwrap());
    out.push(PngBlock {
      raw: data[pos..pos + (length as usize)].to_vec(),
      block_type,
      length,
      options: vec![],
    });
    pos += length as usize;
  }
  out
}

impl PngBlock {
  pub fn draw(&self, area: Rect, buf: &mut Buffer, offset: u16) -> u16 {
    let bytes_in_row = (area.width + 1) / 3;
    let mut total_rows = self.raw.len() as u16 / bytes_in_row;
    if self.raw.len() as u16 % bytes_in_row != 0 {
      total_rows += 1;
    }
    if offset >= total_rows {
      return 0;
    }
    let rows_to_print = std::cmp::min(total_rows - offset, area.height);
    let start: usize = (offset * bytes_in_row) as usize;
    let end: usize = std::cmp::min(
      ((offset + rows_to_print) * bytes_in_row) as usize,
      self.raw.len(),
    );
    let hex_strings = self.raw[start..end]
      .iter()
      .map(|b| format!("{:02x}", b))
      .collect::<Vec<_>>()
      .join(" ");

    Paragraph::new(hex_strings)
      .wrap(Wrap { trim: true })
      .render(
        Rect {
          y: area.y,
          height: rows_to_print,
          ..area
        },
        buf,
      );
    rows_to_print
  }

  pub fn rows(&self, width: u16) -> u16 {
    let bytes_in_row = (width + 1) / 3;
    self.raw.len() as u16 / bytes_in_row
  }
}
