use super::decls::Parser;
use crate::core::lexer::decls::Lexer;

impl Parser {
  pub fn new(contents: &str) -> Self {
    let mut lexer = Lexer::new(contents);
    let tokens = lexer.peek_all_tokens();

    // Adopt collected compile errors to parser layer
    let errors = lexer.errors.clone();

    Self {
      tokens,
      current: 0,
      errors,
    }
  }
}
