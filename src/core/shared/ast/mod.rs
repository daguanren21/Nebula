use std::fmt::{Display, Formatter};

pub mod expressions;
pub mod statements;

#[derive(Debug, Clone)]
pub struct Identifier {
  pub name: String,
  pub location: Position,
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
  pub line: usize,
  pub col: i64,
}
impl Position {
  pub fn new(line: usize, col: i64) -> Position {
    Position { line, col }
  }
}
impl Display for Position {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "line {}:{}", self.line, self.col)
  }
}
