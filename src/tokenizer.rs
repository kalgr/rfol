extern crate itertools;

use crate::language::*;
use itertools::Itertools;

#[derive(Debug)]
pub struct Tokenizer<'a> {
    pub iter: std::str::Chars<'a>,
    pub tokens: Vec<Token>,
}

impl<'a> Tokenizer<'a> {
    pub fn new() -> Tokenizer<'a> {
        Tokenizer {
            iter: "".chars(),
            tokens: Vec::new(),
        }
    }

    fn _tokenize(&mut self) -> () {
        use Token::*;
        if let Some(s) = self.iter.next() {
            let token = match s {
                '(' => LParen,
                ')' => RParen,
                '~' => Not,
                '^' => And,
                'v' => Or,
                '>' => Implies,
                '=' => Equal,
                'V' => Forall,
                'E' => Exists,
                ' ' => return self._tokenize(),
                _ => {
                    let symbol = self
                        .iter
                        .take_while_ref(|s| !matches!(s, '(' | ')' | '=' | 'V' | 'E' | ' '));
                    Symbol(s.to_string() + &symbol.collect::<String>())
                }
            };
            self.tokens.push(token);
            self._tokenize();
        }
    }

    pub fn tokenize(&mut self, s: &'a str) -> Vec<Token> {
        self.iter = s.chars();
        self.tokens.clear();
        self._tokenize();
        self.tokens.to_vec()
    }
}
