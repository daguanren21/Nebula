use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::iter::Peekable;
use std::str::Chars;
use thiserror::Error;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TokenType {
  // ----- keywords :
  // * Module management:
  Use,
  Pub,
  As,
  // * Flow control:
  If,
  Else,
  For,
  Each,
  In,
  Match,
  Break,
  Continue,
  // * Variable declaration:
  Var,
  Const,
  // * Function declaration:
  Fn,
  Return,
  Guard,
  // * Composition and abstract:
  Struct,
  Trait,
  Enum,
  Impl,
  // * Asynchronization:
  Async,
  Await,

  // ----- Literals:
  Nil,
  True,
  False,
  Crate,

  // Number literals:
  DecimalInteger,
  OctalInteger,
  HexadecimalInteger,
  BinaryInteger,
  Exponent,
  Float,

  // String literals:
  Char,
  String,

  // Punctuations:
  Semi,         // ;
  Comma,        // ,
  Colon,        // :
  LeftParen,    // (
  RightParen,   // )
  LeftBrace,    // {
  RightBrace,   // }
  LeftBracket,  // [
  RightBracket, // ]

  // ----- Operators:
  DoubleColon,           // ::
  Dot,                   // .
  Equal,                 // =
  DoubleEqual,           // ==
  BangEqual,             // !=
  Plus,                  // +
  Minus,                 // -
  Star,                  // *
  DoubleStar,            // **
  Slash,                 // /
  Percent,               // %
  Alpha,                 // @
  Wavy,                  // ~
  Caret,                 // ^
  Ampersand,             // &
  Bang,                  // !
  Vertical,              // |
  LeftAngle,             // <
  RightAngle,            // >
  DoubleLeftAngle,       // <<
  DoubleRightAngle,      // >>
  DoubleAmpersand,       // &&
  DoubleVertical,        // ||
  LeftAngleEqual,        // <=
  RightAngleEqual,       // >=
  LeftArrow,             // <-
  RightArrow,            // ->
  PlusEqual,             // +=
  MinusEqual,            // -=
  StarEqual,             // *=
  SlashEqual,            // /=
  PercentEqual,          // %=
  DoubleLeftAngleEqual,  // <<=
  DoubleRightAngleEqual, // >>=
  AmpersandEqual,        // &=
  VerticalEqual,         // |=
  CaretEqual,            // ^=
  DoubleDots,            // ..
  ThreeDots,             // ...

  // ----- Identifier symbol:
  Identifier,

  // EOF
  EOF,
}

#[derive(Debug)]
pub struct Token {
  pub raw: String,
  pub kind: TokenType,
  pub offset: usize,
  pub line: usize,
  pub col: usize,
}

pub struct Lexer<'a> {
  // Human readable position in file
  pub cur_line: usize,
  pub cur_col: usize,

  // offset cursor of character moving
  pub offset_cursor: usize,

  pub chars: Peekable<Chars<'a>>,
  pub pair_balance: HashMap<&'a str, i32>,

  // keywords map
  pub reserved_words_map: RefCell<HashMap<&'a str, TokenType>>,

  // collecting errors, don't interrupt lexing process
  pub errors: Vec<LexerError<'a>>,
}

#[derive(Debug)]
pub struct TokenPos {
  pub line: usize,
  pub col: usize,
}
impl Display for TokenPos {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "line {}:{}", self.line, self.col)
  }
}

#[derive(Debug, Error)]
pub enum LexerError<'d> {
  #[error("Punctuation {kind} is mismatched at line {pos}")]
  ImbalancedPair { kind: &'d str, pos: TokenPos },

  #[error("Invalid{numeric_type}number format at line {pos}")]
  InvalidFormatNumber { numeric_type: String, pos: TokenPos },

  #[error("Invalid empty char at line {pos}")]
  InvalidEmptyChar { pos: TokenPos },

  #[error("Unclosed char literal at line {pos}")]
  UnclosedCharLiteral { pos: TokenPos },
}

pub enum NumberRadix {
  Decimal,
  Octal,
  Binary,
  Hexadecimal,
}
