use super::token::{
    Token, TokenKind
};

pub struct Lexer<'a> {
    input: &'a[u8],
    position: usize,
    commited_position: usize,
    last_char: u8,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.as_bytes(),
            position: 0,
            commited_position: 0,
            last_char: 0,
        }
    }

    /// get next character without consuming
    pub fn peek(&self) -> Option<u8> {
		self.peekn(0)
    }

    pub fn peekn(&self, n: usize) -> Option<u8> {
        if self.position + n >= self.input.len() {
			return None
		} else {
			return Some(self.input[self.position + n])
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

	pub fn tokenize(&mut self) -> Token {
		self.read_spaces();
        self.read_comments();
		self.read_spaces(); // read spaces after comments

		let kind = match self.read() {
			None => TokenKind::EOF,
			Some(c) => {
				match c {
					b'(' => TokenKind::ParenOpen,
					b')' => TokenKind::ParenClose,
					b'[' => TokenKind::BracketOpen,
					b']' => TokenKind::BracketClose,
					b'{' => TokenKind::BraceOpen,
					b'}' => TokenKind::BraceClose,
					b'<' => self.angle_open().unwrap_or(TokenKind::None),
					b'>' => self.angle_close().unwrap_or(TokenKind::None),

					b',' => TokenKind::Comma,
					b'.' => self.period().unwrap_or(TokenKind::None),

					b'+' => TokenKind::Plus,
					b'-' => TokenKind::Minus,
					b'*' => TokenKind::Asterisk,
					b'/' => TokenKind::Slash,
					b'\\' => TokenKind::Backslash,
					b'%' => TokenKind::Percent,
					b'=' => self.equal().unwrap_or(TokenKind::None),

					b'?' => TokenKind::Question,
					b'!' => self.excalmation().unwrap_or(TokenKind::None),

					b'@' => TokenKind::At,

					b'&' => self.and().unwrap_or(TokenKind::None),
					b'|' => self.or().unwrap_or(TokenKind::None),
					b'^' => TokenKind::Xor,

					b'"' => self.double_quote().unwrap_or(TokenKind::None),
					b'\'' => TokenKind::SingleQuote,

					b';' => TokenKind::SemiColon,
					b':' => TokenKind::Colon,
					_ => self.alphanumeric().unwrap_or(TokenKind::None),
				}
			}
		};

        let at = self.commited_position;
        let size = self.position - self.commited_position;

        self.commit();

        Token::new(kind, at, size)
	}

    fn commit(&mut self) {
        self.commited_position = self.position;
    }

	fn read_spaces(&mut self) -> usize {
        let mut count = 0;
        while let Some(c) = self.peek() {
            if c == b' ' {
                self.read();
                count += 1;
            }else{
                break;
            }
        }
        count
	}

    fn read_comments(&mut self) {
        if let Some(c) = self.peek() {
            if c == b'/' {
                if let Some(cc) = self.peekn(1) {
                    if cc == b'/' {
                        self.read();
                        self.read();
                        self.read_till_eol();
                    } else if cc == b'*' {
                        self.read();
                        self.read();
                        self.read_till_blockcomment_close();
                    }
                }
            }
        }
    }

    fn peek_spaces(&mut self, offset: usize) -> usize {
        let mut count = 0;
        while let Some(c) = self.peekn(offset + count) {
            if c == b' ' {
                count += 1;
            }else{
                break;
            }
        }
        count
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

    fn period(&mut self) -> Option<TokenKind> {
        if self.read_spaces() == 0 {
            Some(TokenKind::Period)
        } else {
            None
        }
    }

    fn double_quote(&mut self) -> Option<TokenKind> {
        let mut chars = Vec::new();

        while let Some(c) = self.peek() {
            self.read();

            //  handle escape
            if c == b'\\' {
                if let Some(e) = self.read_escaped_char() {
                    chars.push(e);
                } else {
                    chars.push(b'\\');
                }
                continue;
            }
            chars.push(c);

            if c == b'"' {
                break;
            }
        }

        if chars.len() == 0 {
            return Some(TokenKind::DoubleQuote)
        }

        let str = String::from_utf8(chars[0..chars.len()-1].to_owned()).unwrap();
        Some(TokenKind::String(str))
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

    fn read_till_eol(&mut self) {
        while let Some(c) = self.read() {
            if c == b'\n' {
                break;
            }
        }
    }
    fn read_till_blockcomment_close(&mut self) {
        while let Some(c) = self.read() {
            if c == b'*' {
                if let Some(cc) = self.read() {
                    if cc == b'/' {
                        break;
                    }   
                }
            }
        }
    }


    fn read_numeric_or_dot(&mut self) -> Option<u8> {
        if let Some(c) = self.peek() {
            if Self::is_decimal_digit(c) {
                self.read();
                return Some(c)
            }

            if c == b'.' {
                // to ensure no whitespace after period
                if self.peek_spaces(1) == 0 {
                    self.read();
                    return Some(b'.')
                } else {
                    return None
                }
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

    fn read_escaped_char(&mut self) -> Option<u8> {
        if let Some(e) = self.peek() {
            self.read();
            return match e {
                b'n' => Some(b'\n'),
                b'\\' => Some(b'\\'),
                b'"' => Some(b'"'),
                _ => None,
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
