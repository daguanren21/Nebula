use crate::core::{lexer::decls::Lexer, shared::compile_errors::CompileError};

pub struct Parser<'a> {
  /// Obtain a lexer inside parser to peek next token
  pub lexer: Lexer<'a>,

  /// All the errors that occurred during the parsing.<br>
  /// (Adopted the errors that occurred during the lexing.)
  pub errors: Vec<CompileError>,
}
