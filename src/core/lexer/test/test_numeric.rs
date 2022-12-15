#[test]
fn test_peek_numeric_tokens() {
  use crate::core::lexer::decls::{Lexer, TokenType};
  let mut lexer = Lexer::new(
    "0x33FF\n\
     0X6d5a1\n\
     0b01001\n\
     0B11011\n\
     0145271\n\
     64456\n\
     18.652\n\
     73e6\n\
     1.6e5\n\
     9.88e8.2\n",
  );
  let mut got_pairs = Vec::<(TokenType, String)>::new();
  while let Some(token) = lexer.next() {
    // println!("{:?}", token);
    got_pairs.push((token.kind, token.raw));
  }
  let answer_pairs: Vec<(TokenType, String)> = vec![
    (TokenType::HexadecimalInteger, String::from("0x33FF")),
    (TokenType::HexadecimalInteger, String::from("0x6d5a1")),
    (TokenType::BinaryInteger, String::from("0b01001")),
    (TokenType::BinaryInteger, String::from("0b11011")),
    (TokenType::OctalInteger, String::from("0145271")),
    (TokenType::DecimalInteger, String::from("64456")),
    (TokenType::Float, String::from("18.652")),
    (TokenType::Exponent, String::from("73e6")),
    (TokenType::Exponent, String::from("1.6e5")),
    (TokenType::Exponent, String::from("9.88e8")),
    (TokenType::Dot, String::from(".")),
    (TokenType::DecimalInteger, String::from("2")),
  ];
  assert_eq!(got_pairs.len(), answer_pairs.len());
  for i in 0..got_pairs.len() {
    let (answer_type, answer_string) = &answer_pairs[i];
    assert_eq!(got_pairs[i].0, *answer_type);
    assert_eq!(got_pairs[i].1, *answer_string);
  }
}
