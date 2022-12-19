use std::fmt::{Display, Formatter};

pub mod expressions;
pub mod statements;

#[derive(Debug, Clone)]
pub struct Identifier {
  pub name: String,
  pub location: Position,
}

#[derive(Debug, Clone)]
pub struct Position {
  pub line: usize,
  pub col: i64,
}
impl Display for Position {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "line {}:{}", self.line, self.col)
  }
}
