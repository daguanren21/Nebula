grammar NebulaParser;

// -------------------- Parser Definition
entry_file: global_statement* EOF;

global_statement
  : use_stmt
  | enum_stmt
  | function_stmt
  ;

statement
  : expression_stmt
  | function_stmt
  | return_stmt
  | 'break' ';'
  | 'continue' ';'
  ;

// ------------ Use statement
use_stmt
  : 'use' use_endpoint ';' // use_single_stmt
  | 'use' use_tree // use_collection_stmt
  ;
use_endpoint
  : IDENTIFIER ('as' IDENTIFIER)?
  | use_path ('as' IDENTIFIER)?
  | use_path '.' use_endpoint
  | use_path use_tree
  ;
use_path
  : (IDENTIFIER | 'crate') ('::' IDENTIFIER)*
  ;
use_tree
  : '{' (use_endpoint ',')+ '}'
  ;

// ------------ Enum statement
enum_stmt
  : 'enum' IDENTIFIER '{'
      enum_items*
    '}'
  ;
enum_items
  : enum_item (',' enum_item ','?)*
  ;
enum_item
  : IDENTIFIER enum_item_type_decl?
  ;
enum_item_type_decl
  : '(' type_annotation (',' type_annotation ','?)* ')'
  | '{' enum_item_field (',' enum_item_field ','?)* '}'
  ;
enum_item_field
  : IDENTIFIER ':' type_annotation
  ;

// ------------ Function statement
function_stmt
  : 'pub'? 'fn' IDENTIFIER
     // Todo: Generics parameters
     // Todo: Function parameters
    func_body
  ;
func_body
  : '{' statement* '}'
  ;
return_stmt
  : 'return' simple_expression ';'
  ;

// ------------ Expression
expression
  : simple_expression
  | expression_with_block
  ;
simple_expression
  : literal_expression
  | path_expression
  | simple_expression '.' path_expression_segement '(' call_params? ')'         // ModuleCallExpression
  | simple_expression '.' IDENTIFIER                                            // FieldExpression
  | simple_expression '.' tuple_index                                           // TupleIndexingExpression
  | simple_expression '.' 'await'                                               // AwaitExpression
  | simple_expression '(' call_params? ')'                                      // CallExpression
  | simple_expression '[' simple_expression ']'                                 // IndexExpression
  | '&' simple_expression                                                       // ReferenceExpression
  | '*' simple_expression                                                       // DereferenceExpression
  | ('-' | '!') simple_expression                                               // NegationExpression
  | simple_expression 'as' type_parameters                                      // TypeCastExpression
  | simple_expression ('*' | '/' | '%') simple_expression                       // ArithmeticOrLogicalExpression
  | simple_expression ('+' | '-') simple_expression                             // ArithmeticOrLogicalExpression
  | simple_expression ('<<' | '>>') simple_expression                           // ArithmeticOrLogicalExpression
  | simple_expression '&' simple_expression                                     // ArithmeticOrLogicalExpression
  | simple_expression '^' simple_expression                                     // ArithmeticOrLogicalExpression
  | simple_expression '|' simple_expression                                     // ArithmeticOrLogicalExpression
  | simple_expression ('==' | '!=' | '>' | '<' | '>=' | '<=') simple_expression // ComparisonExpression
  | simple_expression '&&' simple_expression                                    // LazyBooleanExpression
  | simple_expression '||' simple_expression                                    // LazyBooleanExpression
  | simple_expression ('+=' | '-=' | '*=' | '/=' | '%=' | '&=' | '|=' |
    '^=' | '<<=' | '>>=') simple_expression                                     // CompoundAssignmentExpression
  | simple_expression ('..' | '...') simple_expression                          // RangeExpression
  ;
expression_stmt
  : simple_expression ';'
  | expression_with_block
  ;
literal_expression
  : DECIMAL_LIT
  | BINARY_LIT
  | OCTAL_LIT
  | HEX_LIT
  | FLOAT_LIT
  | EXPONENT_LIT
  | CHAR_LIT
  | STRING_LIT
  | 'true'
  | 'false'
  | '[' (simple_expression (',' simple_expression)*)? ']' // Arrayliteral
  ;
expression_with_block
  : block_expression
  | if_expression
  | loop_expression
  // Todo: Match expression
  ;
block_expression
  : '{' statement* '}'
  ;
if_expression
   : 'if' simple_expression block_expression ( 'else' (block_expression | if_expression) )?
   ;
loop_expression
  : infinite_loop_expression
  | while_loop_expression
  | for_loop_expression
  ;
infinite_loop_expression
  : 'loop' block_expression
  ;
while_loop_expression
  : 'while' simple_expression block_expression
  ;
for_loop_expression
  : 'for' for_loop_alias (',' for_loop_alias)? 'in' simple_expression block_expression
  ;
for_loop_alias
  : IDENTIFIER | '_'
  ;
path_expression
  : path_expression_segement ('::' path_expression_segement)*
  ;
path_expression_segement
  : path_identifier_segment type_parameters?
  ;
path_identifier_segment
  : IDENTIFIER
  | 'super'
  | 'self'
  | 'crate'
  ;

// ------------- Expression fragments
call_params
  : simple_expression (',' expression)*
  ;
tuple_index
  : DECIMAL_LIT
  ;

// ------------ Type annotation
type_annotation
  : primitive_types
  | IDENTIFIER type_parameters?
  ;
primitive_types
  : INTEGER_TYPES
  | FLOAT_TYPES
  | BOOL_TYPE
  | CHAR_TYPE
  ;
type_parameters
  : '<' type_annotation (',' type_annotation)* '>'
  ;

// -------------------- Lexer Definition

// keywords
USE : 'use';
PUB : 'pub';
AS : 'as';
IF : 'if';
ELSE : 'else';
LOOP: 'loop';
WHILE: 'while';
FOR : 'for';
IN : 'in';
MATCH : 'match';
BREAK : 'break';
CONTINUE : 'continue';
VAR : 'var';
CONST : 'const';
FN : 'fn';
RETURN : 'return';
GUARD : 'guard';
STRUCT : 'struct';
TRAIT : 'trait';
ENUM : 'enum';
IMPL : 'impl';
ASYNC : 'async';
AWAIT : 'await';

// Literal reserved words
TRUE : 'true';
FALSE : 'false';
NIL : 'nil';
CRATE: 'crate';

// primitive types
fragment INTEGER_BITS: '8' | '16' | '32' | '64' | '128' | 'size';
INTEGER_TYPES: ('i' INTEGER_BITS) | ('u' INTEGER_BITS) ;
FLOAT_TYPES: 'f32' | 'f64';
BOOL_TYPE: 'bool';
CHAR_TYPE: 'char';

// punctuations
L_PAREN: '(';
R_PAREN: ')';
L_BRACE: '{';
R_BRACE: '}';
L_BRACKET: '[';
R_BRACKET: ']';
COMMA: ',';
SEMI: ';';
COLON: ':';

// operators
DOUBLE_COLON: '::';
DOT: '.';
EQUAL: '=';
DOUBLE_EQUAL: '==';
BANG_EQUAL: '!=';
PLUS: '+';
MINUS: '-';
STAR: '*';
DOUBLE_STAR: '**';
SLASH: '/';
PERCENT: '%';
ALPHA: '@';
WAVY: '~';
CARET: '^';
AMPERSAND: '&';
BANG: '!';
VERTICAL: '|';
LEFT_ANGLE: '<';
RIGHT_ANGLE: '>';
DOUBLE_LEFT_ANGLE: '<<';
DOUBLE_RIGHT_ANGLE: '>>';
DOUBLE_AMPERSAND: '&&';
DOUBLE_VERTICAL: '||';
LEFT_ANGLE_EQUAL: '<=';
RIGHT_ANGLE_EQUAL: '>=';
LEFT_ARROW: '<-';
RIGHT_ARROW: '->';
PLUS_EQUAL: '+=';
MINUS_EQUAL: '-=';
STAR_EQUAL: '*=';
SLASH_EQUAL: '/=';
PERCENT_EQUAL: '%=';
DOUBLE_LEFT_ANGLE_EQUAL: '<<=';
DOUBLE_RIGHT_ANGLE_EQUAL: '>>=';
AMPERSAND_EQUAL: '&=';
VERTICAL_EQUAL: '|=';
CARET_EQUAL: '^=';
DOUBLE_DOTS: '..';
THREE_DOTS: '...';

// Lambda label is so specific that we can easily parse
LAMBDA_LABEL: '$:';
ANNONYMOUS_ALIAS: '_';

IDENTIFIER: LETTER (LETTER | UNICODE_DIGIT)*;

// Number literals
DECIMAL_LIT: '0' | [1-9] [0-9]*;
BINARY_LIT: '0' [bB] BIN_DIGIT+;
OCTAL_LIT: '0' OCTAL_DIGIT+;
HEX_LIT: '0' [xX]  HEX_DIGIT+;
FLOAT_LIT: DECIMAL_DIGITS '.' DECIMAL_DIGITS;
EXPONENT_LIT: (DECIMAL_LIT | FLOAT_LIT) [eE] DECIMAL_LIT;
CHAR_LIT: '\'' ( SINGLE_QUOTE_STRING_ESCAPE | ~[\\\r\n'] ) '\'';
STRING_LIT: '"' ( DOUBLE_QUOTE_STRING_ESCAPE | ~[\\\r\n"] )* '"';

// hidden (white spaces / comments)
LINE_COMMENT: ('//' ~[\r\n]*) -> channel(HIDDEN);
WHITESPACE: [\p{Zs}] -> channel(HIDDEN);
NEWLINE: ('\r\n' | [\r\n]) -> channel(HIDDEN);

fragment SINGLE_QUOTE_STRING_ESCAPE: '\\' [tnr\\'];
fragment DOUBLE_QUOTE_STRING_ESCAPE: '\\' [tnr\\"];
fragment DECIMAL_DIGITS: [0-9]+;
fragment OCTAL_DIGIT: [0-7];
fragment HEX_DIGIT: [0-9a-fA-F];
fragment BIN_DIGIT: [01];
fragment LETTER: UNICODE_LETTER | '_';
fragment UNICODE_DIGIT: [\p{Nd}];
fragment UNICODE_LETTER: [\p{L}];
