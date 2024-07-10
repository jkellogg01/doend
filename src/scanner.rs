pub enum Token {
    Illegal(String),
    Ident(String),
    Literal(String),

    Let,
    Be,
    Do,
    End,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            _ => write!(f, "Nothing here"),
        }
    }
}

pub struct Scanner {
    start: usize,
    current: usize,
    data: Vec<char>,
}

impl Iterator for Scanner {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        self.skip_whitespace();
        self.start = self.current;

        match self.advance() {
            '\0' => None,
            _ => Some(Token::Illegal(String::from("unrecognized character")))
        }
    }
}

impl Scanner {
    fn advance(&mut self) -> char {
        let val = self.peek();
        self.current += 1;
        val
    }

    fn peek(&self) -> char {
        if self.current >= self.data.len() {
            '\0'
        } else {
            self.data[self.current]
        }
    }

    fn skip_whitespace(&mut self) {
        while self.peek().is_ascii_whitespace() {
            self.advance();
        }
    }
}
