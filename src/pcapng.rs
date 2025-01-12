use crate::util::div_ceil;
use ratatui::{
  buffer::Buffer,
  layout::Rect,
  style::Stylize,
  text::{Line, Span},
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
  pub fn draw(&self, mut area: Rect, buf: &mut Buffer, hidden: u16) -> u16 {
    let bytes_in_row = (area.width + 1) / 3;
    let total_rows = div_ceil(self.raw.len() as u16, bytes_in_row) + 1;

    let total_rows_to_print = std::cmp::min(total_rows - hidden, area.height);
    if total_rows_to_print <= 1 {
      return 0;
    }
    let mut rows_to_print = total_rows_to_print;

    Line::raw(block_type_str(self.block_type))
      .underlined()
      .bold()
      .render(area, buf);
    area.y += 1;
    area.height -= 1;
    rows_to_print -= 1;

    let mut start: usize = (hidden * bytes_in_row) as usize;
    let end: usize = std::cmp::min(
      ((hidden + rows_to_print) * bytes_in_row) as usize,
      self.raw.len(),
    );
    let mut spans = vec![];
    if start < 4 {
      spans.push(Span::raw(to_hex(&self.raw[start..start + 4])).green());
      start += 4;
    }
    if start < 8 {
      spans.push(Span::raw(to_hex(&self.raw[start..start + 4])).red());
      start += 4;
    }
    let body_end = std::cmp::min(end, self.raw.len() - 4);
    spans.push(Span::raw(to_hex(&self.raw[start..body_end])));
    start = body_end;
    if end > self.raw.len() - 4 {
      spans.push(Span::raw(to_hex(&self.raw[start..end])).red())
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
    self.raw.len() as u16 / bytes_in_row
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
