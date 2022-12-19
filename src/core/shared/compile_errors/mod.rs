use thiserror::Error;

use super::ast::Position;

#[derive(Debug, Error)]
pub enum CompileError {
  // Lexer Errors:
  #[error("(Syntax) Punctuation \"{punc_str}\" is mismatched at {pos}")]
  ImbalancedPair { punc_str: String, pos: Position },

  #[error("(Syntax) Invalid {numeric_type} number format at {pos}")]
  InvalidFormatNumber { numeric_type: String, pos: Position },

  #[error("(Syntax) Invalid empty char at {pos}")]
  InvalidEmptyChar { pos: Position },

  #[error("(Syntax) Unclosed char literal at {pos}")]
  UnclosedCharLiteral { pos: Position },

  // Parser Errors:
  #[error("(Syntax) Unexpected token {token_name} at {pos}")]
  UnexpectedToken { token_name: String, pos: Position },
}
