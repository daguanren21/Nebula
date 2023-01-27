use super::decls::Parser;
use crate::core::{
  lexer::decls::{Lexer, TokenType},
  shared::ast::{
    expressions::{Expression, NormalExpression, SimpleLiteral},
    Position,
  },
};

impl<'a> Parser<'a> {
  pub fn new(contents: &'a str) -> Self {
    Self {
      lexer: Lexer::new(contents),
      errors: Vec::new(),
    }
  }

  pub fn parse_expression_simple_literal(&mut self) -> Option<Expression> {
    if let Some(current_token) = self.lexer.peek_next_token() {
      return match current_token.kind {
        TokenType::DecimalInteger => Some(Expression::Normal(NormalExpression::SimpleLiteral(
          SimpleLiteral::DecimalLiteral(current_token.raw.clone()),
          Position::new(current_token.line, current_token.col),
        ))),
        TokenType::OctalInteger => Some(Expression::Normal(NormalExpression::SimpleLiteral(
          SimpleLiteral::OctalLiteral(current_token.raw.clone()),
          Position::new(current_token.line, current_token.col),
        ))),
        TokenType::HexadecimalInteger => Some(Expression::Normal(NormalExpression::SimpleLiteral(
          SimpleLiteral::HexLiteral(current_token.raw.clone()),
          Position::new(current_token.line, current_token.col),
        ))),
        TokenType::BinaryInteger => Some(Expression::Normal(NormalExpression::SimpleLiteral(
          SimpleLiteral::BinaryLiteral(current_token.raw.clone()),
          Position::new(current_token.line, current_token.col),
        ))),
        TokenType::Exponent => Some(Expression::Normal(NormalExpression::SimpleLiteral(
          SimpleLiteral::ExponentLiteral(current_token.raw.clone()),
          Position::new(current_token.line, current_token.col),
        ))),
        TokenType::Float => Some(Expression::Normal(NormalExpression::SimpleLiteral(
          SimpleLiteral::FloatLiteral(current_token.raw.clone()),
          Position::new(current_token.line, current_token.col),
        ))),
        TokenType::Char => Some(Expression::Normal(NormalExpression::SimpleLiteral(
          SimpleLiteral::CharLiteral(current_token.raw.clone()),
          Position::new(current_token.line, current_token.col),
        ))),
        TokenType::String => Some(Expression::Normal(NormalExpression::SimpleLiteral(
          SimpleLiteral::StringLiteral(current_token.raw.clone()),
          Position::new(current_token.line, current_token.col),
        ))),
        _ => None,
      };
    }
    None
  }
}
