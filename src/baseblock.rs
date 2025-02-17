use crate::pcapng::{BlockErrorKind, PngBlock};
use crate::types::{block_type_str, BlockTypes};
use crate::util::div_ceil;

pub struct BaseBlock {
  pub id_: u32,
  pub raw_: Vec<u8>,
  pub block_type_: BlockTypes,
  pub length_: u32,
  pub options_: Vec<u8>,
  pub error_: BlockErrorKind,
}

impl BaseBlock {
  pub const SIZE: usize = 12;

  pub fn parse(data: &[u8], id: u32) -> (BaseBlock, usize) {
    let block_type: BlockTypes = u32::from_le_bytes(data[..4].try_into().unwrap()).into();
    let length = u32::from_le_bytes(data[4..8].try_into().unwrap());
    if length == 0 {
      return (
        BaseBlock::new(
          vec![],
          block_type,
          length,
          vec![],
          id,
          BlockErrorKind::ZeroLength,
        ),
        0,
      );
    }
    return (
      BaseBlock::new(
        data[..(length as usize)].to_vec(),
        block_type,
        length,
        vec![],
        id,
        BlockErrorKind::None,
      ),
      length as usize,
    );
  }
}

impl BaseBlock {
  pub fn new(
    raw: Vec<u8>,
    block_type: BlockTypes,
    length: u32,
    options: Vec<u8>,
    id: u32,
    error: BlockErrorKind,
  ) -> BaseBlock {
    BaseBlock {
      raw_: raw,
      block_type_: block_type,
      length_: length,
      options_: options,
      id_: id,
      error_: error,
    }
  }
}

impl PngBlock for BaseBlock {
  fn rows(&self, width: u16) -> u16 {
    if self.error_ == BlockErrorKind::ZeroLength {
      return 1;
    }
    let bytes_in_row = (width + 1) / 3;
    div_ceil(self.length_ as u16, bytes_in_row) + 1
  }

  fn sections(&self) -> Vec<(&str, usize)> {
    let sections = vec![
      ("Block Type", 4),
      ("Block Length", 4),
      ("Data", self.length_ as usize - Self::SIZE),
      ("Block Length", 4),
    ];
    assert_eq!(
      sections.iter().map(|s| s.1 as u32).sum::<u32>(),
      self.length_
    );
    sections
  }

  fn error(&self) -> &BlockErrorKind {
    &self.error_
  }

  fn id(&self) -> u32 {
    self.id_
  }

  fn title_line(&self) -> String {
    self.id_.to_string() + ": " + &block_type_str(self.block_type())
  }

  fn length(&self) -> usize {
    self.length_ as usize
  }

  fn block_type(&self) -> &BlockTypes {
    &self.block_type_
  }

  fn raw(&self) -> &Vec<u8> {
    &self.raw_
  }
}
