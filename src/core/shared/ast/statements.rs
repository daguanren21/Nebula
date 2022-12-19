use super::{expressions::Expression, Identifier,};

#[derive(Debug, Clone)]
pub enum Statement {
  ExpressionStatement(Expression),
  VariableDeclaration {
    is_const: bool,
    decls: Vec<(Identifier, Option<Expression>)>,
  },
  ReturnStatement(Option<Expression>),
  BreakStatement(Option<Expression>),
  ContinueStatement,
  FunctionDeclaration(FunctionDeclaration),
}

#[derive(Debug, Clone)]
pub enum TopStatement {
  UseStatement(Vec<UseEntry>),
  EnumStatement {
    name: Identifier,
    variants: Vec<Identifier>,
  },
  FunctionDeclaration(FunctionDeclaration),
  StructDeclaration {
    name: Identifier,
    fields: Vec<StructField>,
  },
  TraitDeclaration {
    name: Identifier,
    methods: Vec<FunctionSignature>,
  },
  ImplDeclaration {
    trait_name: Identifier,
    struct_name: Identifier,
    /// Properties: method implementation, is member method
    methods: Vec<(FunctionDeclaration, bool)>,
  },
}

#[derive(Debug, Clone)]
pub struct UseEntry {
  pub path: Vec<Identifier>,
  pub name: Identifier,
  pub alias: Option<Identifier>,
}

#[derive(Debug, Clone)]
pub struct FunctionDeclaration {
  is_pub: bool,
  is_async: bool,
  name: Identifier,
  params: Vec<Identifier>,
  body: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct StructField {
  name: Identifier,
  is_pub: bool,
  is_const: bool,
}

#[derive(Debug, Clone)]
pub struct FunctionSignature {
  is_pub: bool,
  is_async: bool,
  name: Identifier,
  params: Vec<Identifier>,
}

