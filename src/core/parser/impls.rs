use crate::core::{
  lexer::decls::{Lexer, Token, TokenType},
  shared::{
    ast::{
      expressions::{Expression, NormalExpression, SimpleLiteral},
      Position,
    },
    compile_errors::CompileError,
  },
};

pub struct Parser<'a> {
  last_token: Option<Token>,
  current_token: Option<Token>,

  /// Obtain a lexer inside parser to peek next token
  pub lexer: Lexer<'a>,

  /// All the errors that occurred during the parsing.<br>
  /// (Adopted the errors that occurred during the lexing.)
  pub errors: Vec<CompileError>,
}

impl<'a> Parser<'a> {
  fn move_to_next_token(
    lexer: &mut Lexer,
    ref_last_token: &mut Option<Token>,
    ref_current_token: &mut Option<Token>,
  ) {
    if let Some(current_token) = lexer.peek_next_token() {
      let last_token = (*ref_current_token).take();
      *ref_last_token = last_token;
      *ref_current_token = Some(current_token);
    }
  }

  fn match_current_token_type(ref_current_token: &Option<Token>, token_type: TokenType) -> bool {
    if let Some(current_token) = ref_current_token {
      return current_token.kind == token_type;
    }
    false
  }

  fn get_token_pos(ref_token: &Option<Token>) -> Option<Position> {
    if let Some(token) = ref_token {
      Some(Position {
        line: token.line,
        col: token.col,
      })
    } else {
      None
    }
  }

  fn get_token_kind(ref_token: &Option<Token>) -> Option<TokenType> {
    if let Some(token) = ref_token {
      Some(token.kind)
    } else {
      None
    }
  }

  fn get_token_raw(ref_token: &Option<Token>) -> Option<String> {
    if let Some(token) = ref_token {
      Some(token.raw.clone())
    } else {
      None
    }
  }

  pub fn new(contents: &'a str) -> Self {
    let mut new_parser = Self {
      current_token: None,
      last_token: None,
      lexer: Lexer::new(contents),
      errors: Vec::new(),
    };
    Parser::move_to_next_token(
      &mut new_parser.lexer,
      &mut new_parser.last_token,
      &mut new_parser.current_token,
    );
    return new_parser;
  }

  pub fn parse_expression_simple_literal(&mut self) -> Option<Expression> {
    let current_token_kind = Parser::get_token_kind(&self.current_token);
    if let Some(token_kind) = current_token_kind {
      return match token_kind {
        TokenType::DecimalInteger => {
          let raw = Parser::get_token_raw(&self.current_token);
          let pos = Parser::get_token_pos(&self.current_token);
          Parser::move_to_next_token(
            &mut self.lexer,
            &mut self.last_token,
            &mut self.current_token,
          );
          Some(Expression::NormalExpression(
            NormalExpression::SimpleLiteral(
              SimpleLiteral::DecimalLiteral(raw.unwrap()),
              pos.unwrap(),
            ),
          ))
        }
        TokenType::OctalInteger => {
          let raw = Parser::get_token_raw(&self.current_token);
          let pos = Parser::get_token_pos(&self.current_token);
          Parser::move_to_next_token(
            &mut self.lexer,
            &mut self.last_token,
            &mut self.current_token,
          );
          Some(Expression::NormalExpression(
            NormalExpression::SimpleLiteral(
              SimpleLiteral::OctalLiteral(raw.unwrap()),
              pos.unwrap(),
            ),
          ))
        }
        TokenType::HexadecimalInteger => {
          let raw = Parser::get_token_raw(&self.current_token);
          let pos = Parser::get_token_pos(&self.current_token);
          Parser::move_to_next_token(
            &mut self.lexer,
            &mut self.last_token,
            &mut self.current_token,
          );
          Some(Expression::NormalExpression(
            NormalExpression::SimpleLiteral(SimpleLiteral::HexLiteral(raw.unwrap()), pos.unwrap()),
          ))
        }
        TokenType::BinaryInteger => {
          let raw = Parser::get_token_raw(&self.current_token);
          let pos = Parser::get_token_pos(&self.current_token);
          Parser::move_to_next_token(
            &mut self.lexer,
            &mut self.last_token,
            &mut self.current_token,
          );
          Some(Expression::NormalExpression(
            NormalExpression::SimpleLiteral(
              SimpleLiteral::BinaryLiteral(raw.unwrap()),
              pos.unwrap(),
            ),
          ))
        }
        TokenType::Exponent => {
          let raw = Parser::get_token_raw(&self.current_token);
          let pos = Parser::get_token_pos(&self.current_token);
          Parser::move_to_next_token(
            &mut self.lexer,
            &mut self.last_token,
            &mut self.current_token,
          );
          Some(Expression::NormalExpression(
            NormalExpression::SimpleLiteral(
              SimpleLiteral::ExponentLiteral(raw.unwrap()),
              pos.unwrap(),
            ),
          ))
        }
        TokenType::Float => {
          let raw = Parser::get_token_raw(&self.current_token);
          let pos = Parser::get_token_pos(&self.current_token);
          Parser::move_to_next_token(
            &mut self.lexer,
            &mut self.last_token,
            &mut self.current_token,
          );
          Some(Expression::NormalExpression(
            NormalExpression::SimpleLiteral(
              SimpleLiteral::FloatLiteral(raw.unwrap()),
              pos.unwrap(),
            ),
          ))
        }
        TokenType::Char => {
          let raw = Parser::get_token_raw(&self.current_token);
          let pos = Parser::get_token_pos(&self.current_token);
          Parser::move_to_next_token(
            &mut self.lexer,
            &mut self.last_token,
            &mut self.current_token,
          );
          Some(Expression::NormalExpression(
            NormalExpression::SimpleLiteral(SimpleLiteral::CharLiteral(raw.unwrap()), pos.unwrap()),
          ))
        }
        TokenType::String => {
          let raw = Parser::get_token_raw(&self.current_token);
          let pos = Parser::get_token_pos(&self.current_token);
          Parser::move_to_next_token(
            &mut self.lexer,
            &mut self.last_token,
            &mut self.current_token,
          );
          Some(Expression::NormalExpression(
            NormalExpression::SimpleLiteral(
              SimpleLiteral::StringLiteral(raw.unwrap()),
              pos.unwrap(),
            ),
          ))
        }
        _ => None,
      };
    }
    None
  }

  pub fn parse_expression_grouping(&mut self) -> Option<Expression> {
    if !Parser::match_current_token_type(&mut self.current_token, TokenType::LeftParen) {
      return None;
    }
    let left_paren_pos = Parser::get_token_pos(&self.current_token);
    Parser::move_to_next_token(
      &mut self.lexer,
      &mut self.last_token,
      &mut self.current_token,
    ); // moves over this '('
    if let Some(expression) = self.parse_expression() {
      if Parser::match_current_token_type(&mut self.current_token, TokenType::RightParen) {
        let right_paren_pos = Parser::get_token_pos(&self.current_token);
        Parser::move_to_next_token(
          &mut self.lexer,
          &mut self.last_token,
          &mut self.current_token,
        ); // moves over this ')'
        return Some(Expression::NormalExpression(NormalExpression::Grouping(
          Box::new(expression),
          left_paren_pos.unwrap(),
          right_paren_pos.unwrap(),
        )));
      }
    }
    None
  }

  pub fn parse_expression(&mut self) -> Option<Expression> {
    if let Some(grouping_expr) = self.parse_expression_grouping() {
      return Some(grouping_expr);
    }
    if let Some(simple_literal) = self.parse_expression_simple_literal() {
      return Some(simple_literal);
    }
    None
  }
}
