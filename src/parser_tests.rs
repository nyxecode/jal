use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::ast::*;
use crate::token::TokenType;

#[test]
fn test_parse_switch_statement() {
    let input = r#"
        switch(x) {
            case 1:
                print("one");
                break;
            case 2:
                print("two");
                break;
            default:
                print("other");
        }
    "#;

    let mut lexer = Lexer::new(input);
    lexer.tokenize();
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    assert_eq!(program.len(), 1);
    match &program[0] {
        Statement::SwitchStatement {
            expression,
            cases,
            default,
            ..
        } => {
            assert!(matches!(**expression, Expression::Identifier { .. }));
            assert_eq!(cases.len(), 2);
            assert!(matches!(cases[0].0, Expression::Literal { .. }));
            assert_eq!(cases[0].1.len(), 2);
            assert!(matches!(cases[1].0, Expression::Literal { .. }));
            assert_eq!(cases[1].1.len(), 2);
            assert_eq!(default.as_ref().unwrap().len(), 1);
        }
        _ => panic!("Expected SwitchStatement"),
    }
}

#[test]
fn test_parse_for_of_statement() {
    let input = r#"
        for(element of array) {
            print(element);
        }
    "#;

    let mut lexer = Lexer::new(input);
    lexer.tokenize();
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    assert_eq!(program.len(), 1);
    match &program[0] {
        Statement::ForEachStatement {
            element_variable,
            iterator,
            body,
            ..
        } => {
            assert_eq!(element_variable, "element");
            assert!(matches!(**iterator, Expression::Identifier { .. }));
            assert_eq!(body.len(), 1);
        }
        _ => panic!("Expected ForEachStatement"),
    }
}

#[test]
fn test_parse_dict_literal() {
    let input = r#"
        dict books = {
            dune: {
                author: "Frank Herbert",
                title: "Dune",
                year: 1968
            },
            lotr: {
                author: "Lord of the Rings",
                title: "J.R.R Tolkin",
                year: 1950
            }
        }
    "#;

    let mut lexer = Lexer::new(input);
    lexer.tokenize();
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    assert_eq!(program.len(), 1);
    match &program[0] {
        Statement::VariableDeclaration {
            name,
            value: Some(Expression::DictLiteral { pairs, .. }),
            ..
        } => {
            assert_eq!(name, "books");
            assert_eq!(pairs.len(), 2);
            // Add more assertions for the dict literal structure
        }
        _ => panic!("Expected VariableDeclaration with DictLiteral"),
    }
}

#[test]
fn test_parse_new_expression() {
    let input = r#"
        Human anna = new Human("Anna", "Doe");
    "#;

    let mut lexer = Lexer::new(input);
    lexer.tokenize();
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    assert_eq!(program.len(), 1);
    match &program[0] {
        Statement::VariableDeclaration {
            name,
            value: Some(Expression::NewExpression {
                            class_name,
                            arguments,
                            ..
                        }),
            ..
        } => {
            assert_eq!(name, "anna");
            assert_eq!(class_name, "Human");
            assert_eq!(arguments.len(), 2);
            // Add more assertions for the arguments
        }
        _ => panic!("Expected VariableDeclaration with NewExpression"),
    }
}

#[test]
fn test_parse_this_keyword() {
    let input = r#"
        class Human {
            constructor = (string firstname, string lastname) => {
                this.firstname = firstname;
                this.lastname = lastname;
            }
        }
    "#;

    let mut lexer = Lexer::new(input);
    lexer.tokenize();
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    assert_eq!(program.len(), 1);
    match &program[0] {
        Statement::ClassDeclaration {
            name,
            members,
            ..
        } => {
            assert_eq!(name, "Human");
            assert_eq!(members.len(), 1);
            match &members[0] {
                ClassMember::Method {
                    name,
                    body,
                    ..
                } => {
                    assert_eq!(name, "constructor");
                    assert_eq!(body.len(), 2);
                    // Add more assertions for the method body
                }
                _ => panic!("Expected constructor method"),
            }
        }
        _ => panic!("Expected ClassDeclaration"),
    }
}

#[test]
fn test_invalid_syntax() {
    let input = r#"
        int x = 5
        // Missing semicolon
    "#;

    let mut lexer = Lexer::new(input);
    lexer.tokenize();
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    assert_eq!(program.len(), 1);
    assert!(!parser.get_errors().is_empty());
}

#[test]
fn test_parse_enum_declaration() {
    let input = r#"
        enum ACTION {
            RUN,
            WALK,
            SIT
        }
    "#;

    let mut lexer = Lexer::new(input);
    lexer.tokenize();
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    assert_eq!(program.len(), 1);
    match &program[0] {
        Statement::EnumDeclaration {
            name,
            variants,
            ..
        } => {
            assert_eq!(name, "ACTION");
            assert_eq!(variants.len(), 3);
            assert_eq!(variants[0], "RUN");
            assert_eq!(variants[1], "WALK");
            assert_eq!(variants[2], "SIT");
        }
        _ => panic!("Expected EnumDeclaration"),
    }
}

#[test]
fn test_parse_object_declaration() {
    let input = r#"
        object Animal = {
            string name = "Wolf";
            int age = 5;
            private bool isMoving = false;
        };
    "#;

    let mut lexer = Lexer::new(input);
    lexer.tokenize();
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    assert_eq!(program.len(), 1);
    match &program[0] {
        Statement::ObjectDeclaration {
            name,
            properties,
            ..
        } => {
            assert_eq!(name, "Animal");
            assert_eq!(properties.len(), 3);
            // Add more assertions for the properties
        }
        _ => panic!("Expected ObjectDeclaration"),
    }
}

#[test]
fn test_parse_class_declaration() {
    let input = r#"
        class Human {
            int age = 14;
            public weigt = 66;
            private string firstname = "Samantha";

            private:
                string lastname = "Mustermann";
                bool running = false;

            public:
                int height = 166;
                void run = (float speed) => {
                    running = true;
                }

                constructor = (string firstname, string lastname) => {
                    this.firstname = firstname;
                    this.lastname = lastname;
                }

            static:
                Human createHuman = (string firstname, string lastname, int age) => {
                    Human newHuman = new Human(firstname, lastname);
                    newHuman.age = age;
                    return newHuman;
                }
        }
    "#;

    let mut lexer = Lexer::new(input);
    lexer.tokenize();
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    assert_eq!(program.len(), 1);
    match &program[0] {
        Statement::ClassDeclaration {
            name,
            members,
            ..
        } => {
            assert_eq!(name, "Human");
            assert_eq!(members.len(), 7);
            // Add more assertions for the class members
        }
        _ => panic!("Expected ClassDeclaration"),
    }
}

#[test]
fn test_parse_interface_declaration() {
    let input = r#"
        Interface Animal {
            string name;

            void move = (int speed) => {};
        }
    "#;

    let mut lexer = Lexer::new(input);
    lexer.tokenize();
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    assert_eq!(program.len(), 1);
    match &program[0] {
        Statement::InterfaceDeclaration {
            name,
            members,
            ..
        } => {
            assert_eq!(name, "Animal");
            assert_eq!(members.len(), 2);
            // Add more assertions for the interface members
        }
        _ => panic!("Expected InterfaceDeclaration"),
    }
}

#[test]
fn test_parse_import_declaration() {
    let input = r#"
        import { limit, key as apiKey } from config/default;
    "#;

    let mut lexer = Lexer::new(input);
    lexer.tokenize();
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    assert_eq!(program.len(), 1);
    match &program[0] {
        Statement::ImportDeclaration {
            imports,
            path,
            ..
        } => {
            assert_eq!(imports.len(), 2);
            assert_eq!(path, "config/default");
            // Add more assertions for the import specifiers
        }
        _ => panic!("Expected ImportDeclaration"),
    }
}

#[test]
fn test_parse_export_declaration() {
    let input = r#"
        export { limit, key };
    "#;

    let mut lexer = Lexer::new(input);
    lexer.tokenize();
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    assert_eq!(program.len(), 1);
    match &program[0] {
        Statement::ExportDeclaration {
            specifiers,
            ..
        } => {
            assert_eq!(specifiers.len(), 2);
            // Add more assertions for the export specifiers
        }
        _ => panic!("Expected ExportDeclaration"),
    }
}

#[test]
fn test_invalid_syntax_error_handling() {
    let input = r#"
        int x = 5
        // Missing semicolon
    "#;

    let mut lexer = Lexer::new(input);
    lexer.tokenize();
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    assert_eq!(program.len(), 1);
    assert!(!parser.get_errors().is_empty());
}