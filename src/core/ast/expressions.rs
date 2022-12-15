use super::{
  shared::{Identifier, Location},
  statements::Statement,
};

#[derive(Debug, Clone)]
pub enum Expression {
  Normal(NormalExpression),
  WithBlock(ExpressionWithBlock),
}

#[derive(Debug, Clone)]
pub enum NormalExpression {
  /// A Expression with parenthesis. <br/>
  /// Properties: expression, start, end
  Grouping(Box<Expression>, Location, Location),
  /// A simple literal. such as a number, string, etc. <br/>
  /// Properties: literal, token location
  SimpleLiteral(SimpleLiteral, Location),
  /// An array literal. such as `[3.14, some_returns(), arr[4]]`. <br/>
  /// Properties: elements, start, end             
  ArrayLiteral(Vec<Expression>, Location, Location),
  /// A Path expression. 
  /// Usually used to access some fields under a namespace created by `struct`. <br/>
  /// Examples: `a::b::c`, `self::a::b`, `crate::a::b` <br/>
  /// Properties: path expression
  PathExpression(PathExpression),
  /// A lambda expression. Prefix with `$:` <br/>
  /// Examples: `$: a, b -> a + b` <br/>
  LambdaExpression(LambdaExpression),
  /// A Await expression. Prefix with `await`. <br/>
  /// Examples: `await a`, `await a()` <br/>
  /// Properties: expression returns a `Promise`
  AwaitExpression(Box<Expression>),
  /// A Get expression. <br/>
  /// Examples: `a.b` <br/>
  /// Properties: source, field, optional
  GetExpression(Box<NormalExpression>, Identifier, bool),
  /// A Call expression. <br/>
  /// Examples: `a()`, `a(1, 2, 3)` <br/>
  /// Properties: source, arguments
  CallExpression(Box<NormalExpression>, Vec<NormalExpression>),
  /// A Index expression. <br/>
  /// Examples: `a[some_var]`, `a[1 + 2]` <br/>
  /// Properties: source, index
  IndexExpression(Box<NormalExpression>, Box<NormalExpression>),
  /// A Unary expression. <br/>
  /// Examples: `-a`, `!a` <br/>
  /// Properties: expression, unary operator
  UnaryExpression(Box<NormalExpression>, UnaryOperator),
  /// A Binary expression. <br/>
  /// Examples: `a + b`, `a * b` <br/>
  /// Properties: left hand, binary operator, right hand
  BinaryExpression(Box<NormalExpression>, BinaryOperator, Box<NormalExpression>),
  /// A Assignment expression. <br/>
  /// Examples: `a = 1`, `a.b = 1`, `a[0] = 1`, `[a, b] = [1, 2]` <br/>
  AssignmentExpression(AssignmentLeftHand, Box<Expression>),
  /// A Compound assignment expression. <br/>
  /// Examples: `a += 1`, `a -= 1`, `a *= 1`, `a /= 1`, `a %= 1`, <br/>
  /// `a **= 1`, `a &= 1`, `a |= 1`, `a ^= 1`, <br/>
  /// `a <<= 1`, `a >>= 1`, `a &&= 1`, `a ||= 1` <br/>
  /// Properties: left hand, compound assignment operator, right hand
  CompoundAssignmentExpression(AssignmentLeftHand, CompoundAssignmentOperator, Box<Expression>),
  /// A Range expression. <br/>
  /// Examples: `1..5`, `1..=5` <br/>
  /// Properties: start, end, inclusive
  RangeExpression(Box<NormalExpression>, Box<NormalExpression>, bool),
}

#[derive(Debug, Clone)]
pub enum ExpressionWithBlock {
  // Todo: list all kinds of expressions with block
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
  SelfSymbol(Location),
  CrateSymbol(Location),
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
  Addition,            // +=
  Subtraction,         // -=
  Multiplication,      // *=
  Division,            // /=
  Modulo,              // %=
  Exponent,            // **=
  BitwiseAnd,          // &=
  BitwiseOr,           // |=
  BitwiseXor,          // ^=
  BitwiseShiftLeft,    // <<=
  BitwiseShiftRight,   // >>=
  LogicalAnd,          // &&=
  LogicalOr,           // ||=
}

#[derive(Debug, Clone)]
pub enum AssignmentLeftHand {
  /// Maybe a variable name.
  ///
  /// Example: `a = 1` <br/>
  Identifier(Identifier),
  /// Deconstructing an array into several variables.
  ///
  /// Example: `[a, b] = [1, 2]` <br/>
  /// Properties: variables' identifiers
  Destruct(ArrayDestructAssign),
  /// Accessing a field of another expression, must be a safe get rather than an optional get.
  ///
  /// Example: `a.b = 1` <br/>
  /// Properties: Get expression itself
  GetExpression(Box<Expression>),
  /// Indexing an expression.
  ///
  /// Example: `a[0] = 1` <br/>
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
