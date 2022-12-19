use crate::core::lexer::decls::{Lexer, LexerError, NumberRadix, Token, TokenType};
use crate::core::shared::ast::Position;
use crate::hashmap;
use std::cell::RefCell;
use std::collections::HashMap;

impl<'a> Lexer<'a> {
  fn create_token(&mut self, token_type: TokenType, raw: String) -> Token {
    Token {
      raw,
      kind: token_type,
      offset: self.offset_cursor,
      line: self.cur_line,
      col: self.cur_col,
    }
  }

  fn get_current_pos(&self) -> Position {
    Position {
      line: self.cur_line,
      col: self.cur_col,
    }
  }

  /// consumed one char with updating line/column information
  fn consume_char(&mut self) -> Option<char> {
    match self.chars.next() {
      Some(c) => {
        if c == '\n' {
          self.cur_line += 1;
          self.cur_col = 1;
        } else {
          self.cur_col += 1;
        }
        self.offset_cursor += 1;
        Some(c)
      }
      None => None,
    }
  }

  /// move over \t and spaces and update column information
  fn skip_whitespaces(&mut self) {
    while let Some(c) = self.chars.peek() {
      if !c.is_whitespace() {
        break;
      }
      self.consume_char();
    }
  }

  fn push_pair_punctuator(&mut self, kind: &'a str) {
    let punc_level = self.pair_balance.entry(&kind).or_insert(0);
    *punc_level += 1;
  }

  fn pop_pair_punctuator(&mut self, kind: &'a str) {
    if !self.pair_balance.contains_key(&kind) {
      // a bare close punctuator occurred
      return self.errors.push(LexerError::ImbalancedPair {
        kind,
        pos: self.get_current_pos(),
      });
    }

    let level = self.pair_balance.get_mut(&kind).unwrap();
    if *level - 1 < 0 {
      self.errors.push(LexerError::ImbalancedPair {
        kind,
        pos: self.get_current_pos(),
      });
    } else {
      *level -= 1;
    }
  }

  fn multiple_chars_lexing(
    &mut self,
    first: (TokenType, String),
    second: HashMap<char, (TokenType, String, Option<Vec<(char, TokenType, String)>>)>,
  ) -> Option<Token> {
    let (first_type, first_str) = first;
    if let Some(second_c) = self.chars.peek() {
      for second_option in second.iter() {
        let (second_char, (second_type, second_str, third)) = second_option;
        if *second_c == *second_char {
          self.consume_char();
          if let (Some(third_c), Some(third)) = (self.chars.peek(), third) {
            for third_option in third.iter() {
              if let (third_char, third_type, third_str) = third_option {
                if *third_c == *third_char {
                  self.consume_char();
                  return Some(self.create_token(*third_type, third_str.clone()));
                }
              }
            }
          }
          return Some(self.create_token(*second_type, second_str.clone()));
        }
      }
    }
    Some(self.create_token(first_type, first_str))
  }

  fn match_next_char(&mut self, c: char) -> bool {
    match self.chars.peek() {
      Some(&next) => next == c,
      None => false,
    }
  }

  fn lexing_punctuation(&mut self, c: char) -> Option<(TokenType, String)> {
    match c {
      ';' => Some((TokenType::Semi, String::from(";"))),
      ',' => Some((TokenType::Comma, String::from(","))),
      '(' => {
        self.push_pair_punctuator("parenthesis");
        Some((TokenType::LeftParen, String::from("(")))
      }
      ')' => {
        self.pop_pair_punctuator("parenthesis");
        Some((TokenType::RightParen, String::from(")")))
      }
      '{' => {
        self.push_pair_punctuator("brace");
        Some((TokenType::LeftBrace, String::from("{")))
      }
      '}' => {
        self.pop_pair_punctuator("brace");
        Some((TokenType::RightBrace, String::from("}")))
      }
      '[' => {
        self.push_pair_punctuator("bracket");
        Some((TokenType::LeftBracket, String::from("[")))
      }
      ']' => {
        self.pop_pair_punctuator("bracket");
        Some((TokenType::RightBracket, String::from("]")))
      }
      _ => None,
    }
  }

  fn lexing_slash_more(&mut self) -> Option<Token> {
    let single_slash = Some(self.create_token(TokenType::Slash, String::from("/")));
    if let Some(next_char) = self.chars.peek() {
      return match *next_char {
        '=' => {
          self.consume_char();
          Some(self.create_token(TokenType::SlashEqual, String::from("/=")))
        }
        '/' => {
          // skip for line comments
          self.consume_char(); // eat the second slash character
          while let Some(following_char) = self.consume_char() {
            if following_char == '\n' {
              break;
            }
          }
          None
        }
        '*' => {
          // skip block comment, won't support nested style: /* ... /* ... */ ... */
          self.consume_char(); // eat the star character
          while let Some(following_char) = self.consume_char() {
            if following_char == '*' {
              if let Some(after_star) = self.chars.peek() {
                if *after_star == '/' {
                  self.consume_char(); // eat '*/'
                  break;
                }
              }
            }
          }
          None
        }
        _ => single_slash,
      };
    } else {
      single_slash
    }
  }

  /// Numeric format:
  ///   1. decimal: not starts from '0', and 0-9 is valid for other digit bits
  ///       - normal: e.g. 1654
  ///       - exponent (scientific notation): e.g. 11e3, 1.6e7
  ///   2. octal: starts from '0', and 0-7 is valid for other digit bits
  ///   3. binary: starts from '0b' or '0B', only 0 and 1 are valid for digit bits
  ///   4. hexadecimal: starts from '0x' or '0X', and 0-9a-fA-F is valid for digit bits
  fn lexing_numeric(&mut self, start_digit: char) -> Option<Token> {
    let mut num_radix: NumberRadix = NumberRadix::Decimal;
    if start_digit == '0' {
      if let Some(&second_char) = self.chars.peek() {
        if second_char == 'x' || second_char == 'X' {
          num_radix = NumberRadix::Hexadecimal;
          self.consume_char(); // eat the 'x'/'X'
        } else if second_char == 'b' || second_char == 'B' {
          num_radix = NumberRadix::Binary;
          self.consume_char(); // eat the 'b'/'B'
        } else if second_char.is_digit(8) {
          num_radix = NumberRadix::Octal;
        } else {
          return Some(self.create_token(TokenType::DecimalInteger, String::from("0")));
        }
      }
    }

    match num_radix {
      NumberRadix::Hexadecimal => self.lexing_number_by_radix(16, "0x"),
      NumberRadix::Decimal => self.lexing_decimal(start_digit),
      NumberRadix::Octal => self.lexing_number_by_radix(8, "0"),
      NumberRadix::Binary => self.lexing_number_by_radix(2, "0b"),
    }
  }

  fn lexing_number_by_radix(&mut self, radix: u32, prefix: &'a str) -> Option<Token> {
    let mut digits_collect = Vec::<char>::new();
    while let Some(moved_next_char) = self.consume_char() {
      if moved_next_char.is_digit(radix) {
        digits_collect.push(moved_next_char);
      } else {
        break;
      }
    }
    if digits_collect.len() == 0 {
      let radix_string = match radix {
        16 => String::from(" hexadecimal "),
        8 => String::from(" octal "),
        2 => String::from(" binary "),
        _ => return None,
      };
      self.errors.push(LexerError::InvalidFormatNumber {
        numeric_type: radix_string,
        pos: self.get_current_pos(),
      });
      None
    } else {
      let token_type = match radix {
        16 => TokenType::HexadecimalInteger,
        8 => TokenType::OctalInteger,
        2 => TokenType::BinaryInteger,
        _ => return None,
      };
      Some(self.create_token(
        token_type,
        format!("{}{}", prefix, String::from_iter(digits_collect)),
      ))
    }
  }

  fn output_special_decimal_result(
    &mut self,
    numeric_type: TokenType,
    front: Vec<char>,
    after: Vec<char>,
  ) -> Option<Token> {
    if after.len() <= 1 {
      // length must be greater than 1 because already contains a symbol ('.' or 'e')
      self.errors.push(LexerError::InvalidFormatNumber {
        numeric_type: match numeric_type {
          TokenType::Exponent => String::from(" exponent "),
          TokenType::Float => String::from(" float "),
          _ => return None,
        },
        pos: self.get_current_pos(),
      });
      None
    } else {
      Some(self.create_token(
        numeric_type,
        format!("{}{}", String::from_iter(front), String::from_iter(after)),
      ))
    }
  }

  fn lexing_decimal(&mut self, start_digit: char) -> Option<Token> {
    let mut integer_collect = vec![start_digit];
    let mut fractional_collect = Vec::<char>::new();
    let mut exponent_collect = Vec::<char>::new();
    let mut is_float = false;
    let mut is_exponent = false;
    while let Some(&next_char) = self.chars.peek() {
      if next_char.is_digit(10) {
        if is_exponent {
          exponent_collect.push(next_char);
        } else if is_float {
          fractional_collect.push(next_char);
        } else {
          integer_collect.push(next_char);
        }
        self.consume_char();
      } else if next_char == '.' {
        if !is_exponent {
          is_float = true;
          fractional_collect.push(next_char); // collect the '.' character
          self.consume_char(); // eat the '.' character
        } else {
          // fractional digits are invalid in exponent part
          // so considered as end of exponent collection
          // do not eat the '.'
          break;
        }
      } else if next_char == 'e' || next_char == 'E' {
        is_exponent = true;
        exponent_collect.push(next_char); // add the 'e' character
        self.consume_char();
      } else {
        break;
      }
    }

    if is_exponent {
      if is_float {
        integer_collect.extend(fractional_collect);
      }
      self.output_special_decimal_result(TokenType::Exponent, integer_collect, exponent_collect)
    } else if is_float {
      self.output_special_decimal_result(TokenType::Float, integer_collect, fractional_collect)
    } else {
      Some(self.create_token(
        TokenType::DecimalInteger,
        format!("{}", String::from_iter(integer_collect)),
      ))
    }
  }

  fn lexing_identifier(&mut self, start_char: char) -> Option<Token> {
    let mut chars_collect = vec![start_char];
    while let Some(&next_char) = self.chars.peek() {
      if next_char.is_alphabetic() || next_char.is_digit(10) || next_char == '_' {
        chars_collect.push(next_char);
        self.consume_char();
      } else {
        break;
      }
    }

    let raw = String::from_iter(chars_collect);
    let mut token_type = TokenType::Identifier;
    if let Some(&reserved_word_type) = self.reserved_words_map.borrow().get(raw.as_str()) {
      token_type = reserved_word_type;
    }
    Some(self.create_token(token_type, raw))
  }

  fn lexing_escape_char(&mut self) -> Option<char> {
    if let Some(char_after_backslash) = self.chars.peek() {
      if let Some(escape_flag) = match *char_after_backslash {
        't' | 'n' | 'r' | '\\' => Some(*char_after_backslash),
        _ => None,
      } {
        return match escape_flag {
          't' => Some('\t'),
          'n' => Some('\n'),
          'r' => Some('\r'),
          '\\' => Some('\\'),
          _ => None,
        };
      }
    }
    None
  }

  fn lexing_char(&mut self) -> Option<Token> {
    if let Some(&c) = self.chars.peek() {
      if c == '\\' {
        // escape character format: t/n/r/\ are available
        self.consume_char(); // eat the backslash
        if let Some(&char_after_backslash) = self.chars.peek() {
          if char_after_backslash == '\'' {
            self.consume_char();
            return Some(self.create_token(TokenType::Char, String::from("'")));
          } else if let Some(escape_char) = self.lexing_escape_char() {
            self.consume_char();
            if self.match_next_char('\'') {
              self.consume_char();
              return Some(self.create_token(TokenType::Char, String::from(escape_char)));
            } else {
              self.errors.push(LexerError::UnclosedCharLiteral {
                pos: self.get_current_pos(),
              })
            }
          }
        }
      } else if c == '\'' {
        self.errors.push(LexerError::InvalidEmptyChar {
          pos: self.get_current_pos(),
        });
      } else {
        let got_char = c;
        self.consume_char();
        if self.match_next_char('\'') {
          self.consume_char();
          return Some(self.create_token(TokenType::Char, got_char.to_string()));
        } else {
          self.errors.push(LexerError::UnclosedCharLiteral {
            pos: self.get_current_pos(),
          })
        }
      }
    }
    None
  }

  fn lexing_string(&mut self) -> Option<Token> {
    let mut chars_collect = Vec::<char>::new();
    while let Some(&c) = self.chars.peek() {
      if c == '\\' {
        // escape character format: t/n/r/\ are available
        self.consume_char(); // eat the backslash
        if let Some(&char_after_backslash) = self.chars.peek() {
          if char_after_backslash == '"' {
            chars_collect.push('"');
            self.consume_char();
          } else if let Some(escape_char) = self.lexing_escape_char() {
            chars_collect.push(escape_char);
            self.consume_char();
          }
        }
      } else if c == '"' {
        self.consume_char(); // eat the close quote character
        break;
      } else {
        chars_collect.push(c);
        self.consume_char();
      }
    }
    Some(self.create_token(TokenType::String, String::from_iter(chars_collect)))
  }

  pub fn new(contents: &'a str) -> Lexer<'a> {
    Lexer {
      cur_line: 1,
      cur_col: 1,
      offset_cursor: 0,
      chars: contents.chars().peekable(),
      pair_balance: HashMap::new(),
      reserved_words_map: RefCell::new(hashmap! {
          "use" => TokenType::Use,
          "pub" => TokenType::Pub,
          "as" => TokenType::As,
          "if" => TokenType::If,
          "else" => TokenType::Else,
          "for" => TokenType::For,
          "each" => TokenType::Each,
          "in" => TokenType::In,
          "match" => TokenType::Match,
          "break" => TokenType::Break,
          "continue" => TokenType::Continue,
          "var" => TokenType::Var,
          "const" => TokenType::Const,
          "fn" => TokenType::Fn,
          "return" => TokenType::Return,
          "struct" => TokenType::Struct,
          "new" => TokenType::New,
          "trait" => TokenType::Trait,
          "enum" => TokenType::Enum,
          "impl" => TokenType::Impl,
          "async" => TokenType::Async,
          "await" => TokenType::Await,
          "true" => TokenType::True,
          "false" => TokenType::False,
          "nil" => TokenType::Nil,
          "crate" => TokenType::Crate,
          "self" => TokenType::_Self_
      }),
      errors: Vec::<LexerError>::new(),
    }
  }

  pub fn peek_next_token(&mut self) -> Option<Token> {
    self.skip_whitespaces();
    while let Some(c) = self.consume_char() {
      // punctuations are all single-character
      if let Some(punc_tuple) = self.lexing_punctuation(c) {
        let (punc_type, punc_raw) = punc_tuple;
        return Some(self.create_token(punc_type, punc_raw));
      }

      // special handling for slash character
      // because it may start a comment
      if c == '/' {
        if let Some(got_token) = self.lexing_slash_more() {
          return Some(got_token);
        } else {
          // comment were passed, but didn't got a token
          // still need to consume characters
          continue;
        }
      } else if c.is_digit(10) {
        return self.lexing_numeric(c);
      } else if c.is_alphabetic() || c == '_' {
        return self.lexing_identifier(c);
      }

      // special handling for dollar sign to create a lambda prefix
      if c == '$' && self.match_next_char(':') {
        self.consume_char(); // eat the colon
        return Some(self.create_token(TokenType::DollarColon, String::from("$:")));
      }

      // match for operators, maybe two or three characters.
      match c {
        '~' => return Some(self.create_token(TokenType::Wavy, String::from("~"))),
        '@' => return Some(self.create_token(TokenType::Alpha, String::from("@"))),
        '"' => return self.lexing_string(),
        '\'' => return self.lexing_char(),
        ':' => {
          return self.multiple_chars_lexing(
            (TokenType::Colon, String::from(":")),
            hashmap! {
              ':' => (
                TokenType::DoubleColon,
                String::from("::"),
                None,
              )
            },
          )
        }
        '.' => {
          return self.multiple_chars_lexing(
            (TokenType::Dot, String::from(".")),
            hashmap! {
                '.' => (
                    TokenType::DoubleDots,
                    String::from(".."),
                    Some(vec![
                      ('.', TokenType::ThreeDots, String::from("...")),
                      ('=', TokenType::DoubleDotsEqual, String::from("..=")),
                    ])
                )
            },
          )
        }
        '=' => {
          return self.multiple_chars_lexing(
            (TokenType::Equal, String::from("=")),
            hashmap! {
                '=' => (TokenType::DoubleEqual, String::from("=="), None)
            },
          )
        }
        '+' => {
          return self.multiple_chars_lexing(
            (TokenType::Plus, String::from("+")),
            hashmap! {
                '=' => (TokenType::PlusEqual, String::from("+="), None)
            },
          )
        }
        '-' => {
          return self.multiple_chars_lexing(
            (TokenType::Minus, String::from("-")),
            hashmap! {
                '=' => (TokenType::MinusEqual, String::from("-="), None),
                '>' => (TokenType::RightArrow, String::from("->"), None)
            },
          )
        }
        '*' => {
          return self.multiple_chars_lexing(
            (TokenType::Star, String::from("*")),
            hashmap! {
                '=' => (TokenType::StarEqual, String::from("*="), None),
                '*' => (TokenType::DoubleStar, String::from("**"), None)
            },
          )
        }
        '%' => {
          return self.multiple_chars_lexing(
            (TokenType::Percent, String::from("%")),
            hashmap! {
                '=' => (TokenType::PercentEqual, String::from("%="), None)
            },
          )
        }
        '!' => {
          return self.multiple_chars_lexing(
            (TokenType::Bang, String::from("!")),
            hashmap! {
                '=' => (TokenType::BangEqual, String::from("!="), None)
            },
          )
        }
        '>' => {
          return self.multiple_chars_lexing(
            (TokenType::RightAngle, String::from(">")),
            hashmap! {
                '=' => (TokenType::RightAngleEqual, String::from(">="), None),
                '>' => (
                    TokenType::DoubleRightAngle,
                    String::from(">>"),
                    Some(vec![('=', TokenType::DoubleRightAngleEqual, String::from(">>="))])
                )
            },
          )
        }
        '<' => {
          return self.multiple_chars_lexing(
            (TokenType::LeftAngle, String::from("<")),
            hashmap! {
                '=' => (TokenType::LeftAngleEqual, String::from("<="), None),
                '-' => (TokenType::LeftArrow, String::from("<-"), None),
                '<' => (TokenType::DoubleLeftAngle, String::from("<<"),
                    Some(vec![('=', TokenType::DoubleLeftAngleEqual, String::from("<<="))])
                )
            },
          )
        }
        '^' => {
          return self.multiple_chars_lexing(
            (TokenType::Caret, String::from("^")),
            hashmap! {
                '=' => (TokenType::CaretEqual, String::from("^="), None)
            },
          )
        }
        '&' => {
          return self.multiple_chars_lexing(
            (TokenType::Ampersand, String::from("&")),
            hashmap! {
                '=' => (TokenType::AmpersandEqual, String::from("&="), None),
                '&' => (TokenType::DoubleAmpersand, String::from("&&"), None)
            },
          )
        }
        '|' => {
          return self.multiple_chars_lexing(
            (TokenType::Vertical, String::from("|")),
            hashmap! {
                '=' => (TokenType::VerticalEqual, String::from("|="), None),
                '|' => (TokenType::DoubleVertical, String::from("||"), None)
            },
          )
        }
        '?' => {
          return self.multiple_chars_lexing(
            (TokenType::Question, String::from("?")),
            hashmap! {
              '.' => (
                TokenType::QuestionDot,
                String::from("?."),
                None,
              )
            },
          )
        }

        _ => continue, // accumulating characters (may be identifier)
      };
    }
    Some(self.create_token(TokenType::EOF, String::from("\0")))
  }
}

impl<'a> Iterator for Lexer<'a> {
  type Item = Token;

  /// Lexes the next `Token` and returns it.
  /// On EOF or failure, `None` will be returned.
  fn next(&mut self) -> Option<Self::Item> {
    match self.peek_next_token() {
      Some(token) => {
        if token.kind == TokenType::EOF {
          None
        } else {
          Some(token)
        }
      }
      None => None,
    }
  }
}
