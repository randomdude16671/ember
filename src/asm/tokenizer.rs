use crate::asm::tokens::*;
use crate::map;
use hashbrown::HashMap;

struct Rpos {
    ln: usize,
    col: usize,
}

impl Rpos {
    fn new() -> Self {
        Self { ln: 1, col: 0 }
    }

    fn advance(&mut self, char: Option<char>) {
        if let Some(c) = char {
            if c == '\n' {
                self.ln += 1;
                self.col = 0;
            } else {
                self.col += 1;
            }
        }
    }
}

pub struct Tokenizer<'a> {
    src: &'a String,
    input: std::str::Chars<'a>,
    ch: Option<char>,
    file_path: String,
    position: usize, // current byte offset
    read_pos: usize, // next byte offset
    pos: Rpos,
    keywords: HashMap<String, TokenType>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(src: &'a String, file_path: String) -> Self {
        let mut input = src.chars();
        let ch = input.next();
        let keywords: HashMap<String, TokenType> = map! {};
        Self {
            src,
            input,
            ch,
            file_path,
            pos: Rpos::new(),
            read_pos: ch.map_or(0, |c| c.len_utf8()),
            position: 0,
            keywords: keywords,
        }
    }

    pub fn read_char(&mut self) {
        self.ch = self.input.next();

        if let Some(ch) = self.ch {
            self.position = self.read_pos;
            self.read_pos += ch.len_utf8(); // advance by how much the utf8 length of char is
        } else {
            // EOF
            self.position = self.read_pos;
        }

        self.pos.advance(self.ch);
    }

    pub fn peek_char(&mut self) -> Option<char> {
        self.input.clone().next()
    }
}
