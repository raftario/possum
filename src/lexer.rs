use logos::Logos;

/// Represents the possible source tokens
#[derive(Debug, Clone, PartialEq, Logos)]
pub enum Token<'a> {
    #[regex(r"[ \n\t\r]+", logos::skip)]
    #[error]
    Error,

    #[token(r"(")]
    LeftParen,
    #[token(r")")]
    RightParen,
    #[token(r"{")]
    LeftBrace,
    #[token(r"}")]
    RightBrace,
    #[token(r"[")]
    LeftBracket,
    #[token(r"]")]
    RightBracket,

    #[token(r"+")]
    Plus,
    #[token(r"-")]
    Minus,
    #[token(r"*")]
    Star,
    #[token(r"/")]
    Slash,
    #[token(r"%")]
    Modulo,
    #[token(r"|")]
    Bar,
    #[token(r"&")]
    Ampersand,
    #[token(r"^")]
    Hat,
    #[token(r".")]
    Dot,
    #[token(r",")]
    Comma,
    #[token(r"_")]
    Underscore,
    #[token(r"=")]
    Assign,
    #[token(r"!")]
    Bang,
    #[token(r"~")]
    Tilde,
    #[token(r":")]
    Colon,
    #[token(r";")]
    Semicolon,

    #[token(r"||")]
    Or,
    #[token(r"&&")]
    And,
    #[token(r"+=")]
    PlusAssign,
    #[token(r"-=")]
    MinusAssign,
    #[token(r"*=")]
    TimesAssign,
    #[token(r"/=")]
    DivAssign,
    #[token(r"%=")]
    ModuloAssign,
    #[token(r"|=")]
    OrAssign,
    #[token(r"&=")]
    AndAssign,
    #[token(r"^=")]
    XorAssign,

    #[token("::")]
    Path,
    #[token("..")]
    Range,

    #[token(r"==")]
    Equal,
    #[token(r"!=")]
    NotEqual,
    #[token(r">=")]
    GreaterEqual,
    #[token(r"<=")]
    LessEqual,
    #[token(r">")]
    Greater,
    #[token(r"<")]
    Less,

    #[token(r"let")]
    Let,
    #[token(r"const")]
    Const,
    #[token(r"global")]
    Global,
    #[token(r"fn")]
    Fn,
    #[token(r"struct")]
    Struct,
    #[token(r"if")]
    If,
    #[token(r"else")]
    Else,
    #[token(r"loop")]
    Loop,
    #[token(r"for")]
    For,
    #[token("in")]
    In,
    #[token(r"mut")]
    Mut,
    #[token(r"import")]
    Import,
    #[token(r"export")]
    Export,

    #[token(r"return")]
    Return,
    #[token(r"break")]
    Break,
    #[token(r"continue")]
    Continue,

    #[regex(r"([0-9]+|0x[A-Fa-f0-9]+|0o[0-7]+|0b[01]+)", |lex| parse_integer(lex.slice()))]
    IntegerLiteral(i64),
    #[regex(r"[0-9]+\.[0-9]+", |lex| lex.slice().parse())]
    FloatLiteral(f64),
    #[regex("true|false", |lex| parse_bool(lex.slice()))]
    BoolLiteral(bool),
    #[regex(r#""(\\"|[^"])*""#, |lex| parse_str(lex.slice()))]
    StringLiteral(String),
    #[regex(r#"'(\\'|\\?[^']|)'"#, |lex| parse_char(lex.slice()))]
    CharLiteral(char),
    #[regex(r#"b"(\\"|[\x00-\x21\x23-\x7F])*""#, |lex| parse_byte_str(lex.slice()))]
    ByteStringLiteral(Vec<u8>),
    #[regex(r#"b'(\\'|\\?[\x00-\x26\x28-\x7F]|)'"#, |lex| parse_byte(lex.slice()))]
    ByteLiteral(u8),

    #[regex(r"//.*[\n\r]")]
    Comment,

    #[regex(r"_*[A-Za-z][A-Za-z0-9_]*", |lex| lex.slice())]
    Identifier(&'a str),
}

/// Parses an integer literal to an actual integer
/// Works for decimal, hexadecimal, octal and binary representations
pub fn parse_integer(slice: &str) -> Option<i64> {
    let bytes = slice.as_bytes();
    match (slice.len() >= 2, bytes[0]) {
        (true, b'0') => match bytes[1] {
            b'x' => i64::from_str_radix(&slice[2..], 16).ok(),
            b'o' => i64::from_str_radix(&slice[2..], 8).ok(),
            b'b' => i64::from_str_radix(&slice[2..], 2).ok(),
            _ => slice.parse().ok(),
        },
        _ => slice.parse().ok(),
    }
}

/// Parses a boolean literal to an actual boolean
pub fn parse_bool(slice: &str) -> Option<bool> {
    match slice {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}

/// Parses a string literal to an actual string
pub fn parse_str(slice: &str) -> Option<String> {
    // Ignore quotes
    let slice = &slice[1..(slice.len() - 1)];

    let mut res = String::with_capacity(slice.len());
    let mut chars = slice.chars();

    while let Some(c) = chars.next() {
        res.push(match c {
            '\\' => unescape(chars.next()?)?.into(),
            _ => c,
        });
    }

    Some(res)
}

/// Parses a char literal to an actual char
pub fn parse_char(slice: &str) -> Option<char> {
    // Ignore quotes
    let slice = &slice[1..(slice.len() - 1)];

    let mut chars = slice.chars();
    match chars.next() {
        Some('\\') => unescape(chars.next()?).map(Into::into),
        Some(c) => Some(c),
        None => None,
    }
}

/// Parses a byte string literal to an actual byte string
pub fn parse_byte_str(slice: &str) -> Option<Vec<u8>> {
    // Ignore quotes and prefix
    let slice = slice[2..(slice.len() - 1)].as_bytes();

    let mut res = Vec::with_capacity(slice.len());
    let mut bytes = slice.iter();

    while let Some(b) = bytes.next() {
        res.push(match b {
            b'\\' => unescape((*bytes.next()?).into())?,
            _ => *b,
        });
    }

    Some(res)
}

/// Parses a byte literal to an actual byte
pub fn parse_byte(slice: &str) -> Option<u8> {
    // Ignore quotes and prefix
    let slice = slice[2..(slice.len() - 1)].as_bytes();

    let mut bytes = slice.iter();
    match bytes.next() {
        Some(b'\\') => unescape((*bytes.next()?).into()),
        Some(b) => Some(*b),
        None => None,
    }
}

// TODO: ASCII & Unicode escape support
/// Converts an escape code to the character it represents
pub fn unescape(c: char) -> Option<u8> {
    match c {
        '"' => Some(b'"'),
        '\'' => Some(b'\''),
        '\\' => Some(b'\\'),

        'n' => Some(b'\n'),
        't' => Some(b'\t'),
        'r' => Some(b'\r'),

        '0' => Some(b'\0'),

        _ => None,
    }
}
