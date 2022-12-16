use crate::core::lexer::decls::Token;

pub struct Parser {
  pub tokens: Vec<Token>,
  pub current: usize,
}
