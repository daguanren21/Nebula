use super::decls::Parser;
use crate::core::lexer::decls::Lexer;

impl Parser {
  pub fn new(contents: &str) -> Self {
    let tokens = Lexer::new(contents).collect();
    Self {
      tokens,
      current: 0,
      errors: Vec::new(),
    }
  }
}
