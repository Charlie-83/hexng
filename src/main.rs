use clap::Parser;
use std::io;

mod app;
pub mod hexview;
pub mod pcapng;
pub mod util;
pub mod help;
pub mod baseblock;
pub mod enhanced_packet;
pub mod interface_description;
pub mod section_header;
pub mod types;
pub mod info;

#[derive(Parser)]
struct Cli {
  path: std::path::PathBuf,
}

fn main() -> io::Result<()> {
  let args = Cli::parse();
  let _ = args;
  let mut terminal = ratatui::init();
  let mut app = app::App::new(args.path)?;
  let app_result = app.run(&mut terminal);
  ratatui::restore();
  app_result
}
