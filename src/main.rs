mod scanner;

use std::env;
use std::fs;
use std::io;
use std::io::BufRead;
use std::process::exit;

use scanner::Scanner;
fn run_prompt()  -> Result<(), String>{
    loop {
        print!("> ");
        let stdin = io::stdin();
        let mut buffer = String::new();
        match stdin.lock().read_line(&mut buffer) {
            Ok(n) => {
                if n <= 1 {
                    return Ok(());
                } 
            },
            Err(_) => return Err("Error reading input: {}".to_string()),
        }
        println!("ECHO:{}", buffer);
        match run(&buffer) {
            Ok(_) => todo!(),
            Err(msg) => println!("{}",msg),
        }
    }
}

fn run_file(path: &str) -> Result<(), String> {
    match fs::read_to_string(path){
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
        match run_file(&args[1]){
            Ok(_) => exit(0),
            Err(msg) =>{
                println!("EROR:\n{}", msg);
                exit(1);
            }
        }
    } else {
        match run_prompt() {
            Ok(_) => exit(0),
            Err(msg) =>{
                println!("EROR:\n{}", msg);
                exit(1);
            }
        }
    }
}

#[cfg(test)]
mod  tests {
    use crate::scanner::TokenType::*;
    use crate::scanner::LiteralValue::*;

    use super::*;

    #[test]
    fn handle_one_char_tokens(){
        let source = "(( )) }{ ";
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens().unwrap();
        assert_eq!(scanner.tokens.len(), 7);
        assert_eq!(scanner.tokens[0].token_type, LeftParen);
        assert_eq!(scanner.tokens[1].token_type, LeftParen);
        assert_eq!(scanner.tokens[2].token_type, RigthParen);
        assert_eq!(scanner.tokens[3].token_type, RigthParen);
        assert_eq!(scanner.tokens[4].token_type, RigthBrace);
        assert_eq!(scanner.tokens[5].token_type, LeftBrace);
        assert_eq!(scanner.tokens[6].token_type, Eof);
    }

    #[test]
    fn handle_two_char_tokens(){
        let source = "! != == >=";
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens().unwrap();
        assert_eq!(scanner.tokens.len(), 5);
        assert_eq!(scanner.tokens[0].token_type, Bang);
        assert_eq!(scanner.tokens[1].token_type, BangEqual);
        assert_eq!(scanner.tokens[2].token_type, EqualEqual);
        assert_eq!(scanner.tokens[3].token_type, GreaterEqual);
        assert_eq!(scanner.tokens[4].token_type, Eof);
    }

    #[test]
    fn handle_string_lit(){
        let source = r#""ABC""#;
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens().unwrap();
        assert_eq!(scanner.tokens.len(), 2);
        assert_eq!(scanner.tokens[0].token_type, StringLit);
        match scanner.tokens[0].literal.as_ref().unwrap() {
            StringValue(val) => assert_eq!(val, "ABC"),
            _ => panic!("incorect literal type"),
        }
        assert_eq!(scanner.tokens[1].token_type, Eof);
    }

    #[test]
    fn handle_string_multiline(){
        let source = "\"ABC\nEF\"";
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens().unwrap();
        assert_eq!(scanner.tokens.len(), 2);
        assert_eq!(scanner.tokens[0].token_type, StringLit);
        match scanner.tokens[0].literal.as_ref().unwrap() {
            StringValue(val) => assert_eq!(val, "ABC\nEF"),
            _ => panic!("incorect literal type"),
        }
        assert_eq!(scanner.tokens[1].token_type, Eof);
    }

    #[test]
    fn handle_string_lit_unterminated(){
        let source = r#""ABC"#;
        let mut scanner = Scanner::new(source);
        let scan_tokens = scanner.scan_tokens();
        match scan_tokens {
            Ok(_) => panic!("shoud have failed"),
            Err(_) => (),
        }
        
    }


    #[test]
    fn handle_number_lit(){
        let source = "123.123\n321.0\n5";
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens().unwrap();
        assert_eq!(scanner.tokens.len(), 4);
        for i in 0..3 {
            assert_eq!(scanner.tokens[i].token_type, Number);
        }
        match scanner.tokens[0].literal {
            Some(FValue(val)) => assert_eq!(val, 123.123),
            _ => {
                panic!("incorect literal type")
            },
        }
        match scanner.tokens[1].literal {
            Some(FValue(val)) => assert_eq!(val, 321.0),
            _ => {
                panic!("incorect literal type")
            },
        }
        match scanner.tokens[2].literal {
            Some(FValue(val)) => assert_eq!(val, 5.),
            _ => {
                panic!("incorect literal type")
            },
        }
        assert_eq!(scanner.tokens[3].token_type, Eof);
    }
}
