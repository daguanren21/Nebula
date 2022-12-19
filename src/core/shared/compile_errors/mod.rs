use thiserror::Error;

use super::ast::Position;

#[derive(Debug, Error)]
pub enum CompileError<'d> {
  // Lexer Errors:
  #[error("Punctuation {kind} is mismatched at {pos}")]
  ImbalancedPair { kind: &'d str, pos: Position },

  #[error("Invalid {numeric_type} number format at {pos}")]
  InvalidFormatNumber { numeric_type: String, pos: Position },

  #[error("Invalid empty char at {pos}")]
  InvalidEmptyChar { pos: Position },

  #[error("Unclosed char literal at {pos}")]
  UnclosedCharLiteral { pos: Position },

  // Parser Errors:
  #[error("Unexpected token {token_name} at {pos}")]
  UnexpectedToken { token_name: String, pos: Position },
}
