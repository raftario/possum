use possum::{
    ast::{self, Tokens},
    lexer,
};
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
    let mut tokens = Vec::new();
    let mut errors = Vec::new();

    let start = Instant::now();

    for result in lexer::lex(source) {
        match result {
            Ok(t) => tokens.push(t),
            Err(e) => errors.push(e),
        }
    }

    let elapsed = start.elapsed();

    println!(
        "Parsed {} tokens in {} us with {} errors",
        tokens.len(),
        elapsed.as_micros(),
        errors.len(),
    );
    println!();

    if !errors.is_empty() {
        for (error, span) in errors {
            println!(
                "[{}..{}] {:?} - {:?}",
                span.0,
                span.1,
                &source[span.0..span.1],
                error,
            );
        }
        println!();
    }

    for token in &tokens {
        println!("{:?}", token);
    }

    println!();

    let tokens = Tokens::new(&tokens);

    let start = Instant::now();
    let (ast, errors) = ast::parse(&tokens);
    let elapsed = start.elapsed();

    println!(
        "Parsed AST in {} us with {} errors",
        elapsed.as_micros(),
        errors.len(),
    );
    println!();

    if !errors.is_empty() {
        for error in errors {
            println!("{:?}", error);
        }
        println!();
    }

    println!("{:#?}", ast);
}
