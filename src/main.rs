use std::{env::args, error::Error, io::stdin};

type RuntimeError = Box<dyn Error>;

fn main() -> Result<(), RuntimeError> {
    let args: Vec<String> = args().collect();
    if args.len() > 1 {
        panic!("too much arguments")
    } else if args.len() == 1 {
        Lox::run_file(&args[0])?;
    } else {
        Lox::run_prompt()?;
    }
    Ok(())
}

struct Lox {
    had_error: bool,
}

impl Lox {
    fn run_file(path: &str) -> Result<(), RuntimeError> {
        let data = std::fs::read_to_string(path)?;
        run(&data)?;
        Ok(())
    }

    fn run_prompt() -> Result<(), RuntimeError> {
        let mut buf = String::new();
        let input_stream = stdin();
        loop {
            let n = input_stream.read_line(&mut buf)?;
            if n == 0 {
                break;
            };
            run(&buf);
        }
        Ok(())
    }

    fn error(&mut self, line: usize, message: String) {
        self.report(line, "".into(), message);
    }

    fn report(&mut self, line: usize, where_: String, message: String) {
        println!("[line {line}] Error {where_} : {message}");
        self.had_error = true;
    }
}

#[derive(Debug)]
struct Scanner {}

impl Scanner {
    fn new(data: &str) -> Self {
        Scanner {}
    }
    fn scan_tokens(&self) -> Vec<Token> {
        vec![]
    }
}

#[derive(Debug)]
struct Token {}

impl Token {
    fn new(&self) -> Self {
        Token {}
    }
}

fn run(data: &str) -> Result<(), RuntimeError> {
    let scanner = Scanner::new(data);
    let tokens: Vec<Token> = scanner.scan_tokens();
    for token in tokens {
        println!("{:#?}", token);
    }

    Ok(())
}
