use crate::token::{TokenType, Token};

pub enum LiteralValue {
    Int(i32),
    Float(f32),
    String(String),
    Bool(bool),
}

pub enum Statement {
    VariableDeclaration {
        token: Token,
        name: String,
        type_name: Option<String>,
        value: Option<Expression>,
    },
    FunctionDeclaration {
        token: Token,
        name: String,
        parameters: Vec<(String, String)>, // (name, type)
        body: Vec<Statement>,
        return_type: Option<String>,
    },
    ReturnStatement {
        token: Token,
        value: Option<Expression>,
    },
    Expression(Expression),
    IfStatement {
        token: Token,
        condition: Expression,
        then_branch: Box<Statement>,
        else_branch: Option<Box<Statement>>,
    },
    DoWhileStatement {
        token: Token,
        body: Box<Statement>,
        condition: Expression,
    },
    WhileStatement {
        token: Token,
        condition: Expression,
        body: Box<Statement>,
    },
    ForStatement {
        token: Token,
        initializer: Option<Box<Statement>>,
        condition: Option<Expression>,
        increment: Option<Expression>,
        body: Box<Statement>,
    },
    ForEachStatement {
        token: Token,
        element_variable: String,
        iterator: Expression,
        body: Box<Statement>,
    },
    BreakStatement {
        token: Token,
    },
    ContinueStatement {
        token: Token,
    },
    EnumDeclaration {
        token: Token,
        name: String,
        variants: Vec<String>,
    },
    ObjectDeclaration {
        token: Token,
        name: String,
        properties: Vec<(String, Expression)>,
    },
    ClassDeclaration {
        token: Token,
        name: String,
        superclass: Option<String>,
        interfaces: Vec<String>,
        members: Vec<ClassMember>,
    },
    InterfaceDeclaration {
        token: Token,
        name: String,
        members: Vec<InterfaceMember>,
    },
    ImportDeclaration {
        token: Token,
        path: String,
        imports: Vec<ImportSpecifier>,
    },
    ExportDeclaration {
        token: Token,
        specifiers: Vec<ExportSpecifier>,
    },
    SwitchStatement {
        token: Token,
        expression: Expression,
        cases: Vec<(Expression, Vec<Statement>)>,
        default: Option<Vec<Statement>>,
    },
    BlockStatement(Vec<Statement>),
}

pub enum ClassMember {
    Field {
        token: Token,
        name: String,
        type_name: Option<String>,
        value: Option<Expression>,
        visibility: Visibility,
        is_static: bool,
    },
    Method {
        token: Token,
        name: String,
        parameters: Vec<(String, String)>, // (name, type)
        body: Vec<Statement>,
        return_type: Option<String>,
        visibility: Visibility,
        is_static: bool,
    },
}

pub enum InterfaceMember {
    Method {
        token: Token,
        name: String,
        parameters: Vec<(String, String)>, // (name, type)
        return_type: Option<String>,
    },
}

pub enum ImportSpecifier {
    Named(String),
    Default(String),
}

pub enum ExportSpecifier {
    Named(String),
    Default,
}

pub enum Visibility {
    Public,
    Private,
}

pub enum Expression {
    Literal {
        token: Token,
        value: LiteralValue,
    },
    Identifier {
        token: Token,
        name: String,
    },
    BinaryOperation {
        token: Token,
        left: Box<Expression>,
        operator: TokenType,
        right: Box<Expression>,
    },
    UnaryOperation {
        token: Token,
        operator: TokenType,
        operand: Box<Expression>,
    },
    Assignment {
        token: Token,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    FunctionCall {
        token: Token,
        callee: Box<Expression>,
        arguments: Vec<Expression>,
    },
    ArrayLiteral {
        token: Token,
        elements: Vec<Expression>,
    },
    IndexAccess {
        token: Token,
        array: Box<Expression>,
        index: Box<Expression>,
    },
    MemberAccess {
        token: Token,
        object: Box<Expression>,
        member: String,
    },
    Ternary {
        token: Token,
        condition: Box<Expression>,
        then_expression: Box<Expression>,
        else_expression: Box<Expression>,
    },
    DictLiteral {
        token: Token,
        pairs: Vec<(Expression, Expression)>, // Key-value pairs
    },
    NewExpression {
        token: Token,
        class_name: String,
        arguments: Vec<Expression>,
    },
    This {
        token: Token,
    },
}