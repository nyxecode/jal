use regex::Regex;
use crate::token::{Token, TokenType};
use crate::error::LexerError;

pub struct Lexer {
    pub source: String,
    pub current_char: Option<char>,
    pub current_position: usize,
    pub line: usize,
    pub column: usize,
    pub tokens: Vec<Token>,
    pub errors: Vec<LexerError>,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        let mut lexer = Lexer {
            source: source.to_string(),
            current_char: Some(source.chars().next().unwrap_or('\0')),
            current_position: 0,
            line: 1,
            column: 1,
            tokens: Vec::new(),
            errors: Vec::new(),
        };
        lexer.consume(); // Initialize current_char
        lexer
    }

    pub fn tokenize(&mut self) {
        // Define regular expressions for literals
        let int_regex = Regex::new(r"^\d+").unwrap();
        let float_regex = Regex::new(r"^\d+\.\d+").unwrap();
        let string_regex = Regex::new(r#"^"([^"\\]|\\.)*""#).unwrap(); // Supports escaped quotes
        // Regex for single-line comments
        let single_line_comment_regex = Regex::new(r"//.*").unwrap();
        // Regex for multi-line comments
        let multi_line_comment_regex = Regex::new(r"/\*[\s\S]*?\*/").unwrap();

        while let Some(c) = self.current_char {
            match c {
                ' ' | '\t' => self.consume(),
                '\n' => {
                    self.line += 1;
                    self.column = 1;
                    self.consume();
                }
                '/' => {
                    // Check for single-line comments
                    if self.peek() == Some('/') {
                        self.skip_comment(&single_line_comment_regex);
                    }
                    // Check for multi-line comments
                    else if self.peek() == Some('*') {
                        self.skip_comment(&multi_line_comment_regex);
                    } else {
                        self.add_token(TokenType::Slash);
                        self.consume();
                    }
                }
                '+' => {
                    if self.peek() == Some('+') {
                        self.consume();
                        self.add_token(TokenType::PlusPlus);
                    } else if self.peek() == Some('=') {
                        self.consume();
                        self.add_token(TokenType::PlusEquals);
                    } else {
                        self.add_token(TokenType::Plus);
                    }
                    self.consume();
                }
                '-' => {
                    if self.peek() == Some('-') {
                        self.consume();
                        self.add_token(TokenType::MinusMinus);
                    } else if self.peek() == Some('=') {
                        self.consume();
                        self.add_token(TokenType::MinusEquals);
                    } else {
                        self.add_token(TokenType::Minus);
                    }
                    self.consume();
                }
                '*' => {
                    if self.peek() == Some('=') {
                        self.consume();
                        self.add_token(TokenType::StarEquals);
                    } else {
                        self.add_token(TokenType::Star);
                    }
                    self.consume();
                }
                '/' => {
                    if self.peek() == Some('=') {
                        self.consume();
                        self.add_token(TokenType::SlashEquals);
                    } else {
                        self.add_token(TokenType::Slash);
                    }
                    self.consume();
                }
                '%' => {
                    if self.peek() == Some('=') {
                        self.consume();
                        self.add_token(TokenType::PercentEquals);
                    } else {
                        self.add_token(TokenType::Percent);
                    }
                    self.consume();
                }
                '=' => {
                    if self.peek() == Some('=') {
                        self.consume();
                        self.add_token(TokenType::EqualsEquals);
                    } else if self.peek() == Some('>') {
                        self.consume();
                        self.add_token(TokenType::EqualsGreaterThan);
                    } else {
                        self.add_token(TokenType::Equals);
                    }
                    self.consume();
                }
                '!' => {
                    if self.peek() == Some('=') {
                        self.consume();
                        self.add_token(TokenType::NotEquals);
                    } else {
                        self.add_token(TokenType::LogicalNot);
                    }
                    self.consume();
                }
                '>' => {
                    if self.peek() == Some('=') {
                        self.consume();
                        self.add_token(TokenType::GreaterThanEquals);
                    } else {
                        self.add_token(TokenType::GreaterThan);
                    }
                    self.consume();
                }
                '<' => {
                    if self.peek() == Some('=') {
                        self.consume();
                        self.add_token(TokenType::LessThanEquals);
                    } else {
                        self.add_token(TokenType::LessThan);
                    }
                    self.consume();
                }
                '&' => {
                    if self.peek() == Some('&') {
                        self.consume();
                        self.add_token(TokenType::LogicalAnd);
                    } else {
                        self.error("Invalid character '&'".to_string());
                        self.consume();
                    }
                }
                '|' => {
                    if self.peek() == Some('|') {
                        self.consume();
                        self.add_token(TokenType::LogicalOr);
                    } else {
                        self.error("Invalid character '|'".to_string());
                        self.consume();
                    }
                }
                ';' => {
                    self.add_token(TokenType::Semicolon);
                    self.consume();
                }
                ',' => {
                    self.add_token(TokenType::Comma);
                    self.consume();
                }
                ':' => {
                    self.add_token(TokenType::Colon);
                    self.consume();
                }
                '.' => {
                    self.add_token(TokenType::Dot);
                    self.consume();
                }
                '(' => {
                    self.add_token(TokenType::LeftParen);
                    self.consume();
                }
                ')' => {
                    self.add_token(TokenType::RightParen);
                    self.consume();
                }
                '{' => {
                    self.add_token(TokenType::LeftBrace);
                    self.consume();
                }
                '}' => {
                    self.add_token(TokenType::RightBrace);
                    self.consume();
                }
                '[' => {
                    self.add_token(TokenType::LeftBracket);
                    self.consume();
                }
                ']' => {
                    self.add_token(TokenType::RightBracket);
                    self.consume();
                }
                '=' => {
                    if self.peek() == Some('>') {
                        self.consume();
                        self.add_token(TokenType::EqualsGreaterThan);
                    } else {
                        self.add_token(TokenType::Equals);
                    }
                    self.consume();
                }
                '\"' => self.string(&string_regex),
                '0'..='9' => self.number(&int_regex, &float_regex),
                _ if self.is_valid_identifier_start(c) => self.identifier(),
                _ => {
                    self.error(format!("Unexpected character: '{}'", c));
                    self.consume();
                }
            }
        }
        self.add_token(TokenType::EOF);
    }

    fn consume(&mut self) {
        self.current_position += 1;
        self.column += 1;
        self.current_char = self.source[self.current_position..].chars().next();
    }

    fn peek(&self) -> Option<char> {
        self.source[self.current_position + 1..].chars().next()
    }

    fn add_token(&mut self, token_type: TokenType) {
        let token = Token::new(token_type, self.line, self.column);
        self.tokens.push(token);
    }

    fn skip_comment(&mut self, regex: &Regex) {
        let remaining_source = &self.source[self.current_position..];
        if let Some(mat) = regex.find(remaining_source) {
            let comment_end = mat.end(); // Get the end index of the comment
            self.current_position += comment_end; // Move the current position past the comment
            self.column += comment_end; // Update the column
            self.current_char = self.source[self.current_position..].chars().next();
        }
    }

    fn string(&mut self, regex: &Regex) {
        let remaining_source = &self.source[self.current_position..];
        if let Some(mat) = regex.find(remaining_source) {
            let string_literal = mat.as_str();
            self.consume_matched_string(string_literal);
            // Remove the quotes and add the token
            let value = self.process_escape_sequences(&string_literal[1..string_literal.len() - 1]);
            self.add_token(TokenType::String(value));
        } else {
            self.error("Unterminated string literal".to_string());
        }
    }

    fn process_escape_sequences(&self, input: &str) -> String {
        let mut result = String::new();
        let mut chars = input.chars();
        while let Some(c) = chars.next() {
            if c == '\\' {
                // Escape sequence
                match chars.next() {
                    Some('n') => result.push('\n'),
                    Some('t') => result.push('\t'),
                    Some('r') => result.push('\r'),
                    Some('\\') => result.push('\\'),
                    Some('"') => result.push('"'),
                    Some(other) => {
                        // Invalid escape sequence - handle as needed
                        result.push('\\');
                        result.push(other);
                    }
                    None => result.push('\\'), // Backslash at the end of the string
                }
            } else {
                result.push(c);
            }
        }
        result
    }

    fn number(&mut self, int_regex: &Regex, float_regex: &Regex) {
        let remaining_source = &self.source[self.current_position..];

        // Try to match float first, then int
        if let Some(mat) = float_regex.find(remaining_source) {
            let float_literal = mat.as_str();
            self.consume_matched_string(float_literal);
            // Parse the float and add the token
            match float_literal.parse::<f32>() {
                Ok(float_val) => {
                    self.add_token(TokenType::Float(float_val));
                }
                Err(_) => {
                    self.error("Invalid float literal".to_string());
                }
            }
        } else if let Some(mat) = int_regex.find(remaining_source) {
            let int_literal = mat.as_str();
            self.consume_matched_string(int_literal);
            // Parse the integer and add the token
            match int_literal.parse::<i32>() {
                Ok(int_val) => {
                    self.add_token(TokenType::Int(int_val));
                }
                Err(_) => {
                    self.error("Invalid integer literal".to_string());
                }
            }
        } else {
            self.error("Invalid number literal".to_string());
        }
    }

    fn identifier(&mut self) {
        let start_position = self.current_position;

        while self.current_char.is_some()
            && (self.is_valid_identifier_start(self.current_char.unwrap())
            || self.current_char.unwrap().is_digit(10)
            || self.current_char.unwrap() == '_')
        {
            self.consume();
        }

        let identifier = self.source[start_position..self.current_position].to_string();

        let token_type = match identifier.as_str() {
            "int" => TokenType::IntKeyword,
            "float" => TokenType::FloatKeyword,
            "string" => TokenType::StringKeyword,
            "bool" => TokenType::BoolKeyword,
            "true" => TokenType::TrueKeyword,
            "false" => TokenType::FalseKeyword,
            "const" => TokenType::ConstKeyword,
            "if" => TokenType::IfKeyword,
            "else" => TokenType::ElseKeyword,
            "do" => TokenType::DoKeyword,
            "while" => TokenType::WhileKeyword,
            "for" => TokenType::ForKeyword,
            "of" => TokenType::OfKeyword,
            "switch" => TokenType::SwitchKeyword,
            "case" => TokenType::CaseKeyword,
            "break" => TokenType::BreakKeyword,
            "continue" => TokenType::ContinueKeyword,
            "function" => TokenType::FunctionKeyword,
            "return" => TokenType::ReturnKeyword,
            "enum" => TokenType::EnumKeyword,
            "object" => TokenType::ObjectKeyword,
            "dict" => TokenType::DictKeyword,
            "class" => TokenType::ClassKeyword,
            "extends" => TokenType::ExtendsKeyword,
            "implements" => TokenType::ImplementsKeyword,
            "interface" => TokenType::InterfaceKeyword,
            "public" => TokenType::PublicKeyword,
            "private" => TokenType::PrivateKeyword,
            "static" => TokenType::StaticKeyword,
            "import" => TokenType::ImportKeyword,
            "from" => TokenType::FromKeyword,
            "export" => TokenType::ExportKeyword,
            "default" => TokenType::DefaultKeyword,
            "new" => TokenType::NewKeyword,
            "this" => TokenType::ThisKeyword,
            _ => TokenType::Identifier(identifier),
        };

        self.add_token(token_type);
    }

    fn is_valid_identifier_start(&self, c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }

    fn error(&mut self, message: String) {
        let error = LexerError {
            message,
            line: self.line,
            column: self.column,
        };
        self.errors.push(error);
    }

    fn consume_matched_string(&mut self, matched_string: &str) {
        // Consume the matched string, updating position, line, and column
        self.current_position += matched_string.len();
        // Update line and column based on newlines
        for c in matched_string.chars() {
            if c == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
        self.current_char = self.source[self.current_position..].chars().next();
    }
}