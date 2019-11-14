use crate::common::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) struct Token<'a> {
  pub(crate) offset: usize,
  pub(crate) length: usize,
  pub(crate) line: usize,
  pub(crate) column: usize,
  pub(crate) src: &'a str,
  pub(crate) kind: TokenKind,
}

impl<'a> Token<'a> {
  pub(crate) fn lexeme(&self) -> &'a str {
    &self.src[self.offset..self.offset + self.length]
  }

  pub(crate) fn error(&self, kind: CompilationErrorKind<'a>) -> CompilationError<'a> {
    CompilationError { token: *self, kind }
  }

  pub(crate) fn write_context(&self, f: &mut Formatter, color: Color) -> fmt::Result {
    let width = if self.length == 0 { 1 } else { self.length };

    let line_number = self.line.ordinal();
    match self.src.lines().nth(self.line) {
      Some(line) => {
        let mut i = 0;
        let mut space_column = 0;
        let mut space_line = String::new();
        let mut space_width = 0;
        for c in line.chars() {
          if c == '\t' {
            space_line.push_str("    ");
            if i < self.column {
              space_column += 4;
            }
            if i >= self.column && i < self.column + width {
              space_width += 4;
            }
          } else {
            if i < self.column {
              space_column += UnicodeWidthChar::width(c).unwrap_or(0);
            }
            if i >= self.column && i < self.column + width {
              space_width += UnicodeWidthChar::width(c).unwrap_or(0);
            }
            space_line.push(c);
          }
          i += c.len_utf8();
        }
        let line_number_width = line_number.to_string().len();
        writeln!(f, "{0:1$} |", "", line_number_width)?;
        writeln!(f, "{} | {}", line_number, space_line)?;
        write!(f, "{0:1$} |", "", line_number_width)?;
        write!(
          f,
          " {0:1$}{2}{3:^<4$}{5}",
          "",
          space_column,
          color.prefix(),
          "",
          space_width,
          color.suffix()
        )?;
      }
      None => {
        if self.offset != self.src.len() {
          write!(
            f,
            "internal error: Error has invalid line number: {}",
            line_number
          )?
        }
      }
    }
    Ok(())
  }
}
