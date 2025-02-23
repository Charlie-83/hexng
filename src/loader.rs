use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;

#[derive(Deserialize)]
pub struct JEnhanced {
  pub name: String,
  pub linktype: u16,
  pub sections: Vec<(String, usize)>,
}

#[derive(Deserialize)]
pub struct JTop {
  pub enhanced_packets: Vec<JEnhanced>,
}

pub fn load(path: &str) -> std::io::Result<JTop> {
  let mut file = File::open(path)?;
  let mut file_contents = String::new();
  file.read_to_string(&mut file_contents)?;

  let v: JTop = serde_json::from_str(&file_contents)?;

  Ok(v)
}
