use crate::core::lexer::decls::{Lexer, LexerError, TokenType};

#[test]
fn test_peek_operators_and_punctuation_tokens() {
  let mut lexer = Lexer::new(
    "@/=!;..>>\n\
        ?. ? // this is line comment \n\
        %/  &&(+.../* block comment */\n\
        ||!=*)+=::  >>=\n\
        %=  &=**,)=$:  ;==..<<=",
  );
  let mut got_token_types = Vec::<TokenType>::new();
  let mut got_lexer_errors = Vec::<&LexerError>::new();
  while let Some(token) = lexer.next() {
    // println!("{:?}", token);
    got_token_types.push(token.kind);
  }
  let token_type_answers: Vec<TokenType> = vec![
    TokenType::Alpha,
    TokenType::SlashEqual,
    TokenType::Bang,
    TokenType::Semi,
    TokenType::DoubleDots,
    TokenType::DoubleRightAngle,
    TokenType::QuestionDot,
    TokenType::Question,
    TokenType::Percent,
    TokenType::Slash,
    TokenType::DoubleAmpersand,
    TokenType::LeftParen,
    TokenType::Plus,
    TokenType::ThreeDots,
    TokenType::DoubleVertical,
    TokenType::BangEqual,
    TokenType::Star,
    TokenType::RightParen,
    TokenType::PlusEqual,
    TokenType::DoubleColon,
    TokenType::DoubleRightAngleEqual,
    TokenType::PercentEqual,
    TokenType::AmpersandEqual,
    TokenType::DoubleStar,
    TokenType::Comma,
    TokenType::RightParen,
    TokenType::Equal,
    TokenType::DollarColon,
    TokenType::Semi,
    TokenType::DoubleEqual,
    TokenType::DoubleDots,
    TokenType::DoubleLeftAngleEqual,
  ];
  assert_eq!(got_token_types.len(), token_type_answers.len());
  for i in 0..got_token_types.len() {
    assert_eq!(got_token_types[i], token_type_answers[i]);
  }

  lexer.errors.iter().for_each(|e| {
    // crate::utils::log::error(&format!("{}", e.to_string()));
    got_lexer_errors.push(e);
  });
  assert_eq!(got_lexer_errors.len(), 1);
  if let LexerError::ImbalancedPair { pos, kind } = got_lexer_errors[0] {
    assert_eq!(*kind, "parenthesis");
    assert_eq!(pos.line, 5);
    assert_eq!(pos.col, 11);
  }
}
