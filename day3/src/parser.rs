use std::{fmt::{Debug, Formatter}, str::Chars};

#[derive(Debug)]
enum State {
    Begin,
    Mul(usize),
    LParen,
    Num1(usize),
    Comma,
    Num2(usize),
    RParen,
    End
}

static MULTIPLY: &'static str = "mul";
impl State {
    fn is_valid(&self, character: Option<char>) -> bool {
        match self {
            State::Begin => false,
            State::Mul(i) => {
                character == MULTIPLY.chars().nth(*i)
            },
            State::LParen => character == Some('('),
            State::Num1(_) => character.is_some() && character.unwrap().is_numeric(),
            State::Comma => character == Some(','),
            State::Num2(_) => character.is_some() && character.unwrap().is_numeric(),
            State::RParen => character == Some(')'),
            State::End => false
        }
    }

    /// Advances the state. Returns true if the parser shoud move to the next character.
    pub fn next_state(&mut self, current_character: Option<char>) -> bool {
        let valid = self.is_valid(current_character);
        let (next_state, advance) = match (&self, valid) {
            (State::Begin, _) => (State::Mul(0), false),
            (State::Mul(i), true) => {
                if *i >= MULTIPLY.len() - 1 {
                    (State::LParen, true)
                } else {
                    (State::Mul(*i + 1), true)
                }
            },
            (State::Mul(i), false) => {
                if *i == 0 {
                    (State::Begin, true)
                } else {
                    (State::Begin, false)
                }
            },
            (State::LParen, true) => (State::Num1(0), true),
            (State::LParen, false) => (State::Begin, false),
            (State::Num1(i), true) => {
                if *i >= 2 {
                    (State::Comma, true)
                } else {
                    (State::Num1(*i + 1), true)
                }
            },
            (State::Num1(i), false) => {
                if *i > 0 {
                    (State::Comma, false)
                } else {
                    (State::Begin, false)
                }
            }
            (State::Comma, true) => (State::Num2(0), true),
            (State::Num2(i), true) =>  {
                if *i >= 2 {
                    (State::RParen, true)
                } else {
                    (State::Num2(*i + 1), true)
                }
            },
            (State::Num2(i), false) => {
                if *i > 0 {
                    (State::RParen, false)
                } else {
                    (State::Begin, false)
                }
            }
            (State::RParen, true) => (State::End, false),
            (State::End, _) => (State::Begin, true),
            (_, false) => (State::Begin, false)
        };

        *self = next_state;
        
        advance
    }

    pub fn is_same(a: &State, b: &State) -> bool {
        std::mem::discriminant(a) == std::mem::discriminant(b)
    }
}

pub struct Parser<'a> {
    iter: Chars<'a>,
    current_char: Option<char>,
    state: State,
    last_state: State
}

impl<'a> Debug for Parser<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Parser").field("current_char", &self.current_char).field("state", &self.state).finish()
    }
}

impl<'a> Parser<'a> {
    pub fn new(string: &'a str) -> Parser<'a> {
        let mut iter = string.chars();
        let first = iter.next();
        Parser { iter, current_char: first , state: State::Begin, last_state: State::Begin }
    }

    fn skip_whitespace(&mut self) {
        while self.current_char.is_some() && self.current_char.unwrap().is_whitespace() {
            let next = self.iter.next();

            self.current_char = next;
        }
    }

    fn advance(&mut self) {
        if self.state.next_state(self.current_char) {
            self.current_char = self.iter.next();
        }

        if !State::is_same(&self.last_state, &self.state) {
            self.skip_whitespace();
        }

        // println!("{:?}", self);
    }

    pub fn parse(&mut self) -> Vec<(u32, u32)> {
        let mut vec = Vec::new();

        let mut num1 = String::new();
        let mut num2 = String::new();
        while self.current_char.is_some() {
            match self.state {
                State::Begin => {
                    num1.clear();
                    num2.clear();
                }
                State::Num1(_) => if self.state.is_valid(self.current_char) {
                    num1.push(self.current_char.unwrap());
                },
                State::Num2(_) => if self.state.is_valid(self.current_char) {
                    num2.push(self.current_char.unwrap());
                },
                State::End => {
                    vec.push((num1.parse().unwrap(), num2.parse().unwrap()));
                }
                _ => ()
            }

            self.advance();
        }

        // println!();

        vec
    }
}