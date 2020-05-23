use logos::Logos;

#[derive(Debug, Copy, Clone, PartialEq, Logos)]
pub enum Token<'a> {
    #[regex(r"[ \t\n\r]+", logos::skip)]
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
    Equal,
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
    EqualEqual,
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

    #[regex(r#""(\\"|[^"])*""#, |lex| &lex.slice()[1..(lex.slice().len() - 1)])]
    StringLiteral(&'a str),
    #[regex(r"([0-9]+|0x[A-Fa-f0-9]+|0o[0-7]+|0b[01]+)", |lex| parse_integer(lex.slice()))]
    IntegerLiteral(i64),
    #[regex(r"[0-9]+\.[0-9]+", |lex| lex.slice().parse())]
    FloatLiteral(f64),
    #[token("true")]
    TrueLiteral,
    #[token("false")]
    FalseLiteral,

    #[regex(r"//.*[\n\r]")]
    Comment,

    #[regex(r"_*[A-Za-z][A-Za-z0-9_]*", |lex| lex.slice())]
    Identifier(&'a str),
}

fn parse_integer(slice: &str) -> Option<i64> {
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
