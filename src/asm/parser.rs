#![allow(dead_code)]
use crate::asm::errors::*;
use crate::asm::tokenizer::*;

pub trait Node {
    fn string(&self) -> String;
}

pub trait Expression {
    fn string(&self) -> String;
    fn expression_node(&self) -> Self;
}

pub trait Statement {
    fn string(&self) -> String;
    fn statement_node(&self) -> Self
    where
        Self: Sized;
}

// String Builder
pub struct Sb {
    strings: Vec<String>,
    separator: char,
    append_count: usize,
}

impl Sb {
    pub fn new(sep: char) -> Self {
        Self {
            strings: Vec::new(),
            separator: sep,
            append_count: 0_usize,
        }
    }

    pub fn result(&mut self) -> String {
        let mut result_string = String::new();
        for str in &self.strings {
            result_string.push_str(str.as_str());
            result_string.push(self.separator);
        }
        result_string
    }

    pub fn write_string(&mut self, string_to_append: String) {
        self.strings.push(string_to_append);
        self.append_count += 1;
    }
}

pub struct Program {
    statements: Vec<Box<dyn Statement>>,
}

impl Program {
    pub fn string() -> String {
        let mut out = Sb::new('f');
        out.result()
    }
}

pub struct Parser<'a> {
    t: &'a Tokenizer<'a>,
    position: Rpos,
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
