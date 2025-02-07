use ratatui::{
  buffer::Buffer,
  layout::Rect,
  text::Text,
  widgets::{Block, Clear, Paragraph, Widget},
};

const HELP_TEXT: &[&str] = &[
  "q      : Quit\n",
  "j      : Down\n",
  "i      : Up\n",
  "h      : Left\n",
  "l      : Right\n",
  "G      : Jump to bottom\n",
  "g      : Jump to top\n",
  "CTRL-D : Scroll down half page\n",
  "CTRL-U : Scroll up half page\n",
  "f      : Toggle fold\n",
  "a      : Toggle ascii\n",
  "?      : Toggle help\n",
];

pub const HELP_LINES: u16 = HELP_TEXT.len() as u16;

pub fn draw_help(area: Rect, buf: &mut Buffer) {
  Clear::default().render(area, buf);
  Block::bordered().render(area, buf);
  Paragraph::new("Commands").render(area, buf);

  Paragraph::new(Text::from_iter(HELP_TEXT.iter().map(|s| s.to_string()))).render(
    Rect {
      x: area.x + 2,
      y: area.y + 1,
      width: area.width - 2,
      height: area.height - 1,
    },
    buf,
  );
}
