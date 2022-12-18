pub mod ast;

#[derive(Debug, Clone)]
pub struct Identifier {
  pub name: String,
  pub location: Location,
}

#[derive(Debug, Clone)]
pub struct Location {
  pub line: usize,
  pub col: i64,
}
