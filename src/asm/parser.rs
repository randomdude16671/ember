#![allow(dead_code)]
use crate::asm::errors::*;
use crate::asm::tokenizer::*;

trait Node {
    fn string() -> String;
}

trait Expression {
    fn string() -> String;
    fn expression_node() -> Self;
}

trait Statement {
    fn string() -> String;
    fn statement_node() -> Self;
}

pub struct Parser<'a> {
    t: &'a Tokenizer<'a>,
    position: Rpos, // struct { ln: usize, col: usize }::new() => struct { .ln = 1, .col = 0 };
    errors: Vec<EmberParseError>,
}

impl<'a> Parser<'a> {
    pub fn new(tokenizer: &'a Tokenizer) -> Self {
        Self {
            t: tokenizer,
            position: Rpos::new(),
            errors: Vec::new(),
        }
    }

    fn error(&mut self, err: EmberParseError) {
        self.errors.push(err);
    }
}
