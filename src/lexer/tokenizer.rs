use super::token::TokenKind;

pub struct Lexer<'a> {
    input: &'a[u8],
    position: usize,
    last_char: u8,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.as_bytes(),
            position: 0,
            last_char: 0,
        }
    }

    /// get next character without consuming
    pub fn peek(&self) -> Option<u8> {
		if self.position >= self.input.len() {
			return None
		} else {
			return Some(self.input[self.position])
		}
    }

    /// get and consume next character
	pub fn read(&mut self) -> Option<u8> {
		if self.position >= self.input.len() {
            self.last_char = 0;
			return None
		} else {
			let pos = self.position;
			self.position += 1;
            self.last_char = self.input[pos];
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
					b'\\' => Some(TokenKind::Backslash),
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
					_ => self.alphanumeric(),
				}
			}
		}
	}

	fn skip_spaces(&mut self) {
        while let Some(c) = self.peek() {
            if c == b' ' {
                self.read();
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

    fn alphanumeric(&mut self) -> Option<TokenKind> {
        if let Some(letters) = self.read_chars(self.last_char) {
            let identifier = String::from_utf8(letters).unwrap();
            return Some(TokenKind::Identifier(identifier))
        }
        None
    }

    fn read_chars(&mut self, begin_char: u8) -> Option<Vec<u8>> {
        let mut chars = Vec::new();
        chars.push(begin_char);

        if Self::is_letter(begin_char) {
            while let Some(c) = self.read_letter() {
                chars.push(c);
            }
            return Some(chars)
        }
        
        if begin_char == b'0' {
            if let Some(c) = self.peek() {
                match c {
                    b'b' => {
                        self.read();
                        chars.push(b'b');
                        while let Some(b) = self.read_bits() {
                            chars.push(b);
                        }
                        return Some(chars)
                    },
                    b'x' => {
                        self.read();
                        chars.push(b'x');
                        while let Some(n) = self.read_nibles() {
                            chars.push(n);
                        }
                        return Some(chars)
                    },
                    _ => (),
                }
            }
        }

        if Self::is_decimal_digit(begin_char) {
            while let Some(n) = self.read_numeric_or_dot() {
                chars.push(n);
            }
            return Some(chars)
        }

        None
    }

    fn skip_till_eol(&mut self) {
        while let Some(c) = self.read() {
            if c == b'\n' {
                break;
            }
        }
    }

    fn read_numeric_or_dot(&mut self) -> Option<u8> {
        if let Some(c) = self.peek() {
            if Self::is_decimal_digit(c) || c == b'.' {
                self.read();
                return Some(c)
            }
        }
        None
    }

    fn read_letter(&mut self) -> Option<u8> {
        if let Some(c) = self.peek() {
            if Self::is_letter(c) || Self::is_decimal_digit(c) {
                self.read();
                return Some(c)
            }
        }
        None
    }

    fn read_bits(&mut self) -> Option<u8> {
        if let Some(c) = self.peek() {
            if c == b'1' || c == b'0' {
                self.read();
                return Some(c)
            }
        }
        None
    }

    fn read_nibles(&mut self) -> Option<u8> {
        if let Some(c) = self.peek() {
            if Self::is_nible(c) {
                self.read();
                return Some(c)
            }
        }
        None
    }

    fn is_letter(c: u8) -> bool {
        c.is_ascii_alphabetic() || c == b'_'
    }

    fn is_decimal_digit(c: u8) -> bool {
        c >= b'0' && c <= b'9'
    }

    fn is_nible(c: u8) -> bool {
        (c >= b'0' && c <= b'9') || (c >= b'a' && c <= b'f') || (c >= b'A' && c <= b'F')
    }

}
