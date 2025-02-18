use crate::pcapng::PngBlock;

pub fn get_detail_string(block: &Box<dyn PngBlock>, width: u16, cursor: (u16, u16)) -> String {
  if cursor.1 == 0 {
    return block.title_line();
  }
  let mut offset = ((cursor.1 - 1) * width + cursor.0) / 3;

  let sections = block.sections();
  for (description, size) in sections {
    if offset < size as u16 {
      return description.to_owned();
    }
    offset -= size as u16;
  }
  "".to_owned()
}
