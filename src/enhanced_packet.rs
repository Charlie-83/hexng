use crate::{
  baseblock::BaseBlock,
  pcapng::{BlockErrorKind, PngBlock},
  types::BlockTypes,
};

pub struct EnhancedPacket {
  base: BaseBlock,
  interface_id: u32,
  timestamp_upper: u32,
  timestamp_lower: u32,
  captured_packet_length: u32,
  original_packet_length: u32,
}

impl EnhancedPacket {
  pub const SIZE: usize = BaseBlock::SIZE + 20;

  pub fn parse(data: &[u8], id: u32) -> (EnhancedPacket, usize) {
    let base = BaseBlock::parse(data, id);
    let interface_id = u32::from_le_bytes(data[8..12].try_into().unwrap());
    let timestamp_upper = u32::from_le_bytes(data[12..16].try_into().unwrap());
    let timestamp_lower = u32::from_le_bytes(data[16..20].try_into().unwrap());
    let captured_packet_length = u32::from_le_bytes(data[24..28].try_into().unwrap());
    let original_packet_length = u32::from_le_bytes(data[28..32].try_into().unwrap());
    (
      EnhancedPacket {
        base: base.0,
        interface_id,
        timestamp_upper,
        timestamp_lower,
        captured_packet_length,
        original_packet_length,
      },
      base.1,
    )
  }
}

impl PngBlock for EnhancedPacket {
  fn rows(&self, width: u16) -> u16 {
    self.base.rows(width)
  }

  fn sections(&self) -> Vec<(&str, usize)> {
    let sections: Vec<(&str, usize)> = vec![
      ("Interface ID", 4),
      ("Timestamp Upper", 4),
      ("Timestamp Lower", 4),
      ("Captured Packet Length", 4),
      ("Original Packet Length", 4),
      ("Packet Data", self.base.length_ as usize - Self::SIZE),
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
    &BlockTypes::EnhancedPacketBlock
  }

  fn title_line(&self) -> String {
    self.base.title_line()
  }

  fn raw(&self) -> &Vec<u8> {
    self.base.raw()
  }
}
