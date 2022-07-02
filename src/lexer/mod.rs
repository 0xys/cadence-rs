use self::token::TokenKind;

pub mod token;

pub struct Lexer<'a> {
    input: &'a[u8],
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input: input.as_bytes(), position: 0 }
    }

    pub fn peek(&self) -> Option<u8> {
		if self.position >= self.input.len() {
			return None
		} else {
			return Some(self.input[self.position])
		}
    }

	pub fn read(&mut self) -> Option<u8> {
		if self.position >= self.input.len() {
			return None
		} else {
			let pos = self.position;
			self.position += 1;
			return Some(self.input[pos])
		}
	}

	pub fn tokenize(&mut self) -> Option<token::TokenKind> {
		self.skip_spaces();

		match self.read() {
			None => None,
			Some(c) => {
				match c {
					b'(' => Some(TokenKind::ParenOpen),
					b')' => Some(TokenKind::ParenClose),
					b'[' => Some(TokenKind::BracketOpen),
					b']' => Some(TokenKind::BracketClose),
					b'{' => Some(TokenKind::BraceOpen),
					b'}' => Some(TokenKind::BraceClose),
					b'<' => Some(TokenKind::AngleOpen),
					b'>' => Some(TokenKind::AngleClose),

					b';' => Some(TokenKind::SemiColon),
					b':' => Some(TokenKind::Colon),
					_ => None,
				}
			}
		}
	}

	fn skip_spaces(&mut self) {
		loop {
			if let Some(c) = self.peek() {
				if c == b' ' {
					self.read();
				}else{
					break;
				}
			}else{
				break;
			}
		}
	}

}
