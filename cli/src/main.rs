use possum::lexer;
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

    let mut tokens = Vec::new();
    let mut errors = Vec::new();
    for (result, span) in lexer::lex(source) {
        match result {
            Ok(t) => tokens.push((t, span)),
            Err(e) => errors.push((e, span)),
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

    for (error, span) in errors {
        println!(
            "[{}..{}] {:?} - {:?}",
            span.start,
            span.end,
            &source[span.start..span.end],
            error,
        );
    }
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
