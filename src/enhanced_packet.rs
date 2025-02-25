use crate::{
  baseblock::BaseBlock,
  loader::Config,
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
  link_type: u16,
  sections_: Vec<(String, usize)>,
  link_type_str: String,
}

impl EnhancedPacket {
  pub const SIZE: usize = BaseBlock::SIZE + 20;

  pub fn parse(
    data: &[u8],
    id: u32,
    interfaces: &Vec<u16>,
    config: &Config,
  ) -> (EnhancedPacket, usize) {
    let interface_id = u32::from_le_bytes(data[8..12].try_into().unwrap());
    let base = BaseBlock::parse(data, id);
    let timestamp_upper = u32::from_le_bytes(data[12..16].try_into().unwrap());
    let timestamp_lower = u32::from_le_bytes(data[16..20].try_into().unwrap());
    let captured_packet_length = u32::from_le_bytes(data[24..28].try_into().unwrap());
    let original_packet_length = u32::from_le_bytes(data[28..32].try_into().unwrap());
    let link_type = interfaces[interface_id as usize];
    let link_type_str = config.link_types[&link_type].clone();
    let mut p = EnhancedPacket {
      base: base.0,
      interface_id,
      timestamp_upper,
      timestamp_lower,
      captured_packet_length,
      original_packet_length,
      link_type,
      sections_: vec![],
      link_type_str,
    };
    p.sections_ = p.sections_impl(config);
    (p, base.1)
  }

  fn sections_impl(&self, config: &Config) -> Vec<(String, usize)> {
    let mut sections: Vec<(String, usize)> = vec![
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
    ];

    let mut sum = 0;
    for en in &config.enhanced_packets {
      let l: u16 = en.linktype.try_into().unwrap();
      if l != self.link_type {
        continue;
      }
      for s in &en.sections {
        let data: &[u8] = &self.raw()[28 + sum..28 + sum + s.1];
        let mut data_padded: [u8; 8] = [0; 8];
        for i in 0..data.len().min(8) {
          data_padded[i] = data[i];
        }
        sections.push((
          s.0.clone() + " - " + &u64::from_le_bytes(data_padded).to_string(),
          s.1,
        ));
        sum += s.1;
      }
      break;
    }
    sections.push((
      "Data".to_owned(),
      self.captured_packet_length as usize - sum,
    ));
    sections.push((
      "Options".to_owned(),
      self.length() - Self::SIZE - self.captured_packet_length as usize,
    ));
    let mut base_sections = self.base.sections();
    base_sections.remove(2);
    base_sections.splice(2..2, sections);

    assert_eq!(
      base_sections.iter().map(|s| s.1).sum::<usize>(),
      self.length()
    );
    base_sections
  }
}

impl PngBlock for EnhancedPacket {
  fn rows(&self, width: u16) -> u16 {
    self.base.rows(width)
  }

  fn sections(&self) -> Vec<(String, usize)> {
    self.sections_.clone()
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
    self.base.title_line() + " - " + &self.link_type_str
  }

  fn raw(&self) -> &Vec<u8> {
    self.base.raw()
  }
}
