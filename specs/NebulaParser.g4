grammar NebulaParser;

// -------------------- Parser Definition
entry_file: top_statement* EOF;

top_statement
  : use_stmt
  | enum_stmt
  | function_stmt
  | struct_def_stmt
  | trait_def_stmt
  | impl_def_stmt
  ;

statement
  : expression_stmt
  | var_decl_stmt
  | return_stmt
  | 'break' expression? ';'
  | 'continue' ';'
  | function_stmt
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
  : IDENTIFIER ('::' IDENTIFIER)*
  | 'crate' ('::' IDENTIFIER)+
  ;
use_tree
  : '{' (use_endpoint ',')+ '}'
  ;

// ------------ Enum statement
enum_stmt
  : 'pub'? 'enum' IDENTIFIER '{' enum_items* '}'
  ;
enum_items
  : enum_item (',' enum_item ','?)*
  ;
enum_item
  : IDENTIFIER enum_item_type_decl?
  ;
enum_item_type_decl
  : '{' IDENTIFIER (',' IDENTIFIER ','?)* '}'
  ;

// ------------ Variable declaration statement
var_decl_stmt
  : 'var' var_decl_unit (',' var_decl_unit)* ';'
  | 'const' const_decl_unit (',' const_decl_unit)* ';'
  ;
const_decl_unit
  : IDENTIFIER '=' expression
  | destruct_decl_unit
  ;
var_decl_unit
  : IDENTIFIER ('=' expression)?
  | destruct_decl_unit
  ;
destruct_decl_unit
  : '[' IDENTIFIER (',' IDENTIFIER)* (',' '...' (IDENTIFIER | destruct_decl_unit))? ']' '=' expression
  ;

// ------------ Function statement
function_stmt
  : 'pub'? 'async'? 'fn' IDENTIFIER function_def_params? func_body
  ;
function_def_params
  : '(' function_params ')'
  ;
function_params
  : IDENTIFIER (',' IDENTIFIER)* (',' '...' IDENTIFIER)?
  | '...' IDENTIFIER
  ;
func_body
  : '{' statement* expression? '}'
  ;
return_stmt
  : 'return' expression ';'
  ;

// ------------ Expression
expression
  : normal_expression
  | expression_with_block
  | struct_init_expression
  ;
normal_expression // without struct init expression
  : '(' expression ')'                                          // GroupingExpression
  | simple_literal
  | array_literal
  | name_path_expression
  | lamdba_expression
  | 'await' expression                                          // AwaitExpression
  | normal_expression '?.' IDENTIFIER                           // OptionalChainExpression
  | normal_expression ('.' IDENTIFIER)                          // AccessMemberFieldExpression
  | normal_expression '(' call_args? ')'                        // CallExpression
  | normal_expression '[' normal_expression ']'                 // IndexExpression
  | ('-' | '!') normal_expression                               // NegationExpression
  | normal_expression ('**') normal_expression                  // ArithmeticOrLogicalExpression
  | normal_expression ('*' | '/' | '%') normal_expression       // ArithmeticOrLogicalExpression
  | normal_expression ('+' | '-') normal_expression             // ArithmeticOrLogicalExpression
  | normal_expression ('<<' | '>>') normal_expression           // ArithmeticOrLogicalExpression
  | normal_expression '&' normal_expression                     // ArithmeticOrLogicalExpression
  | normal_expression '^' normal_expression                     // ArithmeticOrLogicalExpression
  | normal_expression '|' normal_expression                     // ArithmeticOrLogicalExpression
  | normal_expression
    ('==' | '!=' | '>' | '<' | '>=' | '<=') normal_expression   // ComparisonExpression
  | normal_expression '&&' normal_expression                    // AndBooleanExpression
  | normal_expression '||' normal_expression                    // OrBooleanExpression
  | assignment_left_hand ('+=' | '-=' | '*=' | '/=' | '%=' | '&=' | '|=' |
    '^=' | '<<=' | '>>=') expression                            // CompoundAssignmentExpression
  | assignment_left_hand '=' expression                         // AssignmentExpression
  | normal_expression ('..' | '..=') normal_expression          // RangeExpression 
  ;
lamdba_expression
  : '$:' function_params '->' expression
  ;

expression_stmt
  : normal_expression ';'
  | expression_with_block
  ;
simple_literal
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
  ;
array_literal
  : '[' (expression (',' expression)*)? ']'
  ;
expression_with_block
  : in_block_expression
  | if_expression
  | loop_expression
  | match_expression
  ;
in_block_expression
  : '{' statement* expression? '}'
  ;
if_expression
   : 'if' normal_expression in_block_expression ( 'else' (in_block_expression | if_expression) )?
   ;
loop_expression
  : infinite_loop_expression
  | while_loop_expression
  | for_loop_expression
  ;
infinite_loop_expression
  : 'loop' in_block_expression
  ;
while_loop_expression
  : 'while' normal_expression in_block_expression
  ;
for_loop_expression
  : 'for' for_loop_alias (',' for_loop_alias)?
    'in' normal_expression
    in_block_expression
  ;
for_loop_alias
  : IDENTIFIER
  | '_'
  ;
name_path_expression
  : name_path_head ('::' IDENTIFIER)*
  ;
name_path_head
  : IDENTIFIER
  | 'crate'
  | 'self'
  | 'Self'
  ;
match_expression
  : 'match' normal_expression '{'
     match_pattern+
  '}'
  ;
match_pattern
  : match_condition '=>' (expression ',' | in_block_expression)
  ;
match_condition
  : simple_literal ('|' simple_literal)*      // simple literal as pattern
  | DECIMAL_LIT ('..' | '..=') DECIMAL_LIT    // range as pattern
  | enum_pattern                              // Enum pattern
  | '_'                                       // fallback pattern
  ;
enum_pattern
  : name_path_head '::' IDENTIFIER
  ;
struct_init_expression
  : (IDENTIFIER | 'struct') '{' (struct_init_field ',')+ '}'
  | 'new' IDENTIFIER
  ;
struct_init_field
  : IDENTIFIER '=' expression
  ;

// ------------- Expression fragments
call_args
  : expression (',' expression)*
  ;
assignment_left_hand
  : IDENTIFIER
  | IDENTIFIER ('.' IDENTIFIER)*
  | IDENTIFIER '[' (normal_expression | expression_with_block) ']'
  | array_destruct
  ;
array_destruct
  : '['
        IDENTIFIER (',' IDENTIFIER)*
        (',' '...' (IDENTIFIER | array_destruct))?
    ']'
  ;

// -------------- Struct Defintion
struct_def_stmt
  : 'struct' IDENTIFIER '{' struct_def_field+ '}'
  ;
struct_def_field
  : ('pub'? IDENTIFIER '?'? ';')
  ;

// -------------- Trait Definition
trait_def_stmt
  : 'trait' IDENTIFIER '{' trait_def_field+ '}'
  ;
method_params
  : '(' function_params? ')'
  | '(' 'self' (',' IDENTIFIER)* ')'
  ;
trait_def_field
  : 'async'? IDENTIFIER method_params? ';'
  ;

// -------------- Impl Definition
impl_def_stmt
  : 'impl' IDENTIFIER ('for' IDENTIFIER)? '{' impl_def_field+ '}'
  ;
impl_def_field
  : 'async'? IDENTIFIER method_params? '{' statement* '}'
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
STRUCT : 'struct';
NEW: 'new';
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
SELF: 'self';
SELF_BIG: 'Self';

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
QUESTION: '?';
QUESTION_DOT: '?.';

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
EXPONENT_LIT: (DECIMAL_LIT | FLOAT_LIT) [eE] '-'? DECIMAL_LIT;
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
