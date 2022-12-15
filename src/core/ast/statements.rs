use super::expressions::Expression;

#[derive(Debug, Clone)]
pub enum Statement {
  ExpressionStatement(Expression),
}
