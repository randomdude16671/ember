#![allow(dead_code)]
use crate::asm::tokenizer::Rpos;

#[derive(Debug, Clone)]
pub enum EmberParseErrorType {
    UnexpectedChar,
    NoExpectedChar,
}

#[derive(Clone)]
pub struct EmberParseError {
    msg: String,
    help_msg: String,
    file_name: String,
    position: Rpos,
    kind: EmberParseErrorType,
}

struct StringBuilder {
    strings: Vec<String>,
}

impl EmberParseError {
    pub fn new(
        message: String,
        help_message: String,
        fname: String,
        pos: Rpos,
        k: EmberParseErrorType,
    ) -> Self {
        Self {
            msg: message,
            help_msg: help_message,
            file_name: fname,
            position: pos,
            kind: k,
        }
    }
}
