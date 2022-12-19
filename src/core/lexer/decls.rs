use std::cell::RefCell;
use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

use crate::core::shared::compile_errors::CompileError;

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
  // * Composition and abstract:
  Struct,
  New,
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
  _Self_,

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
  DoubleDotsEqual,       // ..=
  Question,              // ?
  QuestionDot,           // ?.

  // ----- Important symbols:
  DollarColon, // $:

  // ----- Identifier symbol:
  Identifier,

  // EOF
  EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
  pub raw: String,
  pub kind: TokenType,
  pub offset: usize,
  pub line: usize,
  pub col: i64,
}

pub struct Lexer<'a> {
  // Human readable position in file
  pub cur_line: usize,
  pub cur_col: i64,

  // offset cursor of character moving
  pub offset_cursor: usize,

  pub chars: Peekable<Chars<'a>>,
  pub pair_balance: HashMap<&'a str, i32>,

  // keywords map
  pub reserved_words_map: RefCell<HashMap<&'a str, TokenType>>,

  // collecting errors, don't interrupt lexing process
  pub errors: Vec<CompileError<'a>>,
}

pub enum NumberRadix {
  Decimal,
  Octal,
  Binary,
  Hexadecimal,
}
