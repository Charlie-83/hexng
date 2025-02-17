use crate::{
  baseblock::BaseBlock,
  pcapng::{BlockErrorKind, PngBlock},
  types::BlockTypes,
};

pub struct SectionHeader {
  base: BaseBlock,
  little_endian: bool,
  major_version: u16,
  minor_version: u16,
  section_length: u64,
}

impl SectionHeader {
  pub const SIZE: usize = BaseBlock::SIZE + 16;

  pub fn parse(data: &[u8], id: u32) -> (SectionHeader, usize) {
    let base = BaseBlock::parse(data, id);
    let little_endian = u32::from_le_bytes(data[8..12].try_into().unwrap()) == 0x1a2b3c4d;
    let major_version = u16::from_le_bytes(data[12..14].try_into().unwrap());
    let minor_version = u16::from_le_bytes(data[14..16].try_into().unwrap());
    let section_length = u64::from_le_bytes(data[16..24].try_into().unwrap());
    (
      SectionHeader {
        base: base.0,
        little_endian,
        major_version,
        minor_version,
        section_length,
      },
      base.1,
    )
  }
}

impl PngBlock for SectionHeader {
  fn rows(&self, width: u16) -> u16 {
    self.base.rows(width)
  }

  fn sections(&self) -> Vec<(&str, usize)> {
    let sections: Vec<(&str, usize)> = vec![
      ("Section Byte Order", 4),
      ("Major Version", 2),
      ("Minor Version", 2),
      ("Section Length", 8),
      ("Options", self.base.length() - Self::SIZE),
    ];
    let mut base_sections = self.base.sections();
    base_sections.remove(2);
    base_sections.splice(2..2, sections);
    assert_eq!(
      base_sections.iter().map(|s| s.1 as u32).sum::<u32>(),
      self.base.length_
    );
    base_sections
  }

  fn error(&self) -> &BlockErrorKind {
    self.base.error()
  }

  fn id(&self) -> u32 {
    self.base.id()
  }

  fn length(&self) -> usize {
    self.base.length()
  }

  fn block_type(&self) -> &BlockTypes {
    &BlockTypes::SectionHeaderBlock
  }

  fn title_line(&self) -> String {
    self.base.title_line()
  }

  fn raw(&self) -> &Vec<u8> {
    self.base.raw()
  }
}
