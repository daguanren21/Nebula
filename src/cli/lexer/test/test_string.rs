use crate::cli::lexer::decls::{Lexer, TokenType};

#[test]
fn test_peek_string_tokens() {
  let mut lexer = Lexer::new(
    " 'c' '中' 'な' '\\t' '언' \"hello_world\" \"another string 2022\" \"你好中国\" \"nebula❤️\" \
        \"'dd\\t\\n\" \"something \\\"dead\\\"\" \"愛してる\" \"안녕하세요\" \"Здравствуйте\" \"नमस्ते\" ",
  );
  let mut got_pairs = Vec::<(TokenType, String)>::new();
  while let Some(token) = lexer.next() {
    // println!("{:?}", token);
    got_pairs.push((token.kind, token.raw));
  }
  let answer_pairs: Vec<(TokenType, String)> = vec![
    (TokenType::Char, String::from("c")),
    (TokenType::Char, String::from("中")),
    (TokenType::Char, String::from("な")),
    (TokenType::Char, String::from("\t")),
    (TokenType::Char, String::from("언")),
    (TokenType::String, String::from("hello_world")),
    (TokenType::String, String::from("another string 2022")),
    (TokenType::String, String::from("你好中国")),
    (TokenType::String, String::from("nebula❤️")),
    (TokenType::String, String::from("'dd\t\n")),
    (TokenType::String, String::from("something \"dead\"")),
    (TokenType::String, String::from("愛してる")),
    (TokenType::String, String::from("안녕하세요")),
    (TokenType::String, String::from("Здравствуйте")),
    (TokenType::String, String::from("नमस्ते")),
  ];
  assert_eq!(got_pairs.len(), answer_pairs.len());
  for i in 0..got_pairs.len() {
    let (answer_type, answer_string) = &answer_pairs[i];
    assert_eq!(got_pairs[i].0, *answer_type);
    assert_eq!(got_pairs[i].1, *answer_string);
  }
}
