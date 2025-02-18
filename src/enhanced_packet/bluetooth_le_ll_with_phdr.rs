use crate::{
  pcapng::{BlockErrorKind, PngBlock},
  types::BlockTypes,
};

use super::EnhancedPacket;

pub struct BluetoothLELLWithPHDR {
  base: EnhancedPacket,
  rf_channel: u8,
  signal_power: u8,
  noise_power: u8,
  access_address_offenses: u8,
  reference_access_address: u32,
  flags: u16,
}

impl BluetoothLELLWithPHDR {
  pub const SIZE: usize = EnhancedPacket::SIZE + 10;

  pub fn parse(data: &[u8], base: EnhancedPacket) -> BluetoothLELLWithPHDR {
    let rf_channel = u8::from_le_bytes(data[0..1].try_into().unwrap());
    let signal_power = u8::from_le_bytes(data[1..2].try_into().unwrap());
    let noise_power = u8::from_le_bytes(data[2..3].try_into().unwrap());
    let access_address_offenses = u8::from_le_bytes(data[3..4].try_into().unwrap());
    let reference_access_address = u32::from_le_bytes(data[4..8].try_into().unwrap());
    let flags = u16::from_le_bytes(data[8..10].try_into().unwrap());
    BluetoothLELLWithPHDR {
      base,
      rf_channel,
      signal_power,
      noise_power,
      access_address_offenses,
      reference_access_address,
      flags,
    }
  }
}

impl PngBlock for BluetoothLELLWithPHDR {
  fn rows(&self, width: u16) -> u16 {
    self.base.rows(width)
  }

  fn sections(&self) -> Vec<(String, usize)> {
    let sections: Vec<(String, usize)> = vec![
      ("RF Channel - ".to_owned() + &self.rf_channel.to_string(), 1),
      (
        "Signal Power - ".to_owned() + &self.signal_power.to_string(),
        1,
      ),
      (
        "Noise Power - ".to_owned() + &self.noise_power.to_string(),
        1,
      ),
      (
        "Access Address Offenses - ".to_owned() + &self.access_address_offenses.to_string(),
        1,
      ),
      (
        "Reference Access Address - ".to_owned() + &self.reference_access_address.to_string(),
        4,
      ),
      ("Flags - ".to_owned() + &self.flags.to_string(), 2),
      (
        "Data".to_owned(),
        self.base.captured_packet_length as usize - 10,
      ),
    ];
    let mut base_sections = self.base.sections();
    base_sections.remove(7);
    base_sections.splice(7..7, sections);
    assert_eq!(
      base_sections.iter().map(|s| s.1).sum::<usize>(),
      self.length()
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
