use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[derive(Deserialize)]
pub struct EnhancedPacketConfig {
  pub name: String,
  pub linktype: u16,
  pub sections: Vec<(String, usize)>,
}

pub struct Config {
  pub enhanced_packets: Vec<EnhancedPacketConfig>,
  pub link_types: HashMap<u16, String>,
}

#[derive(Deserialize)]
pub struct JConfig {
  pub enhanced_packets: Option<Vec<EnhancedPacketConfig>>,
  pub link_types: Option<Vec<(u16, String)>>,
}

pub fn load(path: &str) -> std::io::Result<Config> {
  let mut file = File::open(path)?;
  let mut file_contents = String::new();
  file.read_to_string(&mut file_contents)?;

  let v: JConfig = serde_json::from_str(&file_contents)?;

  let enhanced_packets: Vec<EnhancedPacketConfig> = v.enhanced_packets.unwrap_or(vec![]);
  let additional_link_types: HashMap<u16, String> =
    v.link_types.unwrap_or(vec![]).into_iter().collect();

  Ok(Config {
    enhanced_packets,
    link_types: additional_link_types,
  })
}
