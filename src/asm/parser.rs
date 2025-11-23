#![allow(dead_code)]
use crate::asm::errors::EmberParseError;
use crate::asm::tokenizer::Tokenizer;

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
    errors: Vec<EmberParseError>,
}

impl<'a> Parser<'a> {
    pub fn new(tokenizer: &'a Tokenizer) -> Self {
        Self {
            t: tokenizer,
            errors: Vec::new(),
        }
    }
}
