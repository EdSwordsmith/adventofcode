#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Token {
    LeftBracket,
    RightBracket,
    Integer(u64),
    Comma,
}

pub fn scan_tokens(source: String) -> Vec<Token> {
    let mut state = State::new(source.trim().to_owned());

    while !state.is_at_end() {
        state.start = state.current;
        state.scan_token();
    }

    state.tokens
}

struct State {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
}

impl State {
    fn new(source: String) -> State {
        State {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        let token = match c {
            '[' => Token::LeftBracket,
            ']' => Token::RightBracket,
            ',' => Token::Comma,
            _ => self.integer(),
        };
        self.tokens.push(token);
    }

    fn integer(&mut self) -> Token {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        let string = &self.source[self.start..self.current];
        let value = string.parse().unwrap();
        Token::Integer(value)
    }

    fn advance(&mut self) -> char {
        let c = self.get_current_char();
        self.current += c.len_utf8();
        c
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.get_current_char()
    }

    fn get_current_char(&self) -> char {
        self.get_char(self.current)
    }

    fn get_char(&self, index: usize) -> char {
        (&self.source[index..]).chars().next().unwrap()
    }
}
