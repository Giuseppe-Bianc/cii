mod scanner;
mod expr;

use std::env;
use std::fs;
use std::io;
use std::io::BufRead;
use std::process::exit;

use scanner::Scanner;
fn run_prompt() -> Result<(), String> {
    loop {
        print!("> ");
        let stdin = io::stdin();
        let mut buffer = String::new();
        match stdin.lock().read_line(&mut buffer) {
            Ok(n) => {
                if n <= 1 {
                    return Ok(());
                }
            }
            Err(_) => return Err("Error reading input: {}".to_string()),
        }
        println!("ECHO:{}", buffer);
        match run(&buffer) {
            Ok(_) => todo!(),
            Err(msg) => println!("{}", msg),
        }
    }
}

fn run_file(path: &str) -> Result<(), String> {
    match fs::read_to_string(path) {
        Err(msg) => return Err(msg.to_string()),
        Ok(contents) => return run(&contents),
    }
}

fn run(contents: &str) -> Result<(), String> {
    let mut scanner = Scanner::new(contents);
    let tokens = scanner.scan_tokens()?;
    for token in tokens {
        println!("{:?}", token)
    }
    return Ok(());
}
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage  jlox [script]");
        exit(64);
    } else if args.len() == 2 {
        match run_file(&args[1]) {
            Ok(_) => exit(0),
            Err(msg) => {
                println!("EROR:\n{}", msg);
                exit(1);
            }
        }
    } else {
        match run_prompt() {
            Ok(_) => exit(0),
            Err(msg) => {
                println!("EROR:\n{}", msg);
                exit(1);
            }
        }
    }
}