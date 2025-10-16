use grit::codegen::CodeGenerator;
use grit::lexer::Tokenizer;
use grit::parser::{Expr, Parser, Statement};

#[test]
fn test_tokenize_class() {
    let mut tokenizer = Tokenizer::new("class Foo");
    let tokens = tokenizer.tokenize();
    assert_eq!(tokens[0].token_type, grit::lexer::TokenType::Class);
    assert_eq!(
        tokens[1].token_type,
        grit::lexer::TokenType::Identifier("Foo".to_string())
    );
}

#[test]
fn test_tokenize_self() {
    let mut tokenizer = Tokenizer::new("self");
    let tokens = tokenizer.tokenize();
    assert_eq!(tokens[0].token_type, grit::lexer::TokenType::Self_);
}

#[test]
fn test_tokenize_dot() {
    let mut tokenizer = Tokenizer::new("self.field");
    let tokens = tokenizer.tokenize();
    assert_eq!(tokens[0].token_type, grit::lexer::TokenType::Self_);
    assert_eq!(tokens[1].token_type, grit::lexer::TokenType::Dot);
    assert_eq!(
        tokens[2].token_type,
        grit::lexer::TokenType::Identifier("field".to_string())
    );
}

#[test]
fn test_parse_class_def() {
    let input = "class Foo";
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Statement::ClassDef { name } => {
            assert_eq!(name, "Foo");
        }
        _ => panic!("Expected ClassDef"),
    }
}

#[test]
fn test_parse_method_def() {
    let input = "fn Foo > new { self.a = 1 }";
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Statement::MethodDef {
            class_name,
            method_name,
            params,
            body,
        } => {
            assert_eq!(class_name, "Foo");
            assert_eq!(method_name, "new");
            assert_eq!(params.len(), 0);
            assert_eq!(body.len(), 1);
        }
        _ => panic!("Expected MethodDef"),
    }
}

#[test]
fn test_parse_method_with_params() {
    let input = "fn Bar > new(x, y) { self.x = x }";
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Statement::MethodDef {
            class_name,
            method_name,
            params,
            ..
        } => {
            assert_eq!(class_name, "Bar");
            assert_eq!(method_name, "new");
            assert_eq!(params.len(), 2);
            assert_eq!(params[0], "x");
            assert_eq!(params[1], "y");
        }
        _ => panic!("Expected MethodDef"),
    }
}

#[test]
fn test_parse_self_field_assignment() {
    let input = "self.a = 1";
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Statement::Assignment { name, value } => {
            assert_eq!(name, "self.a");
            assert_eq!(*value, Expr::Integer(1));
        }
        _ => panic!("Expected Assignment"),
    }
}

#[test]
fn test_parse_method_call() {
    let input = "Foo.new";
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Statement::Expression(expr) => match expr {
            Expr::MethodCall {
                object,
                method,
                args,
            } => {
                assert!(matches!(**object, Expr::Identifier(ref s) if s == "Foo"));
                assert_eq!(method, "new");
                assert_eq!(args.len(), 0);
            }
            _ => panic!("Expected MethodCall"),
        },
        _ => panic!("Expected Expression"),
    }
}

#[test]
fn test_generate_simple_class() {
    let input = "class Foo\nfn Foo > new { self.a = 1 }\nf = Foo.new";
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    let code = CodeGenerator::generate_program(&program);

    assert!(code.contains("struct Foo"));
    assert!(code.contains("impl Foo"));
    assert!(code.contains("fn new() -> Self"));
    assert!(code.contains("Self {"));
    assert!(code.contains("a: 1"));
    assert!(code.contains("let f = Foo::new()"));
}

#[test]
fn test_generate_class_with_method() {
    let input = "class Foo\nfn Foo > new { self.a = 1 }\nfn Foo > get { a }";
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    let code = CodeGenerator::generate_program(&program);

    assert!(code.contains("struct Foo"));
    assert!(code.contains("impl Foo"));
    assert!(code.contains("fn new() -> Self"));
    assert!(code.contains("fn get(&self) -> i64"));
    assert!(code.contains("self.a"));
}
