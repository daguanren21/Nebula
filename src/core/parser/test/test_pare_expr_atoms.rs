#[test]
fn test_parse_expression_simple_literal() {
  use crate::core::{
    parser::impls::Parser,
    shared::ast::expressions::{
      Expression,
      Expression::NormalExpression,
      NormalExpression::SimpleLiteral,
      SimpleLiteral as SimpleLiteralStruct,
      SimpleLiteral::{
        BinaryLiteral, CharLiteral, DecimalLiteral, ExponentLiteral, FloatLiteral, HexLiteral,
        OctalLiteral, StringLiteral,
      },
    },
  };
  let mut parser = Parser::new(r#"0xEF 0B10110 07132 12 3.14 6.1e-8 "给岁月以文明" '$'"#);
  let mut expr_list = Vec::<Expression>::new();
  while let Some(expr) = parser.parse_expression_simple_literal() {
    expr_list.push(expr)
  }
  let answer_list: Vec<SimpleLiteralStruct> = vec![
    HexLiteral(String::from("0xEF")),
    BinaryLiteral(String::from("0b10110")),
    OctalLiteral(String::from("07132")),
    DecimalLiteral(String::from("12")),
    FloatLiteral(String::from("3.14")),
    ExponentLiteral(String::from("6.1e-8")),
    StringLiteral(String::from("给岁月以文明")),
    CharLiteral(String::from("$")),
  ];
  assert_eq!(expr_list.len(), answer_list.len());
  for i in 0..expr_list.len() {
    if let NormalExpression(SimpleLiteral(simple_lit, _)) = &expr_list[i] {
      assert_eq!(simple_lit, &answer_list[i]);
    }
  }
}

#[test]
fn test_parse_expression_grouping() {
  use crate::core::{
    parser::impls::Parser,
    shared::ast::{
      expressions::{
        Expression::NormalExpression,
        NormalExpression::{Grouping, SimpleLiteral},
        SimpleLiteral::FloatLiteral,
      },
      Position,
    },
  };
  let mut parser = Parser::new(r#"(3.1415)"#);
  let expr_test = parser.parse_expression_grouping();

  assert_eq!(expr_test.is_some(), true);
  if let Some(NormalExpression(Grouping(expr, left_paren_pos, right_paren_pos))) = expr_test {
    if let NormalExpression(SimpleLiteral(FloatLiteral(lit_raw), lit_pos)) = expr.as_ref() {
      assert_eq!(left_paren_pos, Position { line: 1, col: 2 });
      assert_eq!(right_paren_pos, Position { line: 1, col: 9 });
      assert_eq!(*lit_raw, "3.1415".to_string());
      assert_eq!(*lit_pos, Position { line: 1, col: 8 });
    } else {
      panic!("Can not correctly parse 3.1415 as a float literal.");
    }
  } else {
    panic!("Can not correctly parse to a grouping expression.");
  }
}

#[test]
fn test_parse_expression_array_literal() {
  use crate::core::{
    parser::impls::Parser,
    shared::ast::{
      expressions::{
        Expression::NormalExpression,
        NormalExpression::{ArrayLiteral, SimpleLiteral},
        SimpleLiteral::DecimalLiteral,
      },
      Position,
    },
  };
  let mut parser = Parser::new(r#"[1, 2, 3]"#);
  let expr_test = parser.parse_expression_array_literal();

  assert_eq!(expr_test.is_some(), true);
  if let Some(NormalExpression(ArrayLiteral(expr_list, left_bracket_pos, right_bracket_pos))) =
    expr_test
  {
    assert_eq!(left_bracket_pos, Position { line: 1, col: 2 });
    assert_eq!(right_bracket_pos, Position { line: 1, col: 10 });
    assert_eq!(expr_list.len(), 3);
    for i in 0..expr_list.len() {
      if let NormalExpression(SimpleLiteral(DecimalLiteral(lit_raw), lit_pos)) = &expr_list[i] {
        assert_eq!(*lit_raw, (i + 1).to_string());
        assert_eq!(
          *lit_pos,
          Position {
            line: 1,
            col: 3 * (i + 1)
          }
        );
      } else {
        panic!("Can not correctly parse {} as a decimal literal.", i + 1);
      }
    }
  } else {
    panic!("Can not correctly parse to an array literal expression.");
  }
}
