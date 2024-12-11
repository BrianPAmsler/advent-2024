use std::{fmt::{Debug, Formatter}, marker::PhantomData, str::Chars};

pub trait State: Debug + Sized + Clone {
    fn is_valid(&self, current_character: Option<char>) -> bool;
    /// Advances the state. Returns true if the parser shoud move to the next character.
    fn next_state(&mut self, current_character: Option<char>) -> bool;

    fn is_same(a: &Self, b: &Self) -> bool {
        std::mem::discriminant(a) == std::mem::discriminant(b)
    }

    fn initial_state() -> Self;
}

pub trait StateProcessor<S: State, T>: Default {
    fn process_state(&mut self, state: &S, current_character: Option<char>);
    fn finish(self) -> T;
}

#[derive(Debug, Clone, Copy)]
pub enum Part1State {
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
impl State for Part1State {
    fn is_valid(&self, character: Option<char>) -> bool {
        match self {
            Part1State::Begin => false,
            Part1State::Mul(i) => {
                character == MULTIPLY.chars().nth(*i)
            },
            Part1State::LParen => character == Some('('),
            Part1State::Num1(_) => character.is_some() && character.unwrap().is_numeric(),
            Part1State::Comma => character == Some(','),
            Part1State::Num2(_) => character.is_some() && character.unwrap().is_numeric(),
            Part1State::RParen => character == Some(')'),
            Part1State::End => false
        }
    }

    fn next_state(&mut self, current_character: Option<char>) -> bool {
        let valid = self.is_valid(current_character);
        let (next_state, advance) = match (&self, valid) {
            (Part1State::Begin, _) => (Part1State::Mul(0), false),
            (Part1State::Mul(i), true) => {
                if *i >= MULTIPLY.len() - 1 {
                    (Part1State::LParen, true)
                } else {
                    (Part1State::Mul(*i + 1), true)
                }
            },
            (Part1State::Mul(i), false) => {
                if *i == 0 {
                    (Part1State::Begin, true)
                } else {
                    (Part1State::Begin, false)
                }
            },
            (Part1State::LParen, true) => (Part1State::Num1(0), true),
            (Part1State::LParen, false) => (Part1State::Begin, false),
            (Part1State::Num1(i), true) => {
                if *i >= 2 {
                    (Part1State::Comma, true)
                } else {
                    (Part1State::Num1(*i + 1), true)
                }
            },
            (Part1State::Num1(i), false) => {
                if *i > 0 {
                    (Part1State::Comma, false)
                } else {
                    (Part1State::Begin, false)
                }
            }
            (Part1State::Comma, true) => (Part1State::Num2(0), true),
            (Part1State::Num2(i), true) =>  {
                if *i >= 2 {
                    (Part1State::RParen, true)
                } else {
                    (Part1State::Num2(*i + 1), true)
                }
            },
            (Part1State::Num2(i), false) => {
                if *i > 0 {
                    (Part1State::RParen, false)
                } else {
                    (Part1State::Begin, false)
                }
            }
            (Part1State::RParen, true) => (Part1State::End, false),
            (Part1State::End, _) => (Part1State::Begin, true),
            (_, false) => (Part1State::Begin, false)
        };

        *self = next_state;
        
        advance
    }

    fn initial_state() -> Self {
        Self::Begin
    }
}

pub struct Parser<'a, P: StateProcessor<S, T>, T, S: State,> {
    iter: Chars<'a>,
    current_char: Option<char>,
    state: S,
    last_state: S,
    processor: P,
    _pd: PhantomData<T>
}

impl<'a, T, S: State, P: StateProcessor<S,T>> Debug for Parser<'a, P, T, S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Parser").field("current_char", &self.current_char).field("state", &self.state).finish()
    }
}

impl<'a, T, S: State, P: StateProcessor<S, T>> Parser<'a, P, T, S> {
    pub fn new(string: &'a str) -> Parser<'a, P, T, S> {
        let mut iter = string.chars();
        let first = iter.next();
        Parser { iter, current_char: first , state: S::initial_state(), last_state: S::initial_state(), processor: P::default(), _pd: PhantomData }
    }

    fn skip_whitespace(&mut self) {
        while self.current_char.is_some() && self.current_char.unwrap().is_whitespace() {
            let next = self.iter.next();

            self.current_char = next;
        }
    }

    fn advance(&mut self) {
        self.last_state = self.state.clone();
        if self.state.next_state(self.current_char) {
            self.current_char = self.iter.next();
        }

        if !S::is_same(&self.last_state, &self.state) {
            self.skip_whitespace();
        }

        // println!("{:?}", self);
    }

    pub fn parse(mut self) -> T {
        while self.current_char.is_some() {
            self.processor.process_state(&self.state, self.current_char);

            self.advance();
        }

        // println!();

        self.processor.finish()
    }
}

#[derive(Default)]
pub struct Part1Processor {
    num1: String,
    num2: String,
    vec: Vec<(u64, u64)>
}

impl StateProcessor<Part1State, Vec<(u64, u64)>> for Part1Processor {
    fn process_state(&mut self, state: &Part1State, current_character: Option<char>) {
        match state {
            Part1State::Begin => {
                self.num1.clear();
                self.num2.clear();
            }
            Part1State::Num1(_) => if state.is_valid(current_character) {
                self.num1.push(current_character.unwrap());
            },
            Part1State::Num2(_) => if state.is_valid(current_character) {
                self.num2.push(current_character.unwrap());
            },
            Part1State::End => {
                self.vec.push((self.num1.parse().unwrap(), self.num2.parse().unwrap()));
            }
            _ => ()
        }
    }

    fn finish(self) -> Vec<(u64, u64)> {
        self.vec
    }
}