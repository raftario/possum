use crate::lexer::Error::InvalidEscape;
use logos::{Logos, Span};
use std::num::{ParseFloatError, ParseIntError};
use thiserror::Error;

/// Represents a lexing error
#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("invalid token")]
    InvalidToken,

    #[error("invalid integer: {0}")]
    InvalidInteger(#[from] ParseIntError),
    #[error("invalid float: {0}")]
    InvalidFloat(#[from] ParseFloatError),

    #[error("invalid escape sequence: {0}")]
    InvalidEscape(String),
}

/// Represents the possible source tokens
#[derive(Debug, Copy, Clone, PartialEq, Logos)]
pub enum TokenType {
    #[regex(r"[ \n\t\r]+", logos::skip)]
    #[error]
    Error,

    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token("[")]
    LeftBracket,
    #[token("]")]
    RightBracket,

    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("%")]
    Modulo,
    #[token("|")]
    Bar,
    #[token("&")]
    Ampersand,
    #[token("^")]
    Hat,
    #[token(".")]
    Dot,
    #[token(",")]
    Comma,
    #[token("_")]
    Underscore,
    #[token("=")]
    Assign,
    #[token("!")]
    Bang,
    #[token("?")]
    Question,
    #[token("~")]
    Tilde,
    #[token(":")]
    Colon,
    #[token(";")]
    Semicolon,

    #[token("||")]
    Or,
    #[token("&&")]
    And,

    #[token("+=")]
    PlusAssign,
    #[token("-=")]
    MinusAssign,
    #[token("*=")]
    TimesAssign,
    #[token("/=")]
    DivAssign,
    #[token("%=")]
    ModuloAssign,
    #[token("|=")]
    OrAssign,
    #[token("&=")]
    AndAssign,
    #[token("^=")]
    XorAssign,

    #[token("::")]
    Path,
    #[token("..")]
    Range,

    #[token("==")]
    Equal,
    #[token("!=")]
    NotEqual,

    #[token(">=")]
    GreaterEqual,
    #[token("<=")]
    LessEqual,
    #[token(">")]
    Greater,
    #[token("<")]
    Less,

    #[token("let")]
    Let,
    #[token("const")]
    Const,
    #[token("global")]
    Global,
    #[token("fn")]
    Fn,
    #[token("struct")]
    Struct,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("loop")]
    Loop,
    #[token("for")]
    For,
    #[token("in")]
    In,
    #[token("mut")]
    Mut,
    #[token("import")]
    Import,
    #[token("export")]
    Export,
    #[token("use")]
    Use,
    #[token("type")]
    Type,
    #[token("constraint")]
    Constraint,
    #[token("is")]
    Is,

    #[token("return")]
    Return,
    #[token("break")]
    Break,
    #[token("continue")]
    Continue,

    #[regex(r"//.*[\n\r]")]
    Comment,

    // Literals
    #[regex(r"[0-9](_?[0-9])*")]
    IntegerLiteral,
    #[regex(r"0[Xx][A-Fa-f0-9](_?[A-Fa-f0-9])*")]
    HexIntegerLiteral,
    #[regex(r"0[Oo][0-7](_?[0-7])*")]
    OctIntegerLiteral,
    #[regex(r"0[Bb][01](_?[01])*")]
    BinIntegerLiteral,
    #[regex(r"[0-9](_?[0-9])*\.[0-9](_?[0-9])*")]
    FloatLiteral,
    #[token("true")]
    TrueLiteral,
    #[token("false")]
    FalseLiteral,
    #[regex(r#""(\\"|[^"])*""#)]
    StringLiteral,
    #[regex(r#"'(\\'|\\x[0-7][A-Fa-f0-9]|\\?[^'])'"#)]
    CharLiteral,
    #[regex(r#"b"(\\"|[\x00-\x21\x23-\x7F])*""#)]
    ByteStringLiteral,
    #[regex(r#"b'(\\'|\\x[0-7][A-Fa-f0-9]|\\?[\x00-\x26\x28-\x7F])'"#)]
    ByteLiteral,

    // Identifiers
    #[regex(r"_*[A-Za-z][A-Za-z0-9_]*")]
    Identifier,
}

#[derive(Debug, Clone)]
pub enum Token<'a> {
    Scalar(TokenType),
    Literal(Literal),
    Identifier(&'a str),
}

#[derive(Debug, Clone)]
pub enum Literal {
    Integer(u64),
    Float(f64),
    Bool(bool),
    String(String),
    Char(char),
    ByteString(Vec<u8>),
    Byte(u8),
}

/// Parses the provided source code into an iterator of tokens
pub fn lex<'a>(source: &'a str) -> impl Iterator<Item = (Result<Token<'a>, Error>, Span)> + 'a {
    TokenType::lexer(source).spanned().map(move |(ty, span)| {
        let token =
            match ty {
                TokenType::Error => Err(Error::InvalidToken),

                TokenType::IntegerLiteral => parse_int(&source[span.start..span.end])
                    .map(|i| Token::Literal(Literal::Integer(i))),
                TokenType::HexIntegerLiteral => parse_hex_int(&source[span.start..span.end])
                    .map(|i| Token::Literal(Literal::Integer(i))),
                TokenType::OctIntegerLiteral => parse_oct_int(&source[span.start..span.end])
                    .map(|i| Token::Literal(Literal::Integer(i))),
                TokenType::BinIntegerLiteral => parse_bin_int(&source[span.start..span.end])
                    .map(|i| Token::Literal(Literal::Integer(i))),

                TokenType::FloatLiteral => parse_float(&source[span.start..span.end])
                    .map(|f| Token::Literal(Literal::Float(f))),

                TokenType::TrueLiteral => Ok(Token::Literal(Literal::Bool(true))),
                TokenType::FalseLiteral => Ok(Token::Literal(Literal::Bool(false))),

                TokenType::StringLiteral => parse_str(&source[span.start..span.end])
                    .map(|s| Token::Literal(Literal::String(s))),
                TokenType::CharLiteral => parse_char(&source[span.start..span.end])
                    .map(|c| Token::Literal(Literal::Char(c))),
                TokenType::ByteStringLiteral => parse_byte_str(&source[span.start..span.end])
                    .map(|bs| Token::Literal(Literal::ByteString(bs))),
                TokenType::ByteLiteral => parse_byte(&source[span.start..span.end])
                    .map(|b| Token::Literal(Literal::Byte(b))),

                TokenType::Identifier => Ok(Token::Identifier(&source[span.start..span.end])),

                _ => Ok(Token::Scalar(ty)),
            };

        (token, span)
    })
}

/// Parses a decimal integer literal to an actual integer
fn parse_int(slice: &str) -> Result<u64, Error> {
    filter_underscores(slice).parse().map_err(Into::into)
}

/// Parses an hexadecimal integer literal to an actual integer
fn parse_hex_int(slice: &str) -> Result<u64, Error> {
    u64::from_str_radix(&filter_underscores(&slice[2..]), 16).map_err(Into::into)
}

/// Parses an octal integer literal to an actual integer
fn parse_oct_int(slice: &str) -> Result<u64, Error> {
    u64::from_str_radix(&filter_underscores(&slice[2..]), 8).map_err(Into::into)
}

/// Parses a binary integer literal to an actual integer
fn parse_bin_int(slice: &str) -> Result<u64, Error> {
    u64::from_str_radix(&filter_underscores(&slice[2..]), 2).map_err(Into::into)
}

fn parse_float(slice: &str) -> Result<f64, Error> {
    filter_underscores(slice).parse().map_err(Into::into)
}

/// Removes underscores from a slice
fn filter_underscores(slice: &str) -> String {
    slice.chars().filter(|c| *c != '_').collect()
}

// TODO: Unicode escape support

/// Parses a string literal to an actual string
fn parse_str(slice: &str) -> Result<String, Error> {
    // Ignore quotes
    let slice = &slice[1..(slice.len() - 1)];

    let mut res = String::with_capacity(slice.len());
    let mut chars = slice.chars();

    while let Some(c) = chars.next() {
        res.push(match c {
            '\\' => unescape(&mut chars)?,
            _ => c,
        });
    }

    Ok(res)
}

/// Parses a char literal to an actual char
fn parse_char(slice: &str) -> Result<char, Error> {
    // Ignore quotes
    let slice = &slice[1..(slice.len() - 1)];

    let mut chars = slice.chars();
    match chars.next() {
        Some('\\') => unescape(&mut chars),
        Some(c) => Ok(c),
        _ => unreachable!(),
    }
}

/// Parses a byte string literal to an actual byte string
fn parse_byte_str(slice: &str) -> Result<Vec<u8>, Error> {
    // Ignore quotes and prefix
    let slice = slice[2..(slice.len() - 1)].as_bytes();

    let mut res = Vec::with_capacity(slice.len());
    let mut bytes = slice.iter().copied();

    while let Some(b) = bytes.next() {
        res.push(match b {
            b'\\' => unescape(&mut bytes)? as u8,
            _ => b,
        });
    }

    Ok(res)
}

/// Parses a byte literal to an actual byte
fn parse_byte(slice: &str) -> Result<u8, Error> {
    // Ignore quotes and prefix
    let slice = slice[2..(slice.len() - 1)].as_bytes();

    let mut bytes = slice.iter().copied();
    match bytes.next() {
        // The cast is safe since byte literals will not match unicode
        Some(b'\\') => Ok(unescape(&mut bytes)? as u8),
        Some(b) => Ok(b),
        _ => unreachable!(),
    }
}

/// Converts an escape code to the character it represents
fn unescape<Char, Chars>(chars: &mut Chars) -> Result<char, Error>
where
    Char: Into<char>,
    Chars: Iterator<Item = Char>,
{
    match chars.next().map(Into::into) {
        Some('"') => Ok('"'),
        Some('\'') => Ok('\''),
        Some('\\') => Ok('\\'),

        Some('n') => Ok('\n'),
        Some('t') => Ok('\t'),
        Some('r') => Ok('\r'),

        Some('0') => Ok('\0'),

        // ASCII
        Some('x') => match chars.next().map(Into::into) {
            Some(n1) if is_ascii_octdigit(n1) => match chars.next().map(Into::into) {
                Some(n2) if n2.is_ascii_hexdigit() => {
                    u8::from_str_radix(&[n1, n2].iter().collect::<String>(), 16)
                        .map(Into::into)
                        .map_err(Into::into)
                }

                Some(n2) => Err(InvalidEscape(['\\', 'x', n1, n2].iter().collect())),
                None => Err(InvalidEscape(['\\', 'x', n1].iter().collect())),
            },

            Some(n1) => Err(InvalidEscape(['\\', 'x', n1].iter().collect())),
            None => Err(InvalidEscape(['\\', 'x'].iter().collect())),
        },

        Some(c) => Err(Error::InvalidEscape(['\\', c].iter().collect())),
        None => Err(Error::InvalidEscape(['\\'].iter().collect())),
    }
}

fn is_ascii_octdigit(c: char) -> bool {
    c >= '0' && c <= '7'
}
