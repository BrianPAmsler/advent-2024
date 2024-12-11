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
    pub fn is_valid(&self, character: char) -> bool {
        match self {
            State::Begin => false,
            State::Mul(i) => {
                Some(character) == MULTIPLY.chars().nth(*i)
            },
            State::LParen => character == '(',
            State::Num1(_) => character.is_numeric(),
            State::Comma => character == ',',
            State::Num2(_) => character.is_numeric(),
            State::RParen => character == ')',
            State::End => false
        }
    }

    pub fn next_state(&self) -> State {
        match self {
            State::Begin => State::Mul(0),
            State::Mul(i) => {
                if *i >= MULTIPLY.len() - 1 {
                    State::LParen
                } else {
                    State::Mul(i + 1)
                }
            },
            State::LParen => State::Num1(0),
            State::Num1(i) => {
                if *i >= 2 {
                    State::Comma
                } else {
                    State::Num1(i + 1)
                }
            },
            State::Comma => State::Num2(0),
            State::Num2(i) =>  {
                if *i >= 2 {
                    State::RParen
                } else {
                    State::Num2(i + 1)
                }
            },
            State::RParen => State::End,
            State::End => State::End
        }
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
        while self.has_next() && self.current_char.unwrap().is_whitespace() {
            let next = self.iter.next();

            self.current_char = next;
        }
    }

    fn next_state(&mut self) {
        std::mem::swap(&mut self.state, &mut self.last_state);

        self.state = self.last_state.next_state();
    }

    fn advance(&mut self) {
        if !matches!(self.state, State::Begin) {
            self.current_char = self.iter.next();
        }
        
        self.next_state();

        if !State::is_same(&self.last_state, &self.state) {
            self.skip_whitespace();
        }
    }

    pub fn parse_next(&mut self) -> Option<(u32, u32)> {
        self.advance();
        let mut num1 = String::new();
        let mut num2 = String::new();
        while self.has_next() && self.state.is_valid(self.current_char.unwrap()) {
            match self.state {
                State::Num1(_) => {
                    num1.push(self.current_char.unwrap());
                },
                State::Num2(_) => {
                    num2.push(self.current_char.unwrap());
                },
                _ => ()
            }

            self.advance();

            // Keep advancing an invalid number state (this will happen if a number is less than 3 digits)
            if self.has_next() && !self.state.is_valid(self.current_char.unwrap()) {
                match self.state {
                    State::Num1(1..=2) | State::Num2(1..=2) => {
                        while matches!(self.state, State::Num1(_) | State::Num2(_)) {
                            self.next_state();
                        }
                    },
                    _ => ()
                };
            }
        }

        let out = match &self.state {
            State::End => {
                Some((num1.parse().unwrap(), num2.parse().unwrap()))
            },
            _ => {
                None
            }
        };

        if matches!(self.state, State::Mul(0)) {
            self.current_char = self.iter.next();
        }

        if self.current_char.is_some() {
            self.state = State::Begin;
        }

        out
    }

    pub fn has_next(&self) -> bool {
        matches!(self.state, State::Begin) || self.current_char.is_some()
    }
}