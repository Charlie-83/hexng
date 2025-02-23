use crate::baseblock::BaseBlock;
use crate::enhanced_packet::EnhancedPacket;
use crate::interface_description::InterfaceDescription;
use crate::loader::JTop;
use crate::section_header::SectionHeader;
use crate::types::{BlockTypes, LinkTypes};
use crate::util::div_ceil;
use ratatui::{buffer::Buffer, layout::Rect};
use ratatui::{
  style::{Color, Stylize},
  text::{Line, Span},
  widgets::{Paragraph, Widget, Wrap},
};

#[derive(Eq, PartialEq)]
pub enum BlockErrorKind {
  None,
  ZeroLength,
}

pub trait PngBlock {
  fn rows(&self, width: u16) -> u16;
  fn sections(&self) -> Vec<(String, usize)>;
  fn error(&self) -> &BlockErrorKind;
  fn id(&self) -> u32;
  fn length(&self) -> usize;
  fn block_type(&self) -> &BlockTypes;
  fn title_line(&self) -> String;
  fn raw(&self) -> &Vec<u8>;
}

fn box_up<T>(t: (T, usize)) -> (Box<dyn PngBlock>, usize)
where
  T: PngBlock + 'static,
{
  (Box::new(t.0), t.1)
}

pub fn parse(data: &Vec<u8>, config: JTop) -> Vec<Box<dyn PngBlock>> {
  let mut out: Vec<Box<dyn PngBlock>> = vec![];
  let mut interfaces: Vec<LinkTypes> = vec![];
  let mut pos: usize = 0;
  let mut id: u32 = 0;
  while pos < data.len() {
    let block_type: BlockTypes = u32::from_le_bytes(data[pos..pos + 4].try_into().unwrap()).into();
    let single: (Box<dyn PngBlock>, usize);
    match block_type {
      BlockTypes::EnhancedPacketBlock => {
        single = box_up(EnhancedPacket::parse(
          &data[pos..],
          id,
          &interfaces,
          &config,
        ))
      }
      BlockTypes::InterfaceDescriptionBlock => {
        let ifd = InterfaceDescription::parse(&data[pos..], id);
        interfaces.push(ifd.0.link_type);
        single = box_up(ifd);
      }
      BlockTypes::SectionHeaderBlock => single = box_up(SectionHeader::parse(&data[pos..], id)),
      _ => single = box_up(BaseBlock::parse(&data[pos..], id)),
    }
    if single.0.error() != &BlockErrorKind::None {
      out.push(single.0);
      break;
    }
    out.push(single.0);
    pos += single.1;
    id += 1;
  }
  out
}

pub fn draw_block(
  block: &Box<dyn PngBlock>,
  mut area: Rect,
  buf: &mut Buffer,
  hidden: u16,
  folded: bool,
  ascii: bool,
) -> u16 {
  if block.error() == &BlockErrorKind::ZeroLength {
    Line::raw(block.id().to_string() + ": ERROR Block has zero length")
      .underlined()
      .bold()
      .render(area, buf);
  }

  let bytes_in_row = (area.width + 1) / 3;
  let total_rows = div_ceil(block.length() as u16, bytes_in_row) + 1;
  assert!(hidden < total_rows);

  let total_rows_to_print = std::cmp::min(total_rows - hidden, area.height);
  if total_rows_to_print == 0 {
    return 0;
  }
  let mut rows_to_print = total_rows_to_print;

  Line::raw(block.title_line())
    .underlined()
    .bold()
    .render(area, buf);
  if folded || rows_to_print == 1 {
    return 1;
  }

  area.y += 1;
  area.height -= 1;
  rows_to_print -= 1;

  let print_bytes: fn(&[u8]) -> String = if ascii { to_ascii } else { to_hex };

  let start: usize = (hidden * bytes_in_row) as usize;
  let end: usize = std::cmp::min(
    (hidden + rows_to_print) * bytes_in_row,
    block.length() as u16,
  ) as usize;
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
    let section = block.sections()[current_section].1;
    if index < start && index + section > start {
      spans.push(
        Span::raw(print_bytes(&block.raw()[start..index + section]))
          .fg(fg_colours[fg_colour_index])
          .bg(bg_colours[bg_colour_index]),
      );
    } else if index >= start && index + section <= end {
      spans.push(
        Span::raw(print_bytes(&block.raw()[index..index + section]))
          .fg(fg_colours[fg_colour_index])
          .bg(bg_colours[bg_colour_index]),
      );
    } else if index < end && index + section > end {
      spans.push(
        Span::raw(print_bytes(&block.raw()[index..end]))
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

pub fn to_hex(s: &[u8]) -> String {
  s.iter()
    .map(|b| format!("{:02x}", b))
    .collect::<Vec<_>>()
    .join(" ")
}

pub fn to_ascii(s: &[u8]) -> String {
  s.iter()
    .map(|&b| {
      if b > 32 && b < 127 {
        char::from(b)
      } else {
        '.'
      }
    })
    .map(|c| c.to_string() + " ")
    .collect::<Vec<_>>()
    .join(" ")
}
