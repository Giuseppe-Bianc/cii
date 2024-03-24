use std::string::String;

#[allow(dead_code)]
#[derive(Debug, Clone,PartialEq)]
pub enum TokenType {
	LeftParen,
	RigthParen,
	LeftBrace,
	RigthBrace,
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
	StringLit, 
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

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum LiteralValue {
    IntValue(i64),
    FValue(f64), 
    StringValue(String),
    Identifiervalue(String),
}

#[derive(Debug, Clone)]
pub struct Token {
	pub token_type: TokenType,
	pub lexeme: String,
	pub literal: Option<LiteralValue>,
	pub line_number: usize,
}
use LiteralValue::*;

#[allow(dead_code)]
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
	pub source: String,
	pub tokens: Vec<Token>,
	pub start: usize,
	pub current: usize,
	pub line: usize,
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
			for error in errors{
				joined.push_str(&error);
				joined.push_str("\n");
			}
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
			'(' => self.add_token(LeftParen),
			')' => self.add_token(RigthParen),
			'{' => self.add_token(LeftBrace),
			'}' => self.add_token(RigthBrace),
			',' => self.add_token(Comma),
			'.' => self.add_token(Dot),
			'-' => self.add_token(Minus),
			'+' => self.add_token(Plus), 
			';' => self.add_token(SemiColon),
			'*' => self.add_token(Star),
			'!' => {
				let token = if self.char_match('=') {
					BangEqual
				} else {
					Bang
				};
				self.add_token(token)
			}
			'=' => {
				let token = if self.char_match('=') {
					EqualEqual
				} else {
					Equal
				};
				self.add_token(token)
			}
			'<' => {
				let token = if self.char_match('=') {
					LessEqual
				} else {
					Less
				};
				self.add_token(token)
			}
			'>' => {
				let token = if self.char_match('=') {
					GreaterEqual
				} else {
					Greater
				};
				self.add_token(token)
			}
			'/' => {
				if self.char_match('/') {
					loop {
						if self.peek() == '\n' || self.is_at_end() {
							break;
						}
						self.advance();
					}
					
				} else {
					self.add_token(Slash)
				}
			}
			' '|'\r'|'\t' => (),
			'\n' => self.line += 1,
			'"' => self.string()?,
			_ => return Err(format!("Unrecognaised char at line{}: {}",self.line, c)),
		}
		Ok(())
	}

	fn string(self: &mut Self) ->Result<(),String> {
		while self.peek() != '"' && !self.is_at_end() {
			if self.peek() == '\n'{
				self.line += 1;
			}
			self.advance();
		}
		if self.is_at_end() {
			return Err("unterimanted string".to_string());
		}
		self.advance();
		let value = self.source[self.start+1..self.current-1].to_string();
		self.add_token_lit(StringLit, Some(StringValue(value)));
		Ok(())
	}
	
	fn char_match(self: &mut Self, ch: char) -> bool {
		if self.is_at_end() {
			return false;
		}
		if self.source.chars().nth(self.current).unwrap() as char != ch {
			return false;
		} else {
			self.current += 1;
			return true;
		}
	}

	fn advance(self: &mut Self) -> char {
		let c = self.source.chars().nth(self.current).unwrap();
		self.current += 1;

		c
	}
	
	fn add_token(self: &mut Self, token_type: TokenType) {
		self.add_token_lit(token_type, None);
	}

	fn add_token_lit(self: &mut Self, token_type: TokenType, literal: Option<LiteralValue>) {
		let text: String = self.source[self.start..self.current].to_string();
		self.tokens.push(Token { 
			token_type: token_type, 
			lexeme: text, 
			literal: literal, 
			line_number: self.line 
		});
	}
	
	fn peek(self: &Self) -> char {
		if self.is_at_end() {
			return '\0';
		}
		self.source.chars().nth(self.current).unwrap()
	}
	
}