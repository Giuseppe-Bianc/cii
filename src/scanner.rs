use std::string::String;
use std::collections::HashMap;

fn is_digit(ch: char) -> bool {
	let uch = ch as u8;
	uch >= '0' as u8 && uch <= '9' as u8
}
fn is_aplha(ch: char) -> bool {
	let uch = ch as u8;
	(uch >= 'a' as u8 && uch <= 'z' as u8) || (uch >= 'A' as u8 && uch <= 'Z' as u8) || (ch == '_')
}

fn is_aplha_numeric(ch: char) -> bool {
	is_aplha(ch) || is_digit(ch)
}


#[allow(dead_code)]
#[derive(Debug, Copy, Clone,PartialEq)]
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
	For,
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

fn get_kewords_hashmap() -> HashMap<&'static str, TokenType> {
	HashMap::from([
		("and",And),
		("class", Class),
		("else", Else),
		("false", Fasle),
		("for", For),
		("fun", Fun),
		("if", If),
		("nil", Nil),
		("or",Or),
		("print", Print),
		("return", Return),
		("super", Super),
		("this", This),
		("true", True),
		("var", Var),
		("while", While)
	])
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
			line_number,
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
	pub kewords: HashMap<&'static str,TokenType>
}

impl Scanner {
	pub fn new(source:&str) ->Self{
		Self {
			source: source.to_string(), 
			tokens: vec![],
			start: 0,
			current: 0,
			line: 1,
			kewords: get_kewords_hashmap(),
		}
	}
	pub fn scan_tokens(&mut self) -> Result<Vec<Token>,String> {
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
	pub fn is_at_end(&self) -> bool {
		self.current >= self.source.len()
	}
	pub fn scan_token(&mut self) -> Result<(), String> {
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
			c => {
				if is_digit(c) {
					self.number()?;
				} else if is_aplha(c) {
					self.identifier();
				} else {
					return Err(format!("Unrecognaised char at line{}: {}",self.line, c))
				}
			}
		}
		Ok(())
	}
	
	fn number(&mut self) ->Result<(),String> {
		while is_digit(self.peek()) {
			self.advance();
		}
		if self.peek() == '.' && is_digit(self.peek_next()) {
			self.advance();
			while is_digit(self.peek()) {
				self.advance();
			}
		}
		let substring = &self.source[self.start..self.current];
		let value = substring.parse::<f64>();
		match value {
			Ok(value) => self.add_token_lit(Number, Some(FValue(value))),
			Err(_) => return Err(format!("could not parse number: {}", substring)),
		}
		Ok(())
	}
	fn identifier(&mut self) /*->Result<(),String>*/ {
		while is_aplha_numeric(self.peek()) {
			self.advance();
		}
		let substring = &self.source[self.start..self.current];
		if let Some(&t_type) = self.kewords.get(substring) {
			self.add_token(t_type);
		} else {
			self.add_token(Identifier);
		}
		//Ok(())
	}
	
	fn string(&mut self) ->Result<(),String> {
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
	
	fn char_match(&mut self, ch: char) -> bool {
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

	fn advance(&mut self) -> char {
		let c = self.source.chars().nth(self.current).unwrap();
		self.current += 1;

		c
	}
	
	fn add_token(&mut self, token_type: TokenType) {
		self.add_token_lit(token_type, None);
	}

	fn add_token_lit(&mut self, token_type: TokenType, literal: Option<LiteralValue>) {
		let text: String = self.source[self.start..self.current].to_string();
		self.tokens.push(Token { 
			token_type: token_type, 
			lexeme: text, 
			literal: literal, 
			line_number: self.line 
		});
	}
	
	fn peek(&self) -> char {
		if self.is_at_end() {
			return '\0';
		}
		self.source.chars().nth(self.current).unwrap()
	}
	
	fn peek_next(&self) -> char {
		if self.current+1 >= self.source.len() {
			return '\0';
		}
		self.source.chars().nth(self.current + 1).unwrap()
	}
	
	
	
}
