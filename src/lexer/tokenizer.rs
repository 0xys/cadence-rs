use super::token::TokenKind;

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

	pub fn tokenize(&mut self) -> Option<TokenKind> {
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
					b'<' => self.angle_open(),
					b'>' => self.angle_close(),

					b',' => Some(TokenKind::Comma),
					b'.' => Some(TokenKind::Period),

					b'+' => Some(TokenKind::Plus),
					b'-' => Some(TokenKind::Minus),
					b'*' => Some(TokenKind::Asterisk),
					b'/' => self.slash(),
					b'\\' => Some(TokenKind::ReverSlash),
					b'%' => Some(TokenKind::Percent),
					b'=' => self.equal(),

					b'?' => Some(TokenKind::Question),
					b'!' => self.excalmation(),

					b'@' => Some(TokenKind::At),

					b'&' => self.and(),
					b'|' => self.or(),
					b'^' => Some(TokenKind::Xor),

					b'"' => Some(TokenKind::DoubleQuote),
					b'\'' => Some(TokenKind::SingleQuote),

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

    fn slash(&mut self) -> Option<TokenKind> {
        if let Some(c) = self.peek() {
            match c {
                b'/' => {
                    self.skip_till_eol();
                    self.tokenize()
                },
                _ => Some(TokenKind::Slash),
            }
        }else{
            Some(TokenKind::Slash)
        }
    }

    fn excalmation(&mut self) -> Option<TokenKind> {
        if let Some(c) = self.peek() {
            match c {
                b'=' => {
                    self.read();
                    Some(TokenKind::NotEqual)
                },
                _ => Some(TokenKind::Exclamation),
            }
        }else{
            Some(TokenKind::Exclamation)
        }
    }

    fn angle_open(&mut self) -> Option<TokenKind> {
        if let Some(c) = self.peek() {
            match c {
                b'=' => {
                    self.read();
                    Some(TokenKind::LessThanOrEqual)
                },
                b'<' => {
                    self.read();
                    Some(TokenKind::BitwiseShiftLeft)
                },
                _ => Some(TokenKind::AngleOpen)
            }
        }else{
            Some(TokenKind::AngleOpen)
        }
    }

    fn angle_close(&mut self) -> Option<TokenKind> {
        if let Some(c) = self.peek() {
            match c {
                b'=' => {
                    self.read();
                    Some(TokenKind::GreaterThanOrEqual)
                },
                b'>' => {
                    self.read();
                    Some(TokenKind::BitwiseShiftRight)
                },
                _ => Some(TokenKind::AngleClose)
            }
        }else{
            Some(TokenKind::AngleClose)
        }
    }

    fn equal(&mut self) -> Option<TokenKind> {
        if let Some(c) = self.peek() {
            match c {
                b'=' => {
                    self.read();
                    Some(TokenKind::EqualTo)
                },
                _ => Some(TokenKind::Assign)
            }
        }else{
            Some(TokenKind::Assign)
        }
    }

    fn and(&mut self) -> Option<TokenKind> {
        if let Some(c) = self.peek() {
            match c {
                b'&' => {
                    self.read();
                    Some(TokenKind::LogicalConjunction)
                },
                _ => Some(TokenKind::BitwiseAnd)
            }
        }else{
            Some(TokenKind::BitwiseAnd)
        }
    }

    fn or(&mut self) -> Option<TokenKind> {
        if let Some(c) = self.peek() {
            match c {
                b'|' => {
                    self.read();
                    Some(TokenKind::LogicalDisjunction)
                },
                _ => Some(TokenKind::BitwiseOr)
            }
        }else{
            Some(TokenKind::BitwiseOr)
        }
    }

    fn skip_till_eol(&mut self) {
        loop {
            if let Some(c) = self.read() {
                if c == b'\n' {
                    break;
                }
            }else{
                break;
            }
        }
    }

}
