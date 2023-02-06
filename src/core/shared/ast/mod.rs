use std::fmt::{Display, Formatter};

pub mod expressions;
pub mod statements;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier {
  pub name: String,
  pub pos: Position,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
  pub line: usize,
  pub col: usize,
}
impl Position {
  pub fn new(line: usize, col: usize) -> Position {
    Position { line, col }
  }
}
impl Display for Position {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "line {}:{}", self.line, self.col)
  }
}
