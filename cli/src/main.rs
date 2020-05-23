use logos::Span;
use possum::lexer::{self, TokenType};
use std::{
    env, fs,
    io::{self, BufRead, Write},
    path::Path,
    time::Instant,
};

fn main() {
    match env::args().nth(1) {
        Some(f) => run_file(f),
        None => run_prompt(),
    }
}

fn run_file<P: AsRef<Path>>(filename: P) {
    let source = fs::read_to_string(filename).unwrap();
    run(&source)
}

fn run_prompt() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let mut source = String::new();

    loop {
        println!();
        print!("> ");
        io::stdout().flush().unwrap();

        stdin.read_line(&mut source).unwrap();
        println!();

        run(&source);

        source.clear();
    }
}

fn run(source: &str) {
    let start = Instant::now();

    let tokens: Vec<(TokenType, Span)> = lexer::lexer(source).spanned().collect();

    let elapsed = start.elapsed();
    println!(
        "Parsed {} tokens in {} us",
        tokens.len(),
        elapsed.as_micros()
    );
    println!();

    for (token, span) in tokens {
        println!(
            "[{}..{}] {:?} - {:?}",
            span.start,
            span.end,
            &source[span.start..span.end],
            token,
        );
    }
}
