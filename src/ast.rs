use crate::lexer::{Literal, Scalar, Span, Token, TokenType};
use std::sync::atomic::{AtomicUsize, Ordering};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("unimplemented")]
    Unimplemented,
}

#[derive(Debug, Clone)]
pub enum Expr<'a> {
    Binary(Box<Binary<'a>>),
    Unary(Box<Unary<'a>>),
    Primary(Box<Primary<'a>>),

    Invalid(Option<Span>),
}

#[derive(Debug, Clone)]
pub struct Binary<'a> {
    lhs: Expr<'a>,
    op: (Scalar, Span),
    rhs: Expr<'a>,
}

#[derive(Debug, Clone)]
pub struct Unary<'a> {
    op: (Scalar, Span),
    rhs: Expr<'a>,
}

#[derive(Debug, Clone)]
pub enum Primary<'a> {
    Literal(&'a Literal, Span),
    Identifier(&'a str, Span),
    Block {
        lhs: (Scalar, Span),
        expr: Expr<'a>,
        rhs: (Scalar, Span),
    },
}

macro_rules! ass {
    ($name:ident, $($scalar:path),+ $(,)?) => {
        impl crate::lexer::Token<'_> {
            fn $name(&self) -> Option<(crate::lexer::Scalar, crate::lexer::Span)> {
                match self {
                    $(
                        Self {
                            ty: crate::lexer::TokenType::Scalar($scalar),
                            span: Span(s1, s2),
                        } => Some(($scalar, crate::lexer::Span(*s1, *s2))),
                    )+
                    _ => None,
                }
            }
        }
    }
}

macro_rules! binary {
    ($name:ident, $as_name:ident, $next_name:path, $($scalar:path),+ $(,)?) => {
        ass!($as_name, $($scalar),+);
        fn $name<'a, 'b>(
            tokens: &'a crate::ast::Tokens,
            errors: &'b mut Vec<crate::ast::Error>,
        ) -> crate::ast::Expr<'a> {
            let mut expr = $next_name(tokens, errors);
            while let Some(Some((t, s))) = tokens.peek().map(crate::lexer::Token::$as_name) {
                tokens.next();
                let rhs = $next_name(tokens, errors);
                expr = Expr::Binary(Box::new(Binary { lhs: expr, op: (t, s), rhs }));
            }
            expr
        }
    }
}

binary!(
    equality,
    as_equality,
    comparison,
    Scalar::Equal,
    Scalar::NotEqual,
);
binary!(
    comparison,
    as_comparison,
    addition,
    Scalar::GreaterEqual,
    Scalar::LessEqual,
    Scalar::Greater,
    Scalar::Less,
);
binary!(
    addition,
    as_addition,
    multiplication,
    Scalar::Plus,
    Scalar::Minus
);
binary!(
    multiplication,
    as_multiplication,
    unary,
    Scalar::Star,
    Scalar::Slash,
    Scalar::Modulo,
);

ass!(as_unary, Scalar::Bang, Scalar::Plus, Scalar::Minus);

pub fn parse<'a>(tokens: &'a Tokens<'a>) -> (Expr<'a>, Vec<Error>) {
    let mut errors = Vec::new();
    let expr = equality(tokens, &mut errors);
    (expr, errors)
}

fn unary<'a, 'b>(tokens: &'a Tokens<'a>, errors: &'b mut Vec<Error>) -> Expr<'a> {
    if let Some(Some((t, s))) = tokens.peek().map(Token::as_unary) {
        tokens.next();
        let rhs = unary(tokens, errors);
        return Expr::Unary(Box::new(Unary { op: (t, s), rhs }));
    }

    primary(tokens, errors)
}

fn primary<'a, 'b>(tokens: &'a Tokens<'a>, errors: &'b mut Vec<Error>) -> Expr<'a> {
    match tokens.peek() {
        Some(Token {
            ty: TokenType::Literal(l),
            span,
        }) => {
            tokens.next();
            Expr::Primary(Box::new(Primary::Literal(l, *span)))
        }

        Some(Token {
            ty: TokenType::Identifier(i),
            span,
        }) => {
            tokens.next();
            Expr::Primary(Box::new(Primary::Identifier(i, *span)))
        }

        _ => {
            errors.push(Error::Unimplemented);
            let span = tokens.next().map(|t| t.span);
            Expr::Invalid(span)
        }
    }
}

pub struct Tokens<'a> {
    slice: &'a [Token<'a>],
    cursor: AtomicUsize,
}

impl<'a> Tokens<'a> {
    pub fn new(slice: &'a [Token<'a>]) -> Self {
        Self {
            slice,
            cursor: AtomicUsize::new(0),
        }
    }

    fn next(&self) -> Option<&Token> {
        self.slice.get(self.cursor.fetch_add(1, Ordering::Relaxed))
    }

    fn peek(&self) -> Option<&Token> {
        self.slice.get(self.cursor.load(Ordering::Relaxed))
    }
}
