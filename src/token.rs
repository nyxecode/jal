#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Literals
    Identifier(String),
    Int(i32),
    Float(f32),
    String(String),

    // Keywords
    IntKeyword,
    FloatKeyword,
    StringKeyword,
    BoolKeyword,
    TrueKeyword,
    FalseKeyword,
    ConstKeyword,
    IfKeyword,
    ElseKeyword,
    DoKeyword,
    WhileKeyword,
    ForKeyword,
    OfKeyword,
    SwitchKeyword,
    CaseKeyword,
    BreakKeyword,
    ContinueKeyword,
    FunctionKeyword,
    ReturnKeyword,
    EnumKeyword,
    ObjectKeyword,
    DictKeyword,
    ClassKeyword,
    ExtendsKeyword,
    ImplementsKeyword,
    InterfaceKeyword,
    PublicKeyword,
    PrivateKeyword,
    StaticKeyword,
    ImportKeyword,
    FromKeyword,
    ExportKeyword,
    DefaultKeyword,
    NewKeyword,
    ThisKeyword,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    PlusPlus,
    MinusMinus,
    Equals,
    PlusEquals,
    MinusEquals,
    StarEquals,
    SlashEquals,
    PercentEquals,
    EqualsEquals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterThanEquals,
    LessThanEquals,
    LogicalAnd,
    LogicalOr,
    LogicalNot,

    // Punctuation
    Semicolon,
    Comma,
    Colon,
    Dot,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    FatArrow,
    EqualsGreaterThan,

    // Other
    EOF,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(token_type: TokenType, line: usize, column: usize) -> Self {
        Token {
            token_type,
            line,
            column,
        }
    }
}