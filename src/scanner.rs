#[derive(Debug)]
pub enum Token {
    // meta tokens
    Illegal(String),
    Eof,

    // identifiers & literals
    Ident(String),
    String(String),
    Number(String),
    
    // value keywords
    True,
    False,
    Nil,

    // normal keywords
    Let,
    If,
    Else,

    // operator keywords
    Be,
    Do,
    End,
    Into,
    With,
}

pub struct Scanner {
    start: usize,
    current: usize,
    data: Box<[char]>,
}

impl Scanner {
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;

        match self.advance() {
            '\0' => Token::Eof,
            _ => Token::Illegal(String::from("unrecognized character"))
        }
    }

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

impl Iterator for Scanner {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
    }
}
