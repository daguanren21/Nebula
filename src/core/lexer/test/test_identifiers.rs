use crate::core::lexer::decls::{Lexer, TokenType};

#[test]
fn test_peek_identifiers_and_reserved_words() {
  let mut lexer = Lexer::new(
    "a\n\
     a1_b3\n\
     hello_world\n\
     北京\n\
     Bytedance\n\
     _32test\n\
     __private\n\
     trait var const\n\
     fn true",
  );
  let mut got_pairs = Vec::<(TokenType, String)>::new();
  while let Some(token) = lexer.next() {
    // println!("{:?}", token);
    got_pairs.push((token.kind, token.raw));
  }
  let answer_pairs: Vec<(TokenType, String)> = vec![
    (TokenType::Identifier, String::from("a")),
    (TokenType::Identifier, String::from("a1_b3")),
    (TokenType::Identifier, String::from("hello_world")),
    (TokenType::Identifier, String::from("北京")),
    (TokenType::Identifier, String::from("Bytedance")),
    (TokenType::Identifier, String::from("_32test")),
    (TokenType::Identifier, String::from("__private")),
    (TokenType::Trait, String::from("trait")),
    (TokenType::Var, String::from("var")),
    (TokenType::Const, String::from("const")),
    (TokenType::Fn, String::from("fn")),
    (TokenType::True, String::from("true")),
  ];
  assert_eq!(got_pairs.len(), answer_pairs.len());
  for i in 0..got_pairs.len() {
    let (answer_type, answer_string) = &answer_pairs[i];
    assert_eq!(got_pairs[i].0, *answer_type);
    assert_eq!(got_pairs[i].1, *answer_string);
  }
}
