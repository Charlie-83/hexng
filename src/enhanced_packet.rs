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
    let interface_id = u32::from_le_bytes(data[8..12].try_into().unwrap());
    let base = BaseBlock::parse(data, id);
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

  fn sections(&self) -> Vec<(String, usize)> {
    let sections: Vec<(String, usize)> = vec![
      (
        "Interface ID - ".to_owned() + &self.interface_id.to_string(),
        4,
      ),
      (
        "Timestamp Upper - ".to_owned() + &self.timestamp_upper.to_string(),
        4,
      ),
      (
        "Timestamp Lower - ".to_owned() + &self.timestamp_lower.to_string(),
        4,
      ),
      (
        "Captured Packet Length - ".to_owned() + &self.captured_packet_length.to_string(),
        4,
      ),
      (
        "Original Packet Length - ".to_owned() + &self.original_packet_length.to_string(),
        4,
      ),
      (
        "Packet Data".to_owned(),
        self.captured_packet_length as usize,
      ),
      (
        "Options".to_owned(),
        self.base.length() - Self::SIZE - self.captured_packet_length as usize,
      ),
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
