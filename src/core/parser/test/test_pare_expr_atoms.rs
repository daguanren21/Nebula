#[test]
fn test_parse_expression_simple_literal() {
  use crate::core::{
    parser::decls::Parser,
    shared::ast::{
      expressions::{Expression, SimpleLiteral, NormalExpression}
    }
  };
  let mut parser = Parser::new(r#"0xEFE30e 0B10110 07132 12 3.14 6.1e-8 "给岁月以文明" '$'"#);
  let mut expr_list = Vec::<Expression>::new();
  while let Some(expr) = parser.parse_expression_simple_literal() {
    expr_list.push(expr)
  }
  let answer_list: Vec<SimpleLiteral> = vec![
    SimpleLiteral::HexLiteral(String::from("0xEFE30e")),
    SimpleLiteral::BinaryLiteral(String::from("0b10110")),
    SimpleLiteral::OctalLiteral(String::from("07132")),
    SimpleLiteral::DecimalLiteral(String::from("12")),
    SimpleLiteral::FloatLiteral(String::from("3.14")),
    SimpleLiteral::ExponentLiteral(String::from("6.1e-8")),
    SimpleLiteral::StringLiteral(String::from("给岁月以文明")),
    SimpleLiteral::CharLiteral(String::from("$")),
  ];
  assert_eq!(expr_list.len(), answer_list.len());
  for i in 0..expr_list.len() {
    if let Expression::Normal(
      NormalExpression::SimpleLiteral(
        simple_lit, _
      )
    ) = &expr_list[i] {
      assert_eq!(simple_lit, &answer_list[i]);
    }
  }
}
