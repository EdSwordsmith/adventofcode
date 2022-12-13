use crate::scanner::Token;

#[derive(Debug)]
pub enum PacketData {
    Int(u64),
    List(Vec<PacketData>),
}

pub fn parse(tokens: Vec<Token>) -> PacketData {
    let mut state = State::new(tokens);
    state.expression()
}

#[derive(Debug)]
struct State {
    tokens: Vec<Token>,
    current: usize,
}

impl State {
    fn new(tokens: Vec<Token>) -> State {
        State { tokens, current: 0 }
    }

    fn expression(&mut self) -> PacketData {
        let mut token = self.peek();

        if token == Token::LeftBracket {
            self.current += 1;
            token = self.peek();

            let mut list = Vec::new();
            while token != Token::RightBracket {
                if token == Token::Comma {
                    self.current += 1;
                }

                list.push(self.expression());
                token = self.peek();
            }

            // Parsed the right bracket token
            self.current += 1;
            PacketData::List(list)
        } else if let Token::Integer(value) = token {
            self.current += 1;
            PacketData::Int(value)
        } else {
            unreachable!("What are you doing bro?")
        }
    }

    fn peek(&self) -> Token {
        self.tokens[self.current]
    }
}
