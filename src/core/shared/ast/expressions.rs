use super::statements::Statement;
use crate::core::shared::ast::{Identifier, Position};

#[derive(Debug, Clone)]
pub enum Expression {
  Normal(NormalExpression),
  WithBlock(ExpressionWithBlock),
}

#[derive(Debug, Clone)]
pub enum NormalExpression {
  /// A Expression with parenthesis. <br>
  /// Properties: expression, start, end
  Grouping(Box<Expression>, Position, Position),
  /// A simple literal. such as a number, string, etc. <br>
  /// Properties: literal, token location
  SimpleLiteral(SimpleLiteral, Position),
  /// An array literal. such as `[3.14, some_returns(), arr[4]]`. <br>
  /// Properties: elements, start, end             
  ArrayLiteral(Vec<Expression>, Position, Position),
  /// A Path expression.
  /// Usually used to access some fields under a namespace created by `struct` or `enum`. <br>
  /// Examples: `a::b::c`, `HttpStatus::NotFound`, `self::a::b`, `crate::a::b` <br>
  /// Properties: path expression
  PathExpression(PathExpression),
  /// A lambda expression. Prefix with `$:` <br>
  /// Examples: `$: a, b -> a + b` <br>
  LambdaExpression(LambdaExpression),
  /// A Await expression. Prefix with `await`. <br>
  /// Examples: `await a`, `await a()` <br>
  /// Properties: expression returns a `Promise`
  AwaitExpression(Box<Expression>),
  /// A Get expression. <br>
  /// Examples: `a.b` <br>
  /// Properties: source, field, optional
  GetExpression(Box<NormalExpression>, Identifier, bool),
  /// A Call expression. <br>
  /// Examples: `a()`, `a(1, 2, 3)` <br>
  /// Properties: source, arguments
  CallExpression(Box<NormalExpression>, Vec<NormalExpression>),
  /// A Index expression. <br>
  /// Examples: `a[some_var]`, `a[1 + 2]` <br>
  /// Properties: source, index
  IndexExpression(Box<NormalExpression>, Box<NormalExpression>),
  /// A Unary expression. <br>
  /// Examples: `-a`, `!a` <br>
  /// Properties: expression, unary operator
  UnaryExpression(Box<NormalExpression>, UnaryOperator),
  /// A Binary expression. <br>
  /// Examples: `a + b`, `a * b` <br>
  /// Properties: left hand, binary operator, right hand
  BinaryExpression(Box<NormalExpression>, BinaryOperator, Box<NormalExpression>),
  /// A Assignment expression. <br>
  /// Examples: `a = 1`, `a.b = 1`, `a[0] = 1`, `[a, b] = [1, 2]` <br>
  AssignmentExpression(AssignmentLeftHand, Box<Expression>),
  /// A Compound assignment expression. <br>
  /// Examples: `a += 1`, `a -= 1`, `a *= 1`, `a /= 1`, `a %= 1`, <br>
  /// `a **= 1`, `a &= 1`, `a |= 1`, `a ^= 1`, <br>
  /// `a <<= 1`, `a >>= 1`, `a &&= 1`, `a ||= 1` <br>
  /// Properties: left hand, compound assignment operator, right hand
  CompoundAssignmentExpression(
    AssignmentLeftHand,
    CompoundAssignmentOperator,
    Box<Expression>,
  ),
  /// A Range expression. <br>
  /// Examples: `1..5`, `1..=5` <br>
  /// Properties: start, end, inclusive
  RangeExpression(Box<NormalExpression>, Box<NormalExpression>, bool),
}

/// Tips: we will likely use a `Box<Statement>` to represent a block.
/// Because we allow these kinds of with-block-expressions to only contain a single statement.
/// So it maybe one single statement, or a direct block expression.

#[derive(Debug, Clone)]
pub enum ExpressionWithBlock {
  SingleBlock(Vec<Statement>),
  IfExpression {
    condition: Box<Expression>,
    then_block: Box<Statement>,
    else_if: Vec<(Box<Expression>, Box<Statement>)>,
    else_block: Option<Box<Statement>>,
  },
  WhileExpression {
    condition: Box<Expression>,
    block: Box<Statement>,
  },
  ForEachExpression {
    index_var: Identifier,
    element_var: Identifier,
    iterable: Box<Expression>,
    block: Box<Statement>,
  },
  MatchExpression {
    expression: Box<Expression>,
    arms: Vec<(Vec<MatchArmPattern>, Box<Statement>)>,
  },
}

#[derive(Debug, Clone)]
pub enum SimpleLiteral {
  DecimalLiteral(i32),
  BinaryLiteral(i32),
  OctalLiteral(i32),
  HexLiteral(i32),
  StringLiteral(String),
  BooleanLiteral(bool),
  CharLiteral(char),
  FloatLiteral(f32),
  ExponentLiteral(f32),
}

#[derive(Debug, Clone)]
pub enum PathExpressionHead {
  Identifier(Identifier),
  SelfSymbol(Position),
  CrateSymbol(Position),
}

#[derive(Debug, Clone)]
pub struct PathExpression {
  pub head: PathExpressionHead,
  pub fragments: Vec<Identifier>,
}

#[derive(Debug, Clone)]
pub struct LambdaExpression {
  pub is_async: bool,
  pub params: Vec<Identifier>,
  pub body: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum UnaryOperator {
  Negation,
  Not,
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
  Addition,            // +
  Subtraction,         // -
  Multiplication,      // *
  Division,            // /
  Modulo,              // %
  Exponent,            // **
  BitwiseAnd,          // &
  BitwiseOr,           // |
  BitwiseXor,          // ^
  BitwiseShiftLeft,    // <<
  BitwiseShiftRight,   // >>
  LogicalAnd,          // &&
  LogicalOr,           // ||
  Equals,              // ==
  NotEquals,           // !=
  LessThan,            // <
  LessThanOrEquals,    // <=
  GreaterThan,         // >
  GreaterThanOrEquals, // >=
}

#[derive(Debug, Clone)]
pub enum CompoundAssignmentOperator {
  Addition,          // +=
  Subtraction,       // -=
  Multiplication,    // *=
  Division,          // /=
  Modulo,            // %=
  Exponent,          // **=
  BitwiseAnd,        // &=
  BitwiseOr,         // |=
  BitwiseXor,        // ^=
  BitwiseShiftLeft,  // <<=
  BitwiseShiftRight, // >>=
  LogicalAnd,        // &&=
  LogicalOr,         // ||=
}

#[derive(Debug, Clone)]
pub enum AssignmentLeftHand {
  /// Maybe a variable name.
  ///
  /// Example: `a = 1` <br>
  Identifier(Identifier),
  /// Deconstructing an array into several variables.
  ///
  /// Example: `[a, b] = [1, 2]` <br>
  /// Properties: variables' identifiers
  Destruct(ArrayDestructAssign),
  /// Accessing a field of another expression, must be a safe get rather than an optional get.
  ///
  /// Example: `a.b = 1` <br>
  /// Properties: Get expression itself
  GetExpression(Box<Expression>),
  /// Indexing an expression.
  ///
  /// Example: `a[0] = 1` <br>
  /// Properties: Index expression itself
  IndexExpression(Box<Expression>),
}

#[derive(Debug, Clone)]
pub struct ArrayDestructAssign {
  /// Deconstructing an array into these variables.
  pub vars: Vec<Identifier>,
  /// Deconstructing the rest of parent array into a single variable,
  /// or starting another array destructing right after.
  pub rest: Option<ArrayDestructRest>,
}

#[derive(Debug, Clone)]
pub enum ArrayDestructRest {
  /// Destructing the rest of parent array into a single variable.
  Identifier(Identifier),
  /// Maybe another array destructing.
  ///
  /// Example: `[a, b, ...[c, d]]`
  ChildRest(Box<ArrayDestructRest>),
}

#[derive(Debug, Clone)]
pub enum MatchArmPattern {
  /// Single pattern. <br>
  /// Examples: `11.6`, `true`, `0x3E` <br>
  Single(MatchSingleArm),
  /// Mutiple patterns. <br>
  /// Examples: `11.6 | 10.21` <br>
  Mutiple(Vec<MatchSingleArm>),
  /// A Range pattern. <br>
  /// Examples: `1..5`, `1..=5` <br>
  RangePattern(Box<NormalExpression>, Box<NormalExpression>, bool),
  /// Fallback pattern. <br>
  /// Examples: `_` <br>
  Fallback(Position),
}

#[derive(Debug, Clone)]
pub enum MatchSingleArm {
  Literal(SimpleLiteral),
  Identifier(Identifier),
  Path(PathExpression),
}
