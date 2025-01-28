use crate::util::div_ceil;
use ratatui::{
  buffer::Buffer,
  layout::Rect,
  style::{Color, Stylize},
  text::{Line, Span},
  widgets::{Paragraph, Widget, Wrap},
};

pub struct PngBlock {
  pub id: u32,
  pub raw: Vec<u8>,
  pub block_type: u32,
  pub length: u32,
  pub options: Vec<u8>,
}

pub fn parse(data: &Vec<u8>) -> Vec<PngBlock> {
  let mut out: Vec<PngBlock> = vec![];
  let mut pos: usize = 0;
  let mut id: u32 = 0;
  while pos < data.len() {
    let block_type = u32::from_le_bytes(data[pos..pos + 4].try_into().unwrap());
    let length = u32::from_le_bytes(data[pos + 4..pos + 8].try_into().unwrap());
    out.push(PngBlock::new(
      data[pos..pos + (length as usize)].to_vec(),
      block_type,
      length,
      vec![],
      id,
    ));
    id += 1;
    pos += length as usize;
  }
  out
}

impl PngBlock {
  fn new(raw: Vec<u8>, block_type: u32, length: u32, options: Vec<u8>, id: u32) -> PngBlock {
    PngBlock {
      raw,
      block_type,
      length,
      options,
      id,
    }
  }

  pub fn draw(&self, mut area: Rect, buf: &mut Buffer, hidden: u16, folded: bool) -> u16 {
    let bytes_in_row = (area.width + 1) / 3;
    let total_rows = div_ceil(self.length as u16, bytes_in_row) + 1;
    assert!(hidden < total_rows);

    let total_rows_to_print = std::cmp::min(total_rows - hidden, area.height);
    if total_rows_to_print == 0 {
      return 0;
    }
    let mut rows_to_print = total_rows_to_print;

    Line::raw(self.id.to_string() + ": " + &block_type_str(self.block_type))
      .underlined()
      .bold()
      .render(area, buf);
    if folded || rows_to_print == 1 {
      return 1;
    }

    area.y += 1;
    area.height -= 1;
    rows_to_print -= 1;

    let start: usize = (hidden * bytes_in_row) as usize;
    let end: usize =
      std::cmp::min((hidden + rows_to_print) * bytes_in_row, self.length as u16) as usize;
    let mut spans = vec![];
    let mut current_section = 0;
    let mut index = 0;
    let mut fg_colour_index = 0;
    let mut bg_colour_index = 0;
    let fg_colours = [
      Color::White,
      Color::Red,
      Color::Green,
      Color::Magenta,
      Color::LightBlue,
    ];
    let bg_colours = [Color::Black, Color::DarkGray];
    while index < end {
      let section = self.sections()[current_section];
      if index < start && index + section > start {
        spans.push(
          Span::raw(to_hex(&self.raw[start..index + section]))
            .fg(fg_colours[fg_colour_index])
            .bg(bg_colours[bg_colour_index]),
        );
      } else if index > start && index + section <= end {
        spans.push(
          Span::raw(to_hex(&self.raw[index..index + section]))
            .fg(fg_colours[fg_colour_index])
            .bg(bg_colours[bg_colour_index]),
        );
      } else if index > start && index + section > end {
        spans.push(
          Span::raw(to_hex(&self.raw[index..end]))
            .fg(fg_colours[fg_colour_index])
            .bg(bg_colours[bg_colour_index]),
        );
      }
      fg_colour_index = (fg_colour_index + 1) % fg_colours.len();
      bg_colour_index = (bg_colour_index + 1) % bg_colours.len();
      index += section;
      current_section += 1;
    }

    for i in 0..spans.len() {
      spans.insert(2 * i + 1, Span::raw(" "));
    }

    Paragraph::new(Line::from(spans))
      .wrap(Wrap { trim: true })
      .render(area, buf);
    total_rows_to_print
  }

  pub fn rows(&self, width: u16) -> u16 {
    let bytes_in_row = (width + 1) / 3;
    div_ceil(self.length as u16, bytes_in_row) + 1
  }

  fn sections(&self) -> Vec<usize> {
    match self.block_type {
      0x00000006 => vec![4, 4, 4, 4, 4, 4, 4, self.length as usize - 32, 4],
      _ => vec![4, 4, self.length as usize - 12, 4],
    }
  }
}

fn to_hex(s: &[u8]) -> String {
  s.iter()
    .map(|b| format!("{:02x}", b))
    .collect::<Vec<_>>()
    .join(" ")
}

fn block_type_str(block_type: u32) -> String {
  match block_type {
    0x00000001 => "Interface Description Block".to_owned(),
    0x00000002 => "Packet Block".to_owned(),
    0x00000003 => "Simple Packet Block".to_owned(),
    0x00000004 => "Name Resolution Block".to_owned(),
    0x00000005 => "Interface Statistics Block".to_owned(),
    0x00000006 => "Enhanced Packet Block".to_owned(),
    0x00000007 => "IRIG Timestamp/Socket Aggregation Event Block".to_owned(),
    0x00000008 => "AFDX Encapsulation Information Block".to_owned(),
    0x00000009 => "systemd Journal Export Block".to_owned(),
    0x0000000a => "Decryption Secrets Block".to_owned(),
    0x00000101 => "Hone Project Machine Info Block".to_owned(),
    0x00000102 => "Hone Project Connection Event Block".to_owned(),
    0x00000201 => "Sysdig Machine Info Block".to_owned(),
    0x00000202 => "Sysdig Process Info Block, version 1".to_owned(),
    0x00000203 => "Sysdig FD List Block".to_owned(),
    0x00000204 => "Sysdig Event Block".to_owned(),
    0x00000205 => "Sysdig Interface List Block".to_owned(),
    0x00000206 => "Sysdig User List Block".to_owned(),
    0x00000207 => "Sysdig Process Info Block, version 2".to_owned(),
    0x00000208 => "Sysdig Event Block with flags".to_owned(),
    0x00000209 => "Sysdig Process Info Block, version 3".to_owned(),
    0x00000210 => "Sysdig Process Info Block, version 4".to_owned(),
    0x00000211 => "Sysdig Process Info Block, version 5".to_owned(),
    0x00000212 => "Sysdig Process Info Block, version 6".to_owned(),
    0x00000213 => "Sysdig Process Info Block, version 7".to_owned(),
    0x00000BAD => "Custom Block that rewriters can copy into new files".to_owned(),
    0x40000BAD => "Custom Block that rewriters should not copy into new files".to_owned(),
    0x0A0D0D0A => "Section Header Block".to_owned(),
    _ => "Unknown".to_owned(),
  }
}
