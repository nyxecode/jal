use crate::token::{TokenType, Token};
use crate::ast::{
    LiteralValue, Statement, Expression, Visibility, ClassMember, InterfaceMember, ImportSpecifier,
    ExportSpecifier,
};
use crate::lexer::Lexer;

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
    errors: Vec<String>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Parser {
            lexer,
            current_token: Token::new(TokenType::EOF, 0, 0),
            peek_token: Token::new(TokenType::EOF, 0, 0),
            errors: Vec::new(),
        };
        parser.next_token();
        parser.next_token(); // Initialize current_token and peek_token
        parser
    }

    pub fn get_errors(&self) -> &Vec<String> {
        &self.errors
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.tokens.remove(0);
    }

    fn expect_peek(&mut self, token_type: TokenType) -> bool {
        if self.peek_token_is(token_type) {
            self.next_token();
            true
        } else {
            // We need to clone the token_type here to avoid moving it
            self.peek_error(token_type.clone());
            false
        }
    }

    fn peek_error(&mut self, token_type: TokenType) {
        let msg = format!(
            "Expected token: {:?}, got: {:?} instead",
            token_type,
            self.peek_token.token_type
        );
        self.errors.push(msg);
    }

    fn peek_token_is(&self, token_type: TokenType) -> bool {
        self.peek_token.token_type == token_type
    }

    fn current_token_is(&self, token_type: TokenType) -> bool {
        self.current_token.token_type == token_type
    }

    pub fn parse_program(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();

        while !self.current_token_is(TokenType::EOF) {
            match self.parse_statement() {
                Some(stmt) => statements.push(stmt),
                None => {} // Error handling already done in parse_statement
            }
            self.next_token();
        }

        statements
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token.token_type {
            TokenType::IntKeyword
            | TokenType::FloatKeyword
            | TokenType::StringKeyword
            | TokenType::BoolKeyword => self.parse_variable_declaration(),
            TokenType::ConstKeyword => self.parse_const_variable_declaration(),
            TokenType::FunctionKeyword => self.parse_function_declaration(),
            TokenType::ReturnKeyword => self.parse_return_statement(),
            TokenType::IfKeyword => self.parse_if_statement(),
            TokenType::DoKeyword => self.parse_do_while_statement(),
            TokenType::WhileKeyword => self.parse_while_statement(),
            TokenType::ForKeyword => {
                if self.peek_token_is(TokenType::Identifier(String::new()))
                    && self.lexer.tokens.get(1).map_or(false, |t| t.token_type == TokenType::OfKeyword)
                {
                    self.parse_for_of_statement() // Call the new parsing function
                } else {
                    self.parse_for_statement() // Parse the standard for loop
                }
            }
            TokenType::BreakKeyword => self.parse_break_statement(),
            TokenType::ContinueKeyword => self.parse_continue_statement(),
            TokenType::EnumKeyword => self.parse_enum_declaration(),
            TokenType::ObjectKeyword => self.parse_object_declaration(),
            TokenType::ClassKeyword => self.parse_class_declaration(),
            TokenType::InterfaceKeyword => self.parse_interface_declaration(),
            TokenType::ImportKeyword => self.parse_import_declaration(),
            TokenType::ExportKeyword => self.parse_export_declaration(),
            TokenType::SwitchKeyword => self.parse_switch_statement(),
            _ => {
                // For expressions, attempt to parse them.
                // If there's an error, skip to the next semicolon and report it.
                let expr = self.parse_expression(None);
                match expr {
                    Some(_) => {
                        if !self.expect_peek(TokenType::Semicolon) {
                            // Error handling: Expected semicolon after expression
                            self.skip_to_next_statement();
                        }
                        expr.map(Statement::Expression)
                    }
                    None => {
                        self.skip_to_next_statement();
                        None
                    }
                }
            }
        }
    }

    fn skip_to_next_statement(&mut self) {
        while !self.current_token_is(TokenType::Semicolon)
            && !self.current_token_is(TokenType::RightBrace)
            && !self.current_token_is(TokenType::EOF)
        {
            self.next_token();
        }
    }

    fn parse_variable_declaration(&mut self) -> Option<Statement> {
        let token = self.current_token.clone();
        let type_name = match self.current_token.token_type {
            TokenType::IntKeyword => Some("int".to_string()),
            TokenType::FloatKeyword => Some("float".to_string()),
            TokenType::StringKeyword => Some("string".to_string()),
            TokenType::BoolKeyword => Some("bool".to_string()),
            _ => None,
        };

        if !self.expect_peek(TokenType::Identifier(String::new())) {
            return None;
        }
        let name = match self.current_token.token_type {
            TokenType::Identifier(ref identifier) => identifier.clone(),
            _ => unreachable!(),
        };

        let mut value = None;
        if self.peek_token_is(TokenType::Equals) {
            self.next_token(); // consume '='
            self.next_token(); // consume value
            value = self.parse_expression(None);
        }

        if !self.expect_peek(TokenType::Semicolon) {
            return None;
        }

        Some(Statement::VariableDeclaration {
            token,
            name,
            type_name,
            value,
        })
    }

    fn parse_const_variable_declaration(&mut self) -> Option<Statement> {
        let token = self.current_token.clone(); // "const" token

        // Expect a type keyword after "const"
        let type_name = match self.peek_token.token_type {
            TokenType::IntKeyword => {
                self.next_token(); // Consume the type keyword
                Some("int".to_string())
            }
            TokenType::FloatKeyword => {
                self.next_token();
                Some("float".to_string())
            }
            TokenType::StringKeyword => {
                self.next_token();
                Some("string".to_string())
            }
            TokenType::BoolKeyword => {
                self.next_token();
                Some("bool".to_string())
            }
            _ => {
                self.peek_error(TokenType::IntKeyword); // Or any other valid type keyword
                return None;
            }
        };

        if !self.expect_peek(TokenType::Identifier(String::new())) {
            return None;
        }
        let name = match self.current_token.token_type {
            TokenType::Identifier(ref identifier) => identifier.clone(),
            _ => unreachable!(),
        };

        if !self.expect_peek(TokenType::Equals) {
            return None;
        }

        self.next_token(); // consume '='
        let value = self.parse_expression(None);

        if !self.expect_peek(TokenType::Semicolon) {
            return None;
        }

        Some(Statement::VariableDeclaration {
            token,
            name,
            type_name,
            value,
        })
    }

    fn parse_function_declaration(&mut self) -> Option<Statement> {
        let token = self.current_token.clone();

        if !self.expect_peek(TokenType::Identifier(String::new())) {
            return None;
        }

        let name = match self.current_token.token_type.clone() {
            TokenType::Identifier(identifier) => identifier,
            _ => unreachable!(),
        };

        if !self.expect_peek(TokenType::LeftParen) {
            return None;
        }

        let parameters = self.parse_function_parameters();

        if !self.expect_peek(TokenType::RightParen) {
            return None;
        }

        let return_type = if self.peek_token_is(TokenType::EqualsGreaterThan) {
            self.next_token(); // consume '=>'
            if !self.expect_peek(TokenType::Identifier(String::new())) {
                return None;
            }
            match self.current_token.token_type.clone() {
                TokenType::Identifier(identifier) => Some(identifier),
                _ => unreachable!(),
            }
        } else {
            None
        };

        if !self.expect_peek(TokenType::LeftBrace) {
            return None;
        }

        let body = self.parse_block_statement();

        if !self.expect_peek(TokenType::RightBrace) {
            return None;
        }

        Some(Statement::FunctionDeclaration {
            token,
            name,
            parameters,
            body,
            return_type,
        })
    }

    fn parse_function_parameters(&mut self) -> Vec<(String, String)> {
        let mut parameters: Vec<(String, String)> = Vec::new();

        if self.peek_token_is(TokenType::RightParen) {
            return parameters; // Empty parameter list
        }

        loop {
            if !self.expect_peek(TokenType::Identifier(String::new())) {
                return parameters; // Error recovery: return whatever parameters we have
            }
            let name = match self.current_token.token_type.clone() {
                TokenType::Identifier(identifier) => identifier,
                _ => unreachable!(),
            };

            if !self.expect_peek(TokenType::Colon) {
                return parameters;
            }

            if !self.expect_peek(TokenType::Identifier(String::new())) {
                return parameters;
            }
            let type_name = match self.current_token.token_type.clone() {
                TokenType::Identifier(identifier) => identifier,
                _ => unreachable!(),
            };

            parameters.push((name, type_name));

            if !self.peek_token_is(TokenType::Comma) {
                break;
            }
            self.next_token(); // consume ','
        }

        parameters
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        let token = self.current_token.clone();
        let mut value = None;

        if !self.peek_token_is(TokenType::Semicolon) {
            self.next_token(); // consume the return value
            value = self.parse_expression(None);
        }

        if !self.expect_peek(TokenType::Semicolon) {
            return None;
        }

        Some(Statement::ReturnStatement { token, value })
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let expression = self.parse_expression(None);

        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        expression.map(Statement::Expression)
    }

    fn parse_expression(&mut self, precedence: Option<i32>) -> Option<Expression> {
        let mut left_expr = match self.current_token.token_type {
            TokenType::Identifier(_) => self.parse_identifier_expression(),
            TokenType::Int(_)
            | TokenType::Float(_)
            | TokenType::String(_)
            | TokenType::TrueKeyword
            | TokenType::FalseKeyword => self.parse_literal_expression(),
            TokenType::Minus | TokenType::LogicalNot => self.parse_prefix_expression(),
            TokenType::LeftParen => {
                self.next_token(); // consume '('
                let expr = self.parse_expression(None);
                if !self.expect_peek(TokenType::RightParen) {
                    return None;
                }
                expr
            }
            TokenType::LeftBrace => {
                // Check if it's a dict literal or a block statement
                if self.peek_token_is(TokenType::Identifier(String::new()))
                    || self.peek_token_is(TokenType::String(String::new()))
                {
                    self.parse_dict_literal() // Call the new parsing function
                } else {
                    // It's a block statement, parse it as before
                    self.parse_block_expression()
                }
            }
            TokenType::NewKeyword => self.parse_new_expression(),
            TokenType::ThisKeyword => {
                let token = self.current_token.clone();
                self.next_token();
                Some(Expression::This { token })
            }
            _ => {
                self.errors
                    .push(format!("Unexpected token: {:?}", self.current_token));
                return None;
            }
        };

        if left_expr.is_none() {
            return None;
        }

        while !self.peek_token_is(TokenType::Semicolon) && precedence < Some(self.peek_precedence()) {
            match self.peek_token.token_type {
                TokenType::Plus
                | TokenType::Minus
                | TokenType::Star
                | TokenType::Slash
                | TokenType::Percent
                | TokenType::EqualsEquals
                | TokenType::NotEquals
                | TokenType::GreaterThan
                | TokenType::LessThan
                | TokenType::GreaterThanEquals
                | TokenType::LessThanEquals
                | TokenType::LogicalAnd
                | TokenType::LogicalOr => {
                    self.next_token();
                    left_expr = self.parse_infix_expression(left_expr.unwrap());
                }
                TokenType::LeftParen => {
                    self.next_token();
                    left_expr = self.parse_call_expression(left_expr.unwrap());
                }
                TokenType::LeftBracket => {
                    self.next_token();
                    left_expr = self.parse_index_expression(left_expr.unwrap());
                }
                TokenType::Dot => {
                    self.next_token();
                    left_expr = self.parse_member_access(left_expr.unwrap());
                }
                TokenType::Equals
                | TokenType::PlusEquals
                | TokenType::MinusEquals
                | TokenType::StarEquals
                | TokenType::SlashEquals
                | TokenType::PercentEquals => {
                    self.next_token();
                    left_expr = self.parse_assignment_expression(left_expr.unwrap());
                }
                _ => return left_expr,
            }

            if left_expr.is_none() {
                return None;
            }
        }

        left_expr
    }

    fn parse_new_expression(&mut self) -> Option<Expression> {
        let token = self.current_token.clone();

        if !self.expect_peek(TokenType::Identifier(String::new())) {
            return None;
        }

        let class_name = match self.current_token.token_type.clone() {
            TokenType::Identifier(identifier) => identifier,
            _ => unreachable!(),
        };

        if !self.expect_peek(TokenType::LeftParen) {
            return None;
        }

        let arguments = self.parse_expression_list(TokenType::RightParen)?;

        Some(Expression::NewExpression {
            token,
            class_name,
            arguments,
        })
    }

    fn parse_expression_list(&mut self, terminator: TokenType) -> Option<Vec<Expression>> {
        let mut expressions = Vec::new();

        // Check if the list is empty
        if self.peek_token_is(terminator.clone()) {
            self.next_token(); // consume the terminator
            return Some(expressions);
        }

        loop {
            // Parse the next expression, consuming the current token
            let expr = self.parse_expression(None);
            if let Some(expr) = expr {
                expressions.push(expr);
            } else {
                return None; // Error parsing expression
            }

            // If the next token is not a comma, it should be the terminator
            if !self.peek_token_is(TokenType::Comma) {
                if self.expect_peek(terminator.clone()) { // Check for the terminator
                    return Some(expressions);
                } else {
                    return None; // Error: missing terminator
                }
            }

            self.next_token(); // consume ','
        }
    }

    fn parse_identifier_expression(&mut self) -> Option<Expression> {
        let token = self.current_token.clone();
        let name = match &token.token_type {
            TokenType::Identifier(name) => name.clone(),
            _ => unreachable!(),
        };

        Some(Expression::Identifier { token, name })
    }

    fn parse_literal_expression(&mut self) -> Option<Expression> {
        let token = self.current_token.clone();
        let value = match token.token_type {
            TokenType::Int(value) => LiteralValue::Int(value),
            TokenType::Float(value) => LiteralValue::Float(value),
            TokenType::String(ref value) => LiteralValue::String(value.clone()),
            TokenType::TrueKeyword => LiteralValue::Bool(true),
            TokenType::FalseKeyword => LiteralValue::Bool(false),
            _ => {
                self.errors
                    .push(format!("Unexpected token: {:?}", self.current_token));
                return None;
            }
        };

        Some(Expression::Literal { token, value })
    }

    fn parse_prefix_expression(&mut self) -> Option<Expression> {
        let token = self.current_token.clone();
        let operator = match token.token_type {
            TokenType::Minus => TokenType::Minus,
            TokenType::LogicalNot => TokenType::LogicalNot,
            _ => {
                self.errors
                    .push(format!("Unexpected token: {:?}", self.current_token));
                return None;
            }
        };

        self.next_token();
        let operand = self.parse_expression(Some(self.prefix_precedence()));

        operand.map(|right| Expression::UnaryOperation {
            token,
            operator,
            operand: Box::new(right),
        })
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Option<Expression> {
        let token = self.current_token.clone();
        let operator = match token.token_type {
            TokenType::Plus
            | TokenType::Minus
            | TokenType::Star
            | TokenType::Slash
            | TokenType::Percent
            | TokenType::EqualsEquals
            | TokenType::NotEquals
            | TokenType::GreaterThan
            | TokenType::LessThan
            | TokenType::GreaterThanEquals
            | TokenType::LessThanEquals
            | TokenType::LogicalAnd
            | TokenType::LogicalOr => token.token_type,
            _ => {
                self.errors
                    .push(format!("Unexpected token: {:?}", self.current_token));
                return None;
            }
        };

        let precedence = self.infix_precedence();
        self.next_token();
        let right = self.parse_expression(Some(precedence));

        right.map(|right| Expression::BinaryOperation {
            token,
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }

    fn parse_assignment_expression(&mut self, left: Expression) -> Option<Expression> {
        let token = self.current_token.clone();
        let operator = match token.token_type {
            TokenType::Equals => TokenType::Equals,
            TokenType::PlusEquals => TokenType::PlusEquals,
            TokenType::MinusEquals => TokenType::MinusEquals,
            TokenType::StarEquals => TokenType::StarEquals,
            TokenType::SlashEquals => TokenType::SlashEquals,
            TokenType::PercentEquals => TokenType::PercentEquals,
            _ => {
                self.errors
                    .push(format!("Unexpected token: {:?}", self.current_token));
                return None;
            }
        };

        self.next_token();
        let right = self.parse_expression(Some(self.assignment_precedence()));

        right.map(|right| Expression::Assignment {
            token,
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    fn parse_call_expression(&mut self, function: Expression) -> Option<Expression> {
        let token = self.current_token.clone();
        let mut arguments = Vec::new();

        if self.peek_token_is(TokenType::RightParen) {
            self.next_token(); // consume ')'
            return Some(Expression::FunctionCall {
                token,
                callee: Box::new(function),
                arguments,
            });
        }

        self.next_token(); // consume first argument
        arguments.push(self.parse_expression(None).unwrap());

        while self.peek_token_is(TokenType::Comma) {
            self.next_token(); // consume ','
            self.next_token(); // consume next argument
            arguments.push(self.parse_expression(None).unwrap());
        }

        if !self.expect_peek(TokenType::RightParen) {
            return None;
        }

        Some(Expression::FunctionCall {
            token,
            callee: Box::new(function),
            arguments,
        })
    }

    fn parse_dict_literal(&mut self) -> Option<Expression> {
        let token = self.current_token.clone(); // '{' token
        let mut pairs = Vec::new();

        self.next_token(); // consume '{'

        while !self.current_token_is(TokenType::RightBrace) {
            let key = match self.parse_expression(None) {
                Some(expr) => expr,
                None => {
                    // Error handling: Expected expression for key
                    return None;
                }
            };

            if !self.expect_peek(TokenType::Colon) {
                // Error handling: Expected ':' after key
                return None;
            }

            self.next_token(); // consume ':'

            let value = match self.parse_expression(None) {
                Some(expr) => expr,
                None => {
                    // Error handling: Expected expression for value
                    return None;
                }
            };

            pairs.push((key, value));

            if self.peek_token_is(TokenType::Comma) {
                self.next_token(); // consume ','
            } else {
                break; // No comma, end of dict literal
            }
        }

        if !self.expect_peek(TokenType::RightBrace) {
            // Error handling: Expected '}' to close the dict literal
            return None;
        }

        Some(Expression::DictLiteral { token, pairs })
    }

    fn parse_index_expression(&mut self, array: Expression) -> Option<Expression> {
        let token = self.current_token.clone();
        self.next_token(); // consume '['
        let index = self.parse_expression(None);
        if !self.expect_peek(TokenType::RightBracket) {
            return None;
        }

        index.map(|index| Expression::IndexAccess {
            token,
            array: Box::new(array),
            index: Box::new(index),
        })
    }

    fn parse_member_access(&mut self, object: Expression) -> Option<Expression> {
        let token = self.current_token.clone();
        if !self.expect_peek(TokenType::Identifier(String::new())) {
            return None;
        }

        let member = match self.current_token.token_type.clone() {
            TokenType::Identifier(identifier) => identifier,
            _ => unreachable!(),
        };

        Some(Expression::MemberAccess {
            token,
            object: Box::new(object),
            member,
        })
    }

    fn parse_if_statement(&mut self) -> Option<Statement> {
        let token = self.current_token.clone();
        if !self.expect_peek(TokenType::LeftParen) {
            return None;
        }

        self.next_token(); // consume '('
        let condition = self.parse_expression(None);
        if !self.expect_peek(TokenType::RightParen) {
            return None;
        }

        if !self.expect_peek(TokenType::LeftBrace) {
            return None;
        }

        let then_branch = self.parse_block_statement();

        let mut else_branch = None;
        if self.peek_token_is(TokenType::ElseKeyword) {
            self.next_token(); // consume 'else'

            if self.peek_token_is(TokenType::IfKeyword) {
                self.next_token(); // consume 'if'
                else_branch = Some(Box::new(self.parse_if_statement().unwrap()));
            } else if self.expect_peek(TokenType::LeftBrace) {
                else_branch = Some(Box::new(self.parse_block_statement()));
            } else {
                return None;
            }
        }

        condition.map(|condition| Statement::IfStatement {
            token,
            condition,
            then_branch: Box::new(Statement::BlockStatement(then_branch)),
            else_branch,
        })
    }

    fn parse_do_while_statement(&mut self) -> Option<Statement> {
        let token = self.current_token.clone();

        if !self.expect_peek(TokenType::LeftBrace) {
            return None;
        }

        let body = self.parse_block_statement();

        if !self.expect_peek(TokenType::RightBrace) {
            return None;
        }

        if !self.expect_peek(TokenType::WhileKeyword) {
            return None;
        }

        if !self.expect_peek(TokenType::LeftParen) {
            return None;
        }

        self.next_token(); // consume '('
        let condition = self.parse_expression(None);
        if !self.expect_peek(TokenType::RightParen) {
            return None;
        }

        if !self.expect_peek(TokenType::Semicolon) {
            return None;
        }

        condition.map(|condition| Statement::DoWhileStatement {
            token,
            body: Box::new(Statement::BlockStatement(body)),
            condition,
        })
    }

    fn parse_while_statement(&mut self) -> Option<Statement> {
        let token = self.current_token.clone();
        if !self.expect_peek(TokenType::LeftParen) {
            return None;
        }

        self.next_token(); // consume '('
        let condition = self.parse_expression(None);
        if !self.expect_peek(TokenType::RightParen) {
            return None;
        }

        if !self.expect_peek(TokenType::LeftBrace) {
            return None;
        }

        let body = self.parse_block_statement();

        condition.map(|condition| Statement::WhileStatement {
            token,
            condition,
            body: Box::new(Statement::BlockStatement(body)),
        })
    }

    fn parse_for_statement(&mut self) -> Option<Statement> {
        let token = self.current_token.clone();
        if !self.expect_peek(TokenType::LeftParen) {
            return None;
        }

        self.next_token(); // consume '('

        let initializer = if !self.peek_token_is(TokenType::Semicolon) {
            Some(Box::new(self.parse_statement().unwrap()))
        } else {
            None
        };

        self.next_token(); // consume ';'

        let condition = if !self.peek_token_is(TokenType::Semicolon) {
            Some(self.parse_expression(None).unwrap())
        } else {
            None
        };

        self.next_token(); // consume ';'

        let increment = if !self.peek_token_is(TokenType::RightParen) {
            Some(self.parse_expression(None).unwrap())
        } else {
            None
        };

        if !self.expect_peek(TokenType::RightParen) {
            return None;
        }

        if !self.expect_peek(TokenType::LeftBrace) {
            return None;
        }

        let body = self.parse_block_statement();

        Some(Statement::ForStatement {
            token,
            initializer,
            condition,
            increment,
            body: Box::new(Statement::BlockStatement(body)),
        })
    }

    fn parse_for_of_statement(&mut self) -> Option<Statement> {
        let token = self.current_token.clone();

        // Expect an identifier (element variable) after "for"
        if !self.expect_peek(TokenType::Identifier(String::new())) {
            return None;
        }
        let element_variable = match self.current_token.token_type.clone() {
            TokenType::Identifier(identifier) => identifier,
            _ => unreachable!(),
        };

        // Expect "of" keyword
        if !self.expect_peek(TokenType::OfKeyword) {
            return None;
        }

        // Parse the iterable expression
        self.next_token(); // consume 'of'
        let iterator = self.parse_expression(None);
        if iterator.is_none() {
            return None;
        }

        // Expect a block statement for the loop body
        if !self.expect_peek(TokenType::LeftBrace) {
            return None;
        }
        let body = self.parse_block_statement();

        Some(Statement::ForEachStatement {
            token,
            element_variable,
            iterator: iterator.unwrap(),
            body: Box::new(Statement::BlockStatement(body)),
        })
    }

    fn parse_break_statement(&mut self) -> Option<Statement> {
        let token = self.current_token.clone();
        Some(Statement::BreakStatement { token })
    }

    fn parse_continue_statement(&mut self) -> Option<Statement> {
        let token = self.current_token.clone();
        Some(Statement::ContinueStatement { token })
    }

    fn parse_switch_statement(&mut self) -> Option<Statement> {
        let token = self.current_token.clone();

        if !self.expect_peek(TokenType::LeftParen) {
            return None;
        }

        self.next_token(); // consume '('
        let expression = self.parse_expression(None).unwrap();
        if !self.expect_peek(TokenType::RightParen) {
            return None;
        }

        if !self.expect_peek(TokenType::LeftBrace) {
            return None;
        }

        let mut cases = Vec::new();
        let mut default = None;

        while !self.peek_token_is(TokenType::RightBrace) {
            match self.peek_token.token_type {
                TokenType::CaseKeyword => {
                    self.next_token(); // consume 'case'
                    self.next_token(); // consume expression
                    let case_expr = self.parse_expression(None).unwrap();

                    if !self.expect_peek(TokenType::Colon) {
                        return None;
                    }

                    let mut statements = Vec::new();
                    while !self.peek_token_is(TokenType::CaseKeyword)
                        && !self.peek_token_is(TokenType::DefaultKeyword)
                        && !self.peek_token_is(TokenType::RightBrace)
                    {
                        if let Some(stmt) = self.parse_statement() {
                            statements.push(stmt);
                        }
                        self.next_token();
                    }
                    cases.push((case_expr, statements));
                }
                TokenType::DefaultKeyword => {
                    self.next_token(); // consume 'default'
                    if !self.expect_peek(TokenType::Colon) {
                        return None;
                    }

                    let mut statements = Vec::new();
                    while !self.peek_token_is(TokenType::CaseKeyword)
                        && !self.peek_token_is(TokenType::DefaultKeyword)
                        && !self.peek_token_is(TokenType::RightBrace)
                    {
                        if let Some(stmt) = self.parse_statement() {
                            statements.push(stmt);
                        }
                        self.next_token();
                    }
                    default = Some(statements);
                }
                _ => {
                    self.errors.push(format!(
                        "Unexpected token in switch statement: {:?}",
                        self.peek_token.token_type
                    ));
                    return None;
                }
            }
        }

        if !self.expect_peek(TokenType::RightBrace) {
            return None;
        }

        Some(Statement::SwitchStatement {
            token,
            expression,
            cases,
            default,
        })
    }

    fn parse_enum_declaration(&mut self) -> Option<Statement> {
        let token = self.current_token.clone();
        if !self.expect_peek(TokenType::Identifier(String::new())) {
            return None;
        }

        let name = match self.current_token.token_type.clone() {
            TokenType::Identifier(identifier) => identifier,
            _ => unreachable!(),
        };

        if !self.expect_peek(TokenType::LeftBrace) {
            return None;
        }

        let mut variants = Vec::new();
        while !self.peek_token_is(TokenType::RightBrace) {
            if !self.expect_peek(TokenType::Identifier(String::new())) {
                return None;
            }

            variants.push(
                match self.current_token.token_type.clone() {
                    TokenType::Identifier(identifier) => identifier,
                    _ => unreachable!(),
                },
            );

            if !self.peek_token_is(TokenType::Comma) {
                break;
            }
            self.next_token(); // consume ','
        }

        variants
    }

    fn parse_object_declaration(&mut self) -> Option<Statement> {
        let token = self.current_token.clone();
        let name = if self.peek_token_is(TokenType::Identifier(String::new())) {
            self.next_token();
            match self.current_token.token_type.clone() {
                TokenType::Identifier(identifier) => Some(identifier),
                _ => unreachable!(),
            }
        } else {
            None
        };

        if !self.expect_peek(TokenType::Equals) {
            return None;
        }
        self.next_token(); // consume '='

        if !self.expect_peek(TokenType::LeftBrace) {
            return None;
        }

        let mut properties = Vec::new();
        while !self.peek_token_is(TokenType::RightBrace) {
            if !self.expect_peek(TokenType::Identifier(String::new())) {
                return None;
            }
            let key = match self.current_token.token_type.clone() {
                TokenType::Identifier(identifier) => identifier,
                _ => unreachable!(),
            };

            if !self.expect_peek(TokenType::Colon) {
                return None;
            }

            self.next_token(); // consume ':'
            let value = self.parse_expression(None);
            if value.is_none() {
                return None;
            }

            properties.push((key, value.unwrap()));

            if !self.peek_token_is(TokenType::Comma) {
                break;
            }
            self.next_token(); // consume ','
        }

        if !self.expect_peek(TokenType::RightBrace) {
            return None;
        }

        if !self.expect_peek(TokenType::Semicolon) {
            return None;
        }

        Some(Statement::ObjectDeclaration {
            token,
            name: name.unwrap_or_else(|| "".to_string()),
            properties,
        })
    }

    fn parse_class_declaration(&mut self) -> Option<Statement> {
        let token = self.current_token.clone();
        if !self.expect_peek(TokenType::Identifier(String::new())) {
            return None;
        }

        let name = match self.current_token.token_type.clone() {
            TokenType::Identifier(identifier) => identifier,
            _ => unreachable!(),
        };

        let superclass = if self.peek_token_is(TokenType::ExtendsKeyword) {
            self.next_token(); // consume 'extends'
            if !self.expect_peek(TokenType::Identifier(String::new())) {
                return None;
            }
            Some(match self.current_token.token_type.clone() {
                TokenType::Identifier(identifier) => identifier,
                _ => unreachable!(),
            })
        } else {
            None
        };

        let mut interfaces = Vec::new();
        if self.peek_token_is(TokenType::ImplementsKeyword) {
            self.next_token(); // consume 'implements'

            loop {
                if !self.expect_peek(TokenType::Identifier(String::new())) {
                    return None;
                }
                interfaces.push(match self.current_token.token_type.clone() {
                    TokenType::Identifier(identifier) => identifier,
                    _ => unreachable!(),
                });

                if !self.peek_token_is(TokenType::Comma) {
                    break;
                }
                self.next_token(); // consume ','
            }
        }

        if !self.expect_peek(TokenType::LeftBrace) {
            return None;
        }

        let members = self.parse_class_members();

        if !self.expect_peek(TokenType::RightBrace) {
            return None;
        }

        Some(Statement::ClassDeclaration {
            token,
            name,
            superclass,
            interfaces,
            members,
        })
    }

    fn parse_class_members(&mut self) -> Vec<ClassMember> {
        let mut members = Vec::new();

        while !self.peek_token_is(TokenType::RightBrace) {
            let member = self.parse_class_member();
            if let Some(member) = member {
                members.push(member);
            } else {
                // Skip to the next semicolon or closing brace to recover
                while !self.current_token_is(TokenType::Semicolon)
                    && !self.current_token_is(TokenType::RightBrace)
                {
                    self.next_token();
                }
            }
        }

        members
    }

    fn parse_class_member(&mut self) -> Option<ClassMember> {
        let visibility = self.parse_visibility();
        let is_static = if self.peek_token_is(TokenType::StaticKeyword) {
            self.next_token(); // consume 'static'
            true
        } else {
            false
        };

        if self.peek_token_is(TokenType::FunctionKeyword) {
            self.parse_method_declaration(visibility, is_static)
        } else {
            self.parse_field_declaration(visibility, is_static)
        }
    }

    fn parse_visibility(&mut self) -> Visibility {
        if self.peek_token_is(TokenType::PublicKeyword) {
            self.next_token(); // consume 'public'
            Visibility::Public
        } else if self.peek_token_is(TokenType::PrivateKeyword) {
            self.next_token(); // consume 'private'
            Visibility::Private
        } else {
            Visibility::Public // Default visibility is public
        }
    }

    fn parse_field_declaration(
        &mut self,
        visibility: Visibility,
        is_static: bool,
    ) -> Option<ClassMember> {
        let token = self.current_token.clone();

        let type_name = if let TokenType::Identifier(_) = self.peek_token.token_type {
            self.next_token(); // consume type
            Some(match self.current_token.token_type.clone() {
                TokenType::Identifier(identifier) => identifier,
                _ => unreachable!(),
            })
        } else {
            None
        };

        if !self.expect_peek(TokenType::Identifier(String::new())) {
            return None;
        }

        let name = match self.current_token.token_type.clone() {
            TokenType::Identifier(identifier) => identifier,
            _ => unreachable!(),
        };

        let mut value = None;
        if self.peek_token_is(TokenType::Equals) {
            self.next_token(); // consume '='
            value = self.parse_expression(None);
        }

        if !self.expect_peek(TokenType::Semicolon) {
            return None;
        }

        Some(ClassMember::Field {
            token,
            name,
            type_name,
            value,
            visibility,
            is_static,
        })
    }

    fn parse_method_declaration(
        &mut self,
        visibility: Visibility,
        is_static: bool,
    ) -> Option<ClassMember> {
        let token = self.current_token.clone();
        self.next_token(); // consume 'function'

        if !self.expect_peek(TokenType::Identifier(String::new())) {
            return None;
        }

        let name = match self.current_token.token_type.clone() {
            TokenType::Identifier(identifier) => identifier,
            _ => unreachable!(),
        };

        if !self.expect_peek(TokenType::LeftParen) {
            return None;
        }

        let parameters = self.parse_function_parameters();

        if !self.expect_peek(TokenType::RightParen) {
            return None;
        }

        let return_type = if self.peek_token_is(TokenType::EqualsGreaterThan) {
            self.next_token(); // consume '=>'
            if !self.expect_peek(TokenType::Identifier(String::new())) {
                return None;
            }
            Some(match self.current_token.token_type.clone() {
                TokenType::Identifier(identifier) => identifier,
                _ => unreachable!(),
            })
        } else {
            None
        };

        if !self.expect_peek(TokenType::LeftBrace) {
            return None;
        }

        let body = self.parse_block_statement();

        if !self.expect_peek(TokenType::RightBrace) {
            return None;
        }

        Some(ClassMember::Method {
            token,
            name,
            parameters,
            body,
            return_type,
            visibility,
            is_static,
        })
    }

    fn parse_interface_declaration(&mut self) -> Option<Statement> {
        let token = self.current_token.clone();
        if !self.expect_peek(TokenType::Identifier(String::new())) {
            return None;
        }

        let name = match self.current_token.token_type.clone() {
            TokenType::Identifier(identifier) => identifier,
            _ => unreachable!(),
        };

        if !self.expect_peek(TokenType::LeftBrace) {
            return None;
        }

        let members = self.parse_interface_members();

        if !self.expect_peek(TokenType::RightBrace) {
            return None;
        }

        Some(Statement::InterfaceDeclaration {
            token,
            name,
            members,
        })
    }

    fn parse_interface_members(&mut self) -> Vec<InterfaceMember> {
        let mut members = Vec::new();

        while !self.peek_token_is(TokenType::RightBrace) {
            let member = self.parse_interface_member();
            if let Some(member) = member {
                members.push(member);
            } else {
                // Skip to the next semicolon or closing brace to recover
                while !self.current_token_is(TokenType::Semicolon)
                    && !self.current_token_is(TokenType::RightBrace)
                {
                    self.next_token();
                }
            }
        }

        members
    }

    fn parse_interface_member(&mut self) -> Option<InterfaceMember> {
        self.parse_method_declaration(Visibility::Public, false) // Interface methods are always public and non-static
            .map(|member| match member {
                ClassMember::Method {
                    token,
                    name,
                    parameters,
                    return_type,
                    .. // Ignore visibility and is_static
                } => InterfaceMember::Method {
                    token,
                    name,
                    parameters,
                    return_type,
                },
                _ => unreachable!(),
            })
    }

    fn parse_import_declaration(&mut self) -> Option<Statement> {
        let token = self.current_token.clone();
        let mut imports = Vec::new();
        let mut path = String::new();

        if self.peek_token_is(TokenType::LeftBrace) {
            // Named imports: import { identifier1, identifier2 } from 'module-name';
            self.next_token(); // consume '{'

            while !self.peek_token_is(TokenType::RightBrace) {
                if !self.expect_peek(TokenType::Identifier(String::new())) {
                    return None;
                }
                let identifier = match self.current_token.token_type.clone() {
                    TokenType::Identifier(identifier) => identifier,
                    _ => unreachable!(),
                };

                imports.push(ImportSpecifier::Named(identifier));

                if !self.peek_token_is(TokenType::Comma) {
                    break;
                }
                self.next_token(); // consume ','
            }

            if !self.expect_peek(TokenType::RightBrace) {
                return None;
            }
        } else if self.expect_peek(TokenType::Identifier(String::new())) {
            // Default import: import identifier from 'module-name';
            let identifier = match self.current_token.token_type.clone() {
                TokenType::Identifier(identifier) => identifier,
                _ => unreachable!(),
            };

            imports.push(ImportSpecifier::Default(identifier));
        } else {
            self.peek_error(TokenType::Identifier(String::new()));
            return None;
        }

        if !self.expect_peek(TokenType::FromKeyword) {
            return None;
        }

        if !self.expect_peek(TokenType::String(String::new())) {
            return None;
        }
        path = match self.current_token.token_type.clone() {
            TokenType::String(path) => path,
            _ => unreachable!(),
        };

        if !self.expect_peek(TokenType::Semicolon) {
            return None;
        }

        Some(Statement::ImportDeclaration {
            token,
            imports,
            path,
        })
    }

    fn parse_export_declaration(&mut self) -> Option<Statement> {
        let token = self.current_token.clone();
        let mut specifiers = Vec::new();

        if self.peek_token_is(TokenType::DefaultKeyword) {
            // export default expression;
            self.next_token(); // consume 'default'
            specifiers.push(ExportSpecifier::Default);
        } else if self.peek_token_is(TokenType::LeftBrace) {
            // Named exports: export { identifier1, identifier2 };
            self.next_token(); // consume '{'

            while !self.peek_token_is(TokenType::RightBrace) {
                if !self.expect_peek(TokenType::Identifier(String::new())) {
                    return None;
                }
                let identifier = match self.current_token.token_type.clone() {
                    TokenType::Identifier(identifier) => identifier,
                    _ => unreachable!(),
                };

                specifiers.push(ExportSpecifier::Named(identifier));

                if !self.peek_token_is(TokenType::Comma) {
                    break;
                }
                self.next_token(); // consume ','
            }

            if !self.expect_peek(TokenType::RightBrace) {
                return None;
            }
        } else {
            // Error: Expected 'default' or '{'
            self.peek_error(TokenType::DefaultKeyword);
            return None;
        }

        if !self.expect_peek(TokenType::Semicolon) {
            return None;
        }

        Some(Statement::ExportDeclaration {
            token,
            specifiers,
        })
    }

    fn parse_block_statement(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();

        self.next_token(); // consume '{' or first statement

        while !self.current_token_is(TokenType::RightBrace)
            && !self.current_token_is(TokenType::EOF)
        {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }
            self.next_token();
        }

        statements
    }

    fn peek_precedence(&mut self) -> i32 {
        self.get_precedence(self.peek_token.token_type.clone())
    }

    fn get_precedence(&mut self, token_type: TokenType) -> i32 {
        match token_type {
            TokenType::Equals
            | TokenType::PlusEquals
            | TokenType::MinusEquals
            | TokenType::StarEquals
            | TokenType::SlashEquals
            | TokenType::PercentEquals => 1,
            TokenType::LogicalOr => 2,
            TokenType::LogicalAnd => 3,
            TokenType::EqualsEquals | TokenType::NotEquals => 4,
            TokenType::GreaterThan
            | TokenType::LessThan
            | TokenType::GreaterThanEquals
            | TokenType::LessThanEquals => 5,
            TokenType::Plus | TokenType::Minus => 6,
            TokenType::Star | TokenType::Slash | TokenType::Percent => 7,
            TokenType::LeftParen => 8,
            TokenType::LeftBracket => 9,
            TokenType::Dot => 10,
            _ => -1,
        }
    }

    fn prefix_precedence(&mut self) -> i32 {
        match self.current_token.token_type {
            TokenType::Minus | TokenType::LogicalNot => 7,
            _ => -1,
        }
    }

    fn infix_precedence(&mut self) -> i32 {
        self.get_precedence(self.current_token.token_type.clone())
    }

    fn assignment_precedence(&mut self) -> i32 {
        1 // Lowest precedence
    }
}