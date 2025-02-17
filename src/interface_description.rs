use crate::{
  baseblock::BaseBlock,
  pcapng::{BlockErrorKind, PngBlock},
  types::{link_type_str, BlockTypes, LinkTypes},
};

pub struct InterfaceDescription {
  base: BaseBlock,
  link_type: LinkTypes,
  reserved: u16,
  snap_length: u32,
}

impl InterfaceDescription {
  pub const SIZE: usize = BaseBlock::SIZE + 8;

  pub fn parse(data: &[u8], id: u32) -> (Self, usize) {
    let base = BaseBlock::parse(data, id);
    let link_type: LinkTypes = u16::from_le_bytes(data[8..10].try_into().unwrap()).into();
    let reserved = u16::from_le_bytes(data[10..12].try_into().unwrap());
    let snap_length = u32::from_le_bytes(data[12..16].try_into().unwrap());
    (
      InterfaceDescription {
        base: base.0,
        link_type,
        reserved,
        snap_length,
      },
      base.1,
    )
  }
}

impl PngBlock for InterfaceDescription {
  fn rows(&self, width: u16) -> u16 {
    self.base.rows(width)
  }

  fn sections(&self) -> Vec<(&str, usize)> {
    let sections: Vec<(&str, usize)> = vec![
      ("Link Type", 2),
      ("Reserved", 2),
      ("Snap Length", 4),
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
    &BlockTypes::InterfaceDescriptionBlock
  }

  fn title_line(&self) -> String {
    self.base.title_line() + " - " + &link_type_str(&self.link_type)
  }

  fn raw(&self) -> &Vec<u8> {
    self.base.raw()
  }
}
