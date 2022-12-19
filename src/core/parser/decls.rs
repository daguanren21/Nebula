use crate::core::{lexer::decls::Token, shared::compile_errors::CompileError};

pub struct Parser {
  /// All the tokens of the source code.
  pub tokens: Vec<Token>,
  /// The index of the current token.<br>
  /// It's a cursor in the tokens' iteration.
  pub current: usize,

  /// All the errors that occurred during the parsing.<br>
  /// (Adopted the errors that occurred during the lexing.)
  pub errors: Vec<CompileError>,
}
