use crate::core::{
  lexer::decls::{Lexer, Token, TokenType},
  shared::{
    ast::{
      expressions::{Expression, NormalExpression, SimpleLiteral},
      Position,
    },
    compile_errors::CompileError,
    make_internal_err_str,
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

  fn collect_err_on_current_token_pos(
    ref_errors: &mut Vec<CompileError>,
    ref_current_token: &Option<Token>,
    create_err_by_pos: fn(pos: Position) -> CompileError,
  ) {
    let current_token_pos = Parser::get_token_pos(ref_current_token)
      .expect("expect recieving position of current token !");
    ref_errors.push(create_err_by_pos(current_token_pos));
  }

  fn get_current_pos_and_move_token(
    ref_lexer: &mut Lexer,
    ref_last_token: &mut Option<Token>,
    ref_current_token: &mut Option<Token>,
    current_token_desc: &str,
  ) -> Position {
    let current_pos = Parser::get_token_pos(ref_current_token).expect(
      make_internal_err_str(
        format!("expect recieving position of {}!", current_token_desc).as_str(),
      )
      .as_str(),
    );
    Parser::move_to_next_token(ref_lexer, ref_last_token, ref_current_token);
    current_pos
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
    let left_paren_pos = Parser::get_current_pos_and_move_token(
      &mut self.lexer,
      &mut self.last_token,
      &mut self.current_token,
      "left parenthesis",
    ); // moves over this '('
    if let Some(expression) = self.parse_expression() {
      if Parser::match_current_token_type(&mut self.current_token, TokenType::RightParen) {
        let right_paren_pos = Parser::get_current_pos_and_move_token(
          &mut self.lexer,
          &mut self.last_token,
          &mut self.current_token,
          "right parenthesis",
        ); // moves over this ')'
        return Some(Expression::NormalExpression(NormalExpression::Grouping(
          Box::new(expression),
          left_paren_pos,
          right_paren_pos,
        )));
      } else {
        // error: expected right parenthesis to close this grouping expression
        Parser::collect_err_on_current_token_pos(&mut self.errors, &self.current_token, |pos| {
          CompileError::ExpectedRightParenthesisAfterExpression { pos }
        });
      }
    } else {
      // error: expected expression after left parenthesis
      self
        .errors
        .push(CompileError::ExpectedExpressionAfterLeftParenthesis {
          pos: left_paren_pos,
        })
    }
    None
  }

  pub fn parse_expression_array_literal(&mut self) -> Option<Expression> {
    if !Parser::match_current_token_type(&mut self.current_token, TokenType::LeftBracket) {
      return None;
    }
    let left_bracket_pos = Parser::get_current_pos_and_move_token(
      &mut self.lexer,
      &mut self.last_token,
      &mut self.current_token,
      "left bracket",
    ); // moves over this '['
    let mut expr_list: Vec<Expression> = vec![];
    while let Some(expr) = self.parse_expression() {
      expr_list.push(expr);
      if Parser::match_current_token_type(&mut self.current_token, TokenType::Comma) {
        Parser::move_to_next_token(
          &mut self.lexer,
          &mut self.last_token,
          &mut self.current_token,
        ); // moves over this ','
      } else if Parser::match_current_token_type(&mut self.current_token, TokenType::RightBracket) {
        let right_bracket_pos = Parser::get_current_pos_and_move_token(
          &mut self.lexer,
          &mut self.last_token,
          &mut self.current_token,
          "right bracket",
        ); // moves over this ']'
        return Some(Expression::NormalExpression(
          NormalExpression::ArrayLiteral(expr_list, left_bracket_pos, right_bracket_pos),
        ));
      } else {
        // error: Expected a comma to seperate or a right parenthesis to terminate in array literal
        Parser::collect_err_on_current_token_pos(&mut self.errors, &self.current_token, |pos| {
          CompileError::ExpectedCommaOrRightBracketAfterExpression { pos }
        });
        return None;
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
    if let Some(array_literal) = self.parse_expression_array_literal() {
      return Some(array_literal);
    }
    None
  }
}
