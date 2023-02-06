use crate::core::shared::ast::expressions::{NamePathExpression, NamePathHead};
use crate::core::shared::ast::Identifier;
use crate::core::{
  lexer::decls::{Lexer, Token, TokenType},
  shared::{
    ast::{
      expressions::{Expression, NormalExpression, SimpleLiteral},
      Position,
    },
    compile_errors::CompileError,
    nebula_interal_err,
  },
};

struct ParsingTokenMeta {
  raw: String,
  kind: TokenType,
  pos: Position,
}

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

  fn match_current_token_types(
    ref_current_token: &Option<Token>,
    token_types: Vec<TokenType>,
  ) -> bool {
    if let Some(current_token) = ref_current_token {
      token_types
        .iter()
        .find(|&token_type| *token_type == current_token.kind)
        .is_some()
    } else {
      false
    }
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

  fn get_current_token_meta_and_move_next(
    ref_lexer: &mut Lexer,
    ref_last_token: &mut Option<Token>,
    ref_current_token: &mut Option<Token>,
    current_token_desc: &str,
  ) -> ParsingTokenMeta {
    let current_pos = Parser::get_token_pos(ref_current_token).expect(
      nebula_interal_err(format!("expect receiving position of {}!", current_token_desc).as_str())
        .as_str(),
    );
    let current_kind = Parser::get_token_kind(ref_current_token).expect(
      nebula_interal_err(
        format!("expect receiving token type of {}!", current_token_desc).as_str(),
      )
      .as_str(),
    );
    let current_raw = Parser::get_token_raw(ref_current_token).expect(
      nebula_interal_err(format!("expect receiving token raw of {}!", current_token_desc).as_str())
        .as_str(),
    );
    Parser::move_to_next_token(ref_lexer, ref_last_token, ref_current_token);
    ParsingTokenMeta {
      pos: current_pos,
      kind: current_kind,
      raw: current_raw,
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

  fn get_single_bare_name_head(
    path_expr_head_token_meta: &ParsingTokenMeta,
  ) -> Option<NamePathHead> {
    match path_expr_head_token_meta.kind {
      TokenType::Identifier => Some(NamePathHead::Identifier(Identifier {
        name: path_expr_head_token_meta.raw.clone(),
        pos: path_expr_head_token_meta.pos,
      })),
      TokenType::Crate => Some(NamePathHead::CrateSymbol(path_expr_head_token_meta.pos)),
      TokenType::_self_ => Some(NamePathHead::SelfSymbol(path_expr_head_token_meta.pos)),
      TokenType::_Self_ => Some(NamePathHead::BigSelfSymbol(path_expr_head_token_meta.pos)),
      _ => None,
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
        TokenType::True => {
          let lit_token = Parser::get_current_token_meta_and_move_next(
            &mut self.lexer,
            &mut self.last_token,
            &mut self.current_token,
            "bool literal true",
          );
          Some(Expression::NormalExpression(
            NormalExpression::SimpleLiteral(SimpleLiteral::BooleanLiteral(true), lit_token.pos),
          ))
        }
        TokenType::False => {
          let lit_token = Parser::get_current_token_meta_and_move_next(
            &mut self.lexer,
            &mut self.last_token,
            &mut self.current_token,
            "bool literal false",
          );
          Some(Expression::NormalExpression(
            NormalExpression::SimpleLiteral(SimpleLiteral::BooleanLiteral(false), lit_token.pos),
          ))
        }
        TokenType::DecimalInteger => {
          let lit_token = Parser::get_current_token_meta_and_move_next(
            &mut self.lexer,
            &mut self.last_token,
            &mut self.current_token,
            "decimal integer literal",
          );
          Some(Expression::NormalExpression(
            NormalExpression::SimpleLiteral(
              SimpleLiteral::DecimalLiteral(lit_token.raw),
              lit_token.pos,
            ),
          ))
        }
        TokenType::OctalInteger => {
          let lit_token = Parser::get_current_token_meta_and_move_next(
            &mut self.lexer,
            &mut self.last_token,
            &mut self.current_token,
            "octal integer literal",
          );
          Some(Expression::NormalExpression(
            NormalExpression::SimpleLiteral(
              SimpleLiteral::OctalLiteral(lit_token.raw),
              lit_token.pos,
            ),
          ))
        }
        TokenType::HexadecimalInteger => {
          let lit_token = Parser::get_current_token_meta_and_move_next(
            &mut self.lexer,
            &mut self.last_token,
            &mut self.current_token,
            "hexadecimal integer literal",
          );
          Some(Expression::NormalExpression(
            NormalExpression::SimpleLiteral(
              SimpleLiteral::HexLiteral(lit_token.raw),
              lit_token.pos,
            ),
          ))
        }
        TokenType::BinaryInteger => {
          let lit_token = Parser::get_current_token_meta_and_move_next(
            &mut self.lexer,
            &mut self.last_token,
            &mut self.current_token,
            "binary integer literal",
          );
          Some(Expression::NormalExpression(
            NormalExpression::SimpleLiteral(
              SimpleLiteral::BinaryLiteral(lit_token.raw),
              lit_token.pos,
            ),
          ))
        }
        TokenType::Exponent => {
          let lit_token = Parser::get_current_token_meta_and_move_next(
            &mut self.lexer,
            &mut self.last_token,
            &mut self.current_token,
            "float number in exponent format",
          );
          Some(Expression::NormalExpression(
            NormalExpression::SimpleLiteral(
              SimpleLiteral::ExponentLiteral(lit_token.raw),
              lit_token.pos,
            ),
          ))
        }
        TokenType::Float => {
          let lit_token = Parser::get_current_token_meta_and_move_next(
            &mut self.lexer,
            &mut self.last_token,
            &mut self.current_token,
            "float number",
          );
          Some(Expression::NormalExpression(
            NormalExpression::SimpleLiteral(
              SimpleLiteral::FloatLiteral(lit_token.raw),
              lit_token.pos,
            ),
          ))
        }
        TokenType::Char => {
          let lit_token = Parser::get_current_token_meta_and_move_next(
            &mut self.lexer,
            &mut self.last_token,
            &mut self.current_token,
            "character",
          );
          Some(Expression::NormalExpression(
            NormalExpression::SimpleLiteral(
              SimpleLiteral::CharLiteral(lit_token.raw),
              lit_token.pos,
            ),
          ))
        }
        TokenType::String => {
          let lit_token = Parser::get_current_token_meta_and_move_next(
            &mut self.lexer,
            &mut self.last_token,
            &mut self.current_token,
            "character",
          );
          Some(Expression::NormalExpression(
            NormalExpression::SimpleLiteral(
              SimpleLiteral::StringLiteral(lit_token.raw),
              lit_token.pos,
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
    let left_paren_token = Parser::get_current_token_meta_and_move_next(
      &mut self.lexer,
      &mut self.last_token,
      &mut self.current_token,
      "left parenthesis",
    ); // moves over this '('
    if let Some(expression) = self.parse_expression() {
      if Parser::match_current_token_type(&mut self.current_token, TokenType::RightParen) {
        let right_paren_token = Parser::get_current_token_meta_and_move_next(
          &mut self.lexer,
          &mut self.last_token,
          &mut self.current_token,
          "right parenthesis",
        ); // moves over this ')'
        return Some(Expression::NormalExpression(NormalExpression::Grouping(
          Box::new(expression),
          left_paren_token.pos,
          right_paren_token.pos,
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
          pos: left_paren_token.pos,
        })
    }
    None
  }

  pub fn parse_expression_array_literal(&mut self) -> Option<Expression> {
    if !Parser::match_current_token_type(&mut self.current_token, TokenType::LeftBracket) {
      return None;
    }
    let left_bracket_token = Parser::get_current_token_meta_and_move_next(
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
        let right_bracket_token = Parser::get_current_token_meta_and_move_next(
          &mut self.lexer,
          &mut self.last_token,
          &mut self.current_token,
          "right bracket",
        ); // moves over this ']'
        return Some(Expression::NormalExpression(
          NormalExpression::ArrayLiteral(
            expr_list,
            left_bracket_token.pos,
            right_bracket_token.pos,
          ),
        ));
      } else {
        // error: Expected a comma to separate or a right parenthesis to terminate in array literal
        Parser::collect_err_on_current_token_pos(&mut self.errors, &self.current_token, |pos| {
          CompileError::ExpectedCommaOrRightBracketAfterExpression { pos }
        });
        return None;
      }
    }
    None
  }

  pub fn parse_expression_name_path_expression(&mut self) -> Option<Expression> {
    if !Parser::match_current_token_types(
      &self.current_token,
      vec![
        TokenType::Identifier,
        TokenType::Crate,
        TokenType::_self_,
        TokenType::_Self_,
      ],
    ) {
      return None;
    }
    let path_expr_head_token_meta = Parser::get_current_token_meta_and_move_next(
      &mut self.lexer,
      &mut self.last_token,
      &mut self.current_token,
      "path expression head",
    ); // moves over this path expression head
    let name_head = Parser::get_single_bare_name_head(&path_expr_head_token_meta)
      .expect(nebula_interal_err("expected creating name header struct but failed").as_str());
    if !Parser::match_current_token_type(&self.current_token, TokenType::DoubleColon) {
      // Only single bare head here: Identifier, 'crate', 'self' or 'Self'
      return Some(Expression::NormalExpression(
        NormalExpression::NamePathExpression(NamePathExpression {
          head: name_head,
          suffix: None,
        }),
      ));
    }
    let mut suffix = Vec::<Identifier>::new();
    while Parser::match_current_token_type(&self.current_token, TokenType::DoubleColon) {
      Parser::move_to_next_token(
        &mut self.lexer,
        &mut self.last_token,
        &mut self.current_token,
      ); // move over this '::'
      if Parser::match_current_token_type(&self.current_token, TokenType::Identifier) {
        let identifier_token = Parser::get_current_token_meta_and_move_next(
          &mut self.lexer,
          &mut self.last_token,
          &mut self.current_token,
          "path expression head",
        );
        suffix.push(Identifier {
          name: identifier_token.raw,
          pos: identifier_token.pos,
        });
      } else {
        // error: Expected an identifier after double colon in name path expression
        Parser::collect_err_on_current_token_pos(&mut self.errors, &self.current_token, |pos| {
          CompileError::ExpectedIdentifierAfterDoubleColon { pos }
        });
        return None;
      }
    }
    Some(Expression::NormalExpression(
      NormalExpression::NamePathExpression(NamePathExpression {
        head: name_head,
        suffix: Some(suffix),
      }),
    ))
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
