use logos::Logos;

#[derive(Debug, Copy, Clone, PartialEq, Logos)]
enum Token<'a> {
    #[regex(r"[ \t\n\r\f]+", logos::skip)]
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
    SemiColon,

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
    #[token(r"mut")]
    Mut,

    #[regex(r"//.*[\n\r]")]
    Comment,

    #[regex(r#"".*""#, |lex| &lex.slice()[1..lex.slice().len() - 1])]
    StringLiteral(&'a str),
    #[regex(r"[0-9_]+", |lex| lex.slice().chars().filter(|c| *c != '_').collect::<String>().parse())]
    IntegerLiteral(i64),
    #[regex(r"[0-9_]+\.[0-9_]+", |lex| lex.slice().chars().filter(|c| *c != '_').collect::<String>().parse())]
    FloatLiteral(f64),

    #[regex(r"_*[A-Za-z][A-Za-z0-9_]*", |lex| lex.slice())]
    Identifier(&'a str),
}
