use std::{string::String};

#[derive(Debug, Clone)]
pub enum TokenType {
	LeftParen,
	RigthParen,
	LeftBrace,
	RIGTHBrace,
	Comma, 
	Dot, 
	Minus, 
	SemiColon, 
	Plus,
	Slash, 
	Star,

	Bang, 
	BangEqual,
	Equal, 
	EqualEqual,
	Greater,
	GreaterEqual,
	Less, 
	LessEqual,

	Identifier, 
	String, 
	Number,
	 
	And, 
	Class, 
	Else, 
	Fasle,
	Fun,
	If, 
	Nil, 
	Or, 
	Print,
	Return, 
	Super, 
	This, 
	True, 
	Var, 
	While,

	Eof,
}
use TokenType::*;

impl std::fmt::Display for TokenType {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{:?}", self)
	}
}

#[derive(Debug, Clone)]
pub enum LiteralValue {
	IntValue(i64),
	FValue(f64), 
	StringValue(String),
	Identifiervalue(String),
}



#[derive(Debug, Clone)]
pub struct Token {
	token_type: TokenType,
	lexeme: String,
	literal: Option<LiteralValue>,
	line_number: usize,
}

impl Token {
	pub fn new(token_type: TokenType,lexeme: String, literal: Option<LiteralValue>, line_number: usize) -> Self {
		Self { 
			token_type, 
			lexeme, 
			literal, 
			line_number
		}
	}
	pub fn to_string(self: &Self) -> String {
		format!("{} {} {:?}", self.token_type, self.lexeme, self.literal)
	}
}

pub struct Scanner {
	source: String,
	tokens: Vec<Token>,
	start: usize,
	current: usize,
	line: usize,
}

impl Scanner {
	pub fn new(source:&str) ->Self{
		Self {
			source: source.to_string(), 
			tokens: vec![],
			start: 0,
			current: 0,
			line: 1,
		}
	}
	pub fn scan_tokens(self: &mut Self) -> Result<Vec<Token>,String> {
		let mut errors = vec![];
		while !self.is_at_end() {
			self.start = self.current;
			match self.scan_token() {
				Ok(_) => (),
				Err(msg) => errors.push(msg),
			}
		}
		self.tokens.push(Token{
			token_type: TokenType::Eof, 
			lexeme: "".to_string(), 
			literal: None, 
			line_number: self.line,
		});
		if errors.len() > 0 {
			let mut joined = "".to_string();
			errors.iter().for_each(|msg| {
				joined.push_str(&msg);
				joined.push_str("\n");
			});
			return Err(joined);
		}
		Ok(self.tokens.clone())
	}
	pub fn is_at_end(self: &Self) -> bool {
		self.current >= self.source.len()
	}
	pub fn scan_token(self: &mut Self) -> Result<(), String> {
		let c= self.advance();

		match c {
			'(' => Ok(self.add_token(LeftParen)),
			')' => Ok(self.add_token(RigthParen)),
			'{' => Ok(self.add_token(LeftBrace)),
			'}' => Ok(self.add_token(RigthParen)),
			',' => Ok(self.add_token(Comma)),
			'.' => Ok(self.add_token(Dot)),
			'-' => Ok(self.add_token(Minus)),
			'+' => Ok(self.add_token(Plus)), 
			';' => Ok(self.add_token(SemiColon)),
			'*' => Ok(self.add_token(Star)),
			 _ => return Err(format!("Unrecognaised char at line{}: {}",self.line, c)),
		}
	}

	fn advance(self: &mut Self) -> char {
		let c = self.source.as_bytes()[self.current];
		self.current += 1;

		c as char
	}
	
	fn add_token(self: &mut Self, token_type: TokenType) {
		self.add_token_lit(token_type, None);
	}
	fn add_token_lit(self: &mut Self, token_type: TokenType, literal: Option<LiteralValue>) {
		let mut text = "".to_string();
		let bytes = self.source.as_bytes();
		for i in self.start..self.current  {
			text.push(bytes[i] as char);
		}
		self.tokens.push(Token { 
			token_type: token_type, 
			lexeme: text, 
			literal: literal, 
			line_number: self.line 
		});
	}
}